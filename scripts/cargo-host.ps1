# TODO Reviewed ??-??-2026

# Validate host storage and optionally configure direct Cargo's central fallback.
param(
    [Parameter(Mandatory)][string]$BuildRoot,
    [switch]$ConfigureHost,
    [switch]$LinkOutputs
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

# Connect checkout-local Cargo/IDE paths to this checkout's central output slot.
if ($LinkOutputs) {
    $checkout = Split-Path -Parent $PSScriptRoot
    $parent = Split-Path -Parent $checkout
    $slot = if ((Split-Path -Leaf $parent) -eq '.worktrees') { Split-Path -Leaf $checkout } else { 'main' }
    if ($slot -ne 'main' -and $slot -cnotmatch '^[ab]$') { throw 'Use only .worktrees/a or .worktrees/b.' }
    $repo = if ($slot -eq 'main') { Split-Path -Leaf $checkout } else { Split-Path -Leaf (Split-Path -Parent $parent) }
    $cacheHome = if ($env:XDG_CACHE_HOME) { $env:XDG_CACHE_HOME } else { Join-Path $HOME '.cache' }
    $targetRoot = if ($env:CODE_TARGET_ROOT) { $env:CODE_TARGET_ROOT } elseif ($IsWindows) { 'D:/target' } else { Join-Path $cacheHome 'cargo-target' }
    $cacheRoot = if ($env:CODE_SCCACHE_ROOT) { $env:CODE_SCCACHE_ROOT } elseif ($env:SCCACHE_DIR) { $env:SCCACHE_DIR } elseif ($IsWindows) { 'D:/sccache' } else { Join-Path $cacheHome 'sccache' }
    $outputs = [ordered]@{
        target = Join-Path (Join-Path $targetRoot $repo) $slot
        build = Join-Path (Join-Path $fullBuildRoot $repo) $slot
        '.sccache' = $cacheRoot
    }
    foreach ($name in $outputs.Keys) {
        $destination = [IO.Path]::GetFullPath($outputs[$name])
        $link = Join-Path $checkout $name
        $item = Get-Item -LiteralPath $link -Force -ErrorAction SilentlyContinue
        if ($null -ne $item) {
            if (-not ($item.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw "Refusing to replace existing directory or file: $link" }
            $existingTarget = [IO.Path]::GetFullPath($item.ResolveLinkTarget($true).FullName)
            if ($existingTarget -ne $destination) { throw "Refusing to replace $link -> $existingTarget (expected $destination)" }
        } else {
            New-Item -ItemType Directory -Path $destination -Force | Out-Null
            $kind = if ($IsWindows) { 'Junction' } else { 'SymbolicLink' }
            New-Item -ItemType $kind -Path $link -Target $destination | Out-Null
        }
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
