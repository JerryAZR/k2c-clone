# Search the locally-generated Bevy 0.19 docs.
# Usage: .\tools\search-docs.ps1 <query>

param(
    [Parameter(Mandatory=$true)]
    [string]$Query
)

$DocRoot = Join-Path $PSScriptRoot ".." "target" "doc"
$BevyDir = Join-Path $DocRoot "bevy"

if (-not (Test-Path $BevyDir)) {
    Write-Error "Docs not found at $BevyDir. Run: cargo doc"
    exit 1
}

Write-Host "Searching Bevy docs for: $Query"
rg -i $Query "$BevyDir" "$DocRoot\bevy_*"
