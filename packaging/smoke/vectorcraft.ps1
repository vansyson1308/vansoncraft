<#
.SYNOPSIS
  Install VectorCraft, the project VansonCraft is based on, so the MSI check can prove that  # brand-ok: the app this test installs beside VansonCraft
  installing and removing VansonCraft leaves it alone. Used by .github/workflows/release.yml.

.DESCRIPTION
  Downloads VectorCraft's own Windows x64 MSI of version -Tag from its GitHub releases  # brand-ok: the app this test installs beside VansonCraft
  (storytold/vectorcraft, with the workflow's read-only token), installs it for the current user,  # brand-ok: the app this test installs beside VansonCraft
  and writes -Out\vectorcraft.json: its uninstall entry, install folder and a SHA-256 of every  # brand-ok: the app this test installs beside VansonCraft
  installed file, which msi.ps1 compares before and after.
  The download must match -Sha256. The pinned value is the hash of the v0.24.1 release's
  vectorcraft-0.24.1-windows-x64.msi as the release workflow first downloaded it (2026-11-13);  # brand-ok: the file this test installs beside VansonCraft
  with an empty -Sha256 the file's hash is printed as a warning so it can be pinned.
#>
param(
  [ValidateSet('install')] [string] $Action = 'install',
  [Parameter(Mandatory)] [string] $Out,
  [string] $Tag = 'v0.24.1',
  [string] $Sha256 = '72756c96d8533d7bb347f013e7f24e8be7d91dfe080849bb259fdb9d5cedee62'
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $Out | Out-Null
gh release download $Tag --repo storytold/vectorcraft --pattern '*-windows-x64.msi' --dir $Out  # brand-ok: the app this test installs beside VansonCraft
if ($LASTEXITCODE -ne 0) { throw "could not download VectorCraft $Tag's Windows x64 MSI" }  # brand-ok: the app this test installs beside VansonCraft
$msis = @(Get-ChildItem -LiteralPath $Out -Filter '*.msi')
if ($msis.Count -ne 1) { throw "expected one Windows x64 MSI in the $Tag release, found $($msis.Count)" }
$msi = $msis[0]
$hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $msi.FullName).Hash.ToLowerInvariant()
if ($Sha256) {
  if ($hash -ne $Sha256.ToLowerInvariant()) { throw "$($msi.Name) has SHA-256 $hash, expected $Sha256" }
  Write-Output "ok   $($msi.Name) matches its pinned SHA-256"
} else {
  Write-Output "::warning::$($msi.Name) is not pinned; its SHA-256 is $hash"
}
Write-Output "installing $($msi.Name)"
$p = Start-Process msiexec.exe -ArgumentList '/i', "`"$($msi.FullName)`"", '/qn', '/norestart', '/l*v', "`"$(Join-Path $Out 'install.log')`"" -Wait -PassThru
if ($p.ExitCode -ne 0) { throw "VectorCraft's MSI failed with $($p.ExitCode)" }  # brand-ok: the app this test installs beside VansonCraft

$entry = Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*', 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*' -ErrorAction SilentlyContinue |
  Where-Object { $_.DisplayName -eq 'VectorCraft' } | Select-Object -First 1  # brand-ok: the app this test installs beside VansonCraft
if (-not $entry) { throw 'no uninstall entry for VectorCraft after installing it' }  # brand-ok: the app this test installs beside VansonCraft
# Where it went, as the package itself records it: the App Paths entry names the program (the
# folder is not ours to guess), else the uninstall entry's InstallLocation.
$appPaths = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\App Paths\vectorcraft.exe', 'HKLM:\Software\Microsoft\Windows\CurrentVersion\App Paths\vectorcraft.exe'  # brand-ok: the app this test installs beside VansonCraft
$exe = $appPaths | ForEach-Object { (Get-ItemProperty -LiteralPath $_ -ErrorAction SilentlyContinue).'(default)' } |
  Where-Object { $_ -and (Test-Path -LiteralPath $_) } | Select-Object -First 1
if (-not $exe -and $entry.InstallLocation) {
  $exe = Join-Path $entry.InstallLocation 'vectorcraft.exe'  # brand-ok: the app this test installs beside VansonCraft
  if (-not (Test-Path -LiteralPath $exe)) { $exe = $null }
}
if (-not $exe) { throw 'neither the App Paths entry nor the uninstall entry leads to the installed program' }
$folder = (Resolve-Path -LiteralPath (Split-Path -Parent $exe)).ProviderPath.TrimEnd('\')
$files = Get-ChildItem -LiteralPath $folder -Recurse -File | ForEach-Object { @{ path = $_.FullName.Substring($folder.Length + 1); sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash } }
@{ productCode = $entry.PSChildName; version = $entry.DisplayVersion; folder = $folder; files = $files } |
  ConvertTo-Json -Depth 4 | Set-Content -Encoding utf8 (Join-Path $Out 'vectorcraft.json')  # brand-ok: the app this test installs beside VansonCraft
Write-Output "ok   VectorCraft $($entry.DisplayVersion) installed in $folder ($($files.Count) files)"  # brand-ok: the app this test installs beside VansonCraft
