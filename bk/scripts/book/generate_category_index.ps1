# Purpose: Generate the book page listing crates by category.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
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

    if (Test-Path -LiteralPath $indexPath) {
        $current = [System.IO.File]::ReadAllText($indexPath)
        $reviewMarker = [regex]::Match($current, '\A(<!-- TODO Reviewed [^\r\n]* -->\r?\n)')
        if ($reviewMarker.Success) {
            $generated = [System.IO.File]::ReadAllText($candidatePath)
            if (-not $generated.StartsWith($reviewMarker.Groups[1].Value)) {
                [System.IO.File]::WriteAllText(
                    $candidatePath,
                    $reviewMarker.Groups[1].Value + $generated,
                    [System.Text.UTF8Encoding]::new($false)
                )
            }
        }
    }

    Move-Item -LiteralPath $candidatePath -Destination $indexPath -Force
}
finally {
    Pop-Location
    if (Test-Path -LiteralPath $candidatePath) {
        Remove-Item -LiteralPath $candidatePath
    }
}
