<#
.SYNOPSIS
  Install VansonCraft's MSI for the current user, use it, and uninstall it, with VectorCraft  # brand-ok: the app this test installs beside VansonCraft
  installed beside it. Used by .github/workflows/release.yml (smoke-windows-msi).

.DESCRIPTION
  - installs -Msi silently (msiexec /qn: the package's default, the current user);
  - checks that vansoncraft.exe and vansoncraft-cli.exe are in %LOCALAPPDATA%\Programs\VansonCraft,
    that the uninstall entry and the App Paths entry name them, that the Start menu and desktop
    shortcuts exist, and that the CLI reports -Version;
  - runs mcp_smoke.py on the installed CLI and the desktop check (desktop.ps1) on the installed app;
  - checks that VectorCraft (-VectorCraft\vectorcraft.json, written by vectorcraft.ps1) is still  # brand-ok: the app this test installs beside VansonCraft
    installed, with every file as it was;
  - installs the same file again (a repair-style maintenance run, not an upgrade);
  - uninstalls VansonCraft and checks that its files, entries and shortcuts are gone and that
    VectorCraft is still intact.  # brand-ok: the app this test installs beside VansonCraft
  Writes -Out\report.json with the install and uninstall times.
#>
param(
  [Parameter(Mandatory)] [string] $Msi,
  [Parameter(Mandatory)] [string] $Version,
  [Parameter(Mandatory)] [string] $Out,
  [Parameter(Mandatory)] [string] $VectorCraft  # brand-ok: the app this test installs beside VansonCraft
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$folder = Join-Path $env:LOCALAPPDATA 'Programs\VansonCraft'
$uninstallKeys = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*', 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'

function Get-Entry([string] $Name) {
  Get-ItemProperty $uninstallKeys -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -eq $Name }
}

# VansonCraft.lnk in the Start menu's Programs folder and on the desktop, for this user or all users.
function Get-Shortcuts {
  $folders = 'Programs', 'CommonPrograms', 'Desktop', 'CommonDesktopDirectory' | ForEach-Object { [Environment]::GetFolderPath($_) } | Where-Object { $_ }
  @(Get-ChildItem -LiteralPath $folders -Filter 'VansonCraft*.lnk' -Recurse -ErrorAction SilentlyContinue)
}

function Assert-VectorCraftIntact([string] $When) {
  $before = Get-Content -Raw (Join-Path $VectorCraft 'vectorcraft.json') | ConvertFrom-Json  # brand-ok: the app this test installs beside VansonCraft
  if (-not (Get-Entry 'VectorCraft' | Where-Object { $_.PSChildName -eq $before.productCode })) { throw "$When, VectorCraft's uninstall entry is gone" }  # brand-ok: the app this test installs beside VansonCraft
  foreach ($f in $before.files) {
    $p = Join-Path $before.folder $f.path
    if (-not (Test-Path -LiteralPath $p)) { throw "$When, VectorCraft's $($f.path) is gone" }  # brand-ok: the app this test installs beside VansonCraft
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $p).Hash -ne $f.sha256) { throw "$When, VectorCraft's $($f.path) changed" }  # brand-ok: the app this test installs beside VansonCraft
  }
  Write-Output "ok   $When, VectorCraft $($before.version) is installed and unchanged ($($before.files.Count) files)"  # brand-ok: the app this test installs beside VansonCraft
}

function Invoke-Msiexec([string[]] $Arguments, [string] $What) {
  $t = [Diagnostics.Stopwatch]::StartNew()
  $p = Start-Process msiexec.exe -ArgumentList $Arguments -Wait -PassThru
  if ($p.ExitCode -ne 0) { throw "$What failed with $($p.ExitCode) (see $Out)" }
  return [math]::Round($t.Elapsed.TotalSeconds, 2)
}

if (Test-Path $folder) { throw "$folder exists before the install" }
$installS = Invoke-Msiexec @('/i', "`"$Msi`"", '/qn', '/norestart', '/l*v', "`"$(Join-Path $Out 'install.log')`"") 'the install'
foreach ($f in 'vansoncraft.exe', 'vansoncraft-cli.exe') {
  if (-not (Test-Path (Join-Path $folder $f))) { throw "the MSI did not install $f in $folder" }
}
$entry = @(Get-Entry 'VansonCraft')
if ($entry.Count -ne 1) { throw "expected one uninstall entry for VansonCraft, found $($entry.Count)" }
# The MSI's ProductVersion is numeric: package.ps1 drops a pre-release suffix such as -rc.1.
$msiVersion = ($Version -split '-')[0]
if ($entry[0].DisplayVersion -ne $msiVersion) { throw "the uninstall entry says version $($entry[0].DisplayVersion), expected $msiVersion" }
if ($entry[0].Publisher -ne 'vansyson1308') { throw "the uninstall entry names publisher '$($entry[0].Publisher)'" }
$appPath = (Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\App Paths\vansoncraft.exe').'(default)'
if ($appPath -ne (Join-Path $folder 'vansoncraft.exe')) { throw "App Paths names '$appPath'" }
$shortcuts = Get-Shortcuts
$inStart = @($shortcuts | Where-Object { $_.DirectoryName -in [Environment]::GetFolderPath('Programs'), [Environment]::GetFolderPath('CommonPrograms') })
$onDesktop = @($shortcuts | Where-Object { $_.DirectoryName -in [Environment]::GetFolderPath('Desktop'), [Environment]::GetFolderPath('CommonDesktopDirectory') })
if ($inStart.Count -ne 1 -or $onDesktop.Count -ne 1) { throw "expected one Start menu and one desktop shortcut, found: $($shortcuts.FullName -join ', ')" }
$cli = Join-Path $folder 'vansoncraft-cli.exe'
$said = & $cli --version
if ($LASTEXITCODE -ne 0 -or $said -ne "vansoncraft-cli $Version") { throw "the installed CLI said '$said'" }
Write-Output "ok   installed for this user in $folder in $installS s; $said"
Assert-VectorCraftIntact 'after installing VansonCraft'

python -I (Join-Path $here 'mcp_smoke.py') --cli $cli --work (Join-Path $Out 'work') --version $Version --report (Join-Path $Out 'report-headless.json')
if ($LASTEXITCODE -ne 0) { throw 'the MCP check of the installed CLI failed' }
& (Join-Path $here 'desktop.ps1') -App (Join-Path $folder 'vansoncraft.exe') -Cli $cli -Out (Join-Path $Out 'desktop') -Version $Version -Port 7980
python -I (Join-Path $here 'mcp_smoke.py') --cli $cli --work (Join-Path $Out 'setup') --setup --report (Join-Path $Out 'report-setup.json')
if ($LASTEXITCODE -ne 0) { throw 'setup and doctor of the installed CLI failed' }

# The same file again: Windows Installer runs it as maintenance of the installed product (same
# ProductCode), not as an upgrade. The CLI a client was set up with is still there, at the same
# path, and there is still one entry.
$reinstallS = Invoke-Msiexec @('/i', "`"$Msi`"", '/qn', '/norestart', '/l*v', "`"$(Join-Path $Out 'reinstall.log')`"") 'installing again'
$said = & $cli --version
if ($LASTEXITCODE -ne 0 -or $said -ne "vansoncraft-cli $Version") { throw "after installing again, the CLI said '$said'" }
if (@(Get-Entry 'VansonCraft').Count -ne 1) { throw 'installing again left more than one uninstall entry' }
$entry = @(Get-Entry 'VansonCraft')
Write-Output "ok   installed again in $reinstallS s: $cli still answers, one uninstall entry"

$uninstallS = Invoke-Msiexec @('/x', $entry[0].PSChildName, '/qn', '/norestart', '/l*v', "`"$(Join-Path $Out 'uninstall.log')`"") 'the uninstall'
$left = @(Get-ChildItem -LiteralPath $folder -Recurse -File -ErrorAction SilentlyContinue)
if ($left.Count -ne 0) { throw "files left in $folder after uninstalling: $($left.FullName -join ', ')" }
if (Test-Path $folder) { Write-Output "::warning::$folder is left behind, empty, after uninstalling" }
if (@(Get-Entry 'VansonCraft').Count -ne 0) { throw 'the uninstall entry is still there' }
if (Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\App Paths\vansoncraft.exe') { throw 'the App Paths entry is still there' }
$shortcuts = Get-Shortcuts
if ($shortcuts.Count -ne 0) { throw "shortcuts left after uninstalling: $($shortcuts.FullName -join ', ')" }
Write-Output "ok   uninstalled in $uninstallS s: no files, entries or shortcuts left"
Assert-VectorCraftIntact 'after uninstalling VansonCraft'

@{ install_s = $installS; reinstall_s = $reinstallS; uninstall_s = $uninstallS } | ConvertTo-Json | Set-Content -Encoding utf8 (Join-Path $Out 'report.json')
