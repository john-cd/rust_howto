param(
    [Parameter(Mandatory = $true)]
    [string] $BookRoot,
    [Parameter(Mandatory = $true)]
    [string] $ToolsRoot
)

$ErrorActionPreference = 'Stop'
$BookRoot = [System.IO.Path]::GetFullPath($BookRoot)
$ToolsRoot = [System.IO.Path]::GetFullPath($ToolsRoot)
$indexPath = Join-Path $BookRoot 'src/indices/crates_by_category.md'
$candidatePath = "$indexPath.tmp"
$crateListPath = Join-Path $BookRoot 'master/crates.txt'
$metadataPath = Join-Path $BookRoot 'master/crate_categories.toml'

Push-Location $ToolsRoot
try {
    Get-Content -LiteralPath $crateListPath |
        cargo run --locked -p crate_indices -- category_page_from_metadata --metadata $metadataPath --book-root $BookRoot > $candidatePath
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
