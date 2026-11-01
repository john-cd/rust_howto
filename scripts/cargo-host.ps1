# Validate host storage and optionally configure direct Cargo's central fallback.
param(
    [Parameter(Mandatory)][string]$BuildRoot,
    [switch]$ConfigureHost
)
$ErrorActionPreference = 'Stop'
$fullBuildRoot = [IO.Path]::GetFullPath($BuildRoot)
if ($IsWindows) {
    $existing = $fullBuildRoot
    while (-not (Test-Path -LiteralPath $existing)) {
        $parent = Split-Path -Parent $existing
        if (-not $parent -or $parent -eq $existing) { throw "Cannot resolve build volume: $BuildRoot" }
        $existing = $parent
    }
    # Native drive information avoids starting a CIM query on every runner call.
    # Resolve mount points/junctions through the volume API when a parent is a link.
    $driveRoot = [IO.Path]::GetPathRoot($existing)
    $fileSystem = [IO.DriveInfo]::new($driveRoot).DriveFormat
    $ancestor = $existing
    while ($ancestor -and $ancestor -ne $driveRoot) {
        if ((Get-Item -LiteralPath $ancestor -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) {
            $fileSystem = (Get-Volume -FilePath $existing -ErrorAction Stop).FileSystem
            break
        }
        $ancestor = Split-Path -Parent $ancestor
    }
    if ($fileSystem -ne 'NTFS') {
        throw "Cargo intermediate builds require NTFS; $BuildRoot uses $fileSystem. Set CODE_BUILD_ROOT to an NTFS directory."
    }
}
if ($ConfigureHost) {
    $packages = if ($env:CODE_PACKAGES_ROOT) { $env:CODE_PACKAGES_ROOT } elseif ($IsWindows) { 'D:/packages' } else { Join-Path $HOME '.local/share' }
    $cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $packages 'cargo' }
    $targetRoot = if ($env:CODE_TARGET_ROOT) { $env:CODE_TARGET_ROOT } elseif ($IsWindows) { 'D:/target' } else { Join-Path $(if ($env:XDG_CACHE_HOME) { $env:XDG_CACHE_HOME } else { Join-Path $HOME '.cache' }) 'cargo-target' }
    $configPath = Join-Path $cargoHome 'config.toml'
    New-Item -ItemType Directory -Path $cargoHome -Force | Out-Null
    $text = if (Test-Path -LiteralPath $configPath) { [IO.File]::ReadAllText($configPath).Replace("`r`n", "`n") } else { '' }
    # Change only build output keys; preserve registry, authentication, and other settings.
    $values = [ordered]@{
        'rustc-wrapper' = 'sccache'
        'incremental' = 'false'
        'target-dir' = (Join-Path $targetRoot 'direct-cargo').Replace('\', '/')
        'build-dir' = ((Join-Path $fullBuildRoot 'direct-cargo').Replace('\', '/') + '/{workspace-path-hash}')
    }
    if ($text -notmatch '(?m)^\[build\]\s*$') { $text = $text.TrimEnd() + "`n`n[build]`n" }
    $match = [regex]::Match($text, '(?ms)^\[build\][^\n]*\n(?<body>.*?)(?=^\[|\z)')
    $body = $match.Groups['body'].Value
    foreach ($key in $values.Keys) {
        if ($values[$key].Contains("'")) { throw "Cargo roots cannot contain a single quote: $($values[$key])" }
        $line = if ($key -eq 'incremental') { "$key = false" } else { "$key = '$($values[$key])'" }
        $pattern = '(?m)^\s*' + [regex]::Escape($key) + '\s*=.*$'
        if ([regex]::IsMatch($body, $pattern)) { $body = [regex]::Replace($body, $pattern, [System.Text.RegularExpressions.MatchEvaluator]{ param($m) $line }) }
        else { $body = $body.TrimEnd() + "`n$line`n" }
    }
    $replacement = "[build]`n" + $body.TrimStart("`n")
    $text = $text.Substring(0, $match.Index) + $replacement + $text.Substring($match.Index + $match.Length)
    [IO.File]::WriteAllText($configPath, $text.TrimStart("`n").TrimEnd() + "`n", [Text.UTF8Encoding]::new($false))
    Write-Output "Configured $configPath"
} else { Write-Output 'ok' }
