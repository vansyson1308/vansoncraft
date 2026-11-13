<#
.SYNOPSIS
  Check that -Name in -Folder has the SHA-256 that -Folder\SHA256SUMS.txt lists for it.
  Used by .github/workflows/release.yml before a package is installed or unpacked.
#>
param([Parameter(Mandatory)] [string] $Folder, [Parameter(Mandatory)] [string] $Name)
$ErrorActionPreference = 'Stop'
$want = $null
foreach ($line in Get-Content -LiteralPath (Join-Path $Folder 'SHA256SUMS.txt')) {
  $parts = $line -split '  ', 2
  if ($parts.Count -eq 2 -and $parts[1] -ceq $Name -and $parts[0] -match '^[0-9a-f]{64}$') { $want = $parts[0]; break }
}
if (-not $want) { throw "SHA256SUMS.txt does not list $Name" }
$have = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $Folder $Name)).Hash.ToLowerInvariant()
if ($have -ne $want) { throw "$Name has SHA-256 $have, SHA256SUMS.txt says $want" }
Write-Output "ok   $Name matches SHA256SUMS.txt ($have)"
