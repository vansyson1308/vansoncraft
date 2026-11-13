<#
.SYNOPSIS
  Start the VansonCraft window from a release package and drive it through the MCP bridge.
  Used by .github/workflows/release.yml (the portable ZIP and the installed MSI).

.DESCRIPTION
  Starts -App with `--control -Port` (settings in -Out\config, so the runner's own profile is never
  touched), waits until it listens (cold start), runs mcp_smoke.py --desktop against it (the
  server must report the desktop backend and its edit must land in the window's document; the
  window's screenshot goes to -Out), stops it, starts it again (warm start) and stops it.
  Writes -Out\report.json.
#>
param(
  [Parameter(Mandatory)] [string] $App,
  [Parameter(Mandatory)] [string] $Cli,
  [Parameter(Mandatory)] [string] $Out,
  [Parameter(Mandatory)] [string] $Version,
  [int] $Port = 7979
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$env:VANSONCRAFT_CONFIG_DIR = Join-Path $Out 'config'

function Test-Port([int] $P) {
  $c = New-Object System.Net.Sockets.TcpClient
  try { $c.Connect('127.0.0.1', $P); return $true } catch { return $false } finally { $c.Dispose() }
}

function Start-App([string] $Log) {
  if (Test-Port $Port) { throw "something already listens on port $Port" }
  $t = [Diagnostics.Stopwatch]::StartNew()
  $p = Start-Process -FilePath $App -ArgumentList '--control', $Port -PassThru -RedirectStandardError $Log
  while ($t.Elapsed.TotalSeconds -lt 120) {
    if (Test-Port $Port) { return @{ Process = $p; Seconds = [math]::Round($t.Elapsed.TotalSeconds, 3) } }
    if ($p.HasExited) { Get-Content -ErrorAction SilentlyContinue $Log; throw "the app exited with $($p.ExitCode) before it listened" }
    Start-Sleep -Milliseconds 100
  }
  Get-Content -ErrorAction SilentlyContinue $Log
  throw "the app did not listen on $Port within 120 s"
}

function Stop-App($Started) {
  Stop-Process -Id $Started.Process.Id -Force -ErrorAction SilentlyContinue
  $Started.Process.WaitForExit(30000) | Out-Null
}

$cold = Start-App (Join-Path $Out 'app-cold.log')
try {
  $here = Split-Path -Parent $MyInvocation.MyCommand.Path
  python -I (Join-Path $here 'mcp_smoke.py') --cli $Cli --work (Join-Path $Out 'work') --desktop $Port --version $Version --report (Join-Path $Out 'report-desktop.json')
  if ($LASTEXITCODE -ne 0) { throw 'the desktop MCP check failed' }
} finally {
  Stop-App $cold
}
$warm = Start-App (Join-Path $Out 'app-warm.log')
Stop-App $warm
Write-Output "ok   the app listened after $($cold.Seconds) s (cold) and $($warm.Seconds) s (warm)"

$report = Get-Content -Raw (Join-Path $Out 'report-desktop.json') | ConvertFrom-Json
$report | Add-Member -NotePropertyName app_cold_start_s -NotePropertyValue $cold.Seconds
$report | Add-Member -NotePropertyName app_warm_start_s -NotePropertyValue $warm.Seconds
$report | ConvertTo-Json | Set-Content -Encoding utf8 (Join-Path $Out 'report.json')
