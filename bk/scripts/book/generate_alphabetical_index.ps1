# TODO Reviewed ??-??-2026
param(
    [Parameter(Mandatory = $true)]
    [string] $BookRoot,
    [Parameter(Mandatory = $true)]
    [string] $ToolsRoot
)

$ErrorActionPreference = 'Stop'
$BookRoot = [System.IO.Path]::GetFullPath($BookRoot)
$ToolsRoot = [System.IO.Path]::GetFullPath($ToolsRoot)
$indexPath = Join-Path $BookRoot 'src/indices/crates_alphabetical.md'
$crateListPath = Join-Path $BookRoot 'master/crates.txt'
$candidatePath = Join-Path (Split-Path -Parent $indexPath) ("crates_alphabetical." + [Guid]::NewGuid().ToString('N') + '.tmp')

Push-Location $ToolsRoot
try {
    Get-Content -LiteralPath $crateListPath |
        cargo run --locked -p crate_indices -- alphabetical_page --book-root $BookRoot > $candidatePath
    if ($LASTEXITCODE -ne 0) {
        throw "crate_indices exited with code $LASTEXITCODE"
    }

    Move-Item -LiteralPath $candidatePath -Destination $indexPath -Force
}
finally {
    Pop-Location
    if (Test-Path -LiteralPath $candidatePath) {
        Remove-Item -LiteralPath $candidatePath
    }
}
