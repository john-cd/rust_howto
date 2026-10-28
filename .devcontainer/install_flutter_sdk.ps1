<#
.SYNOPSIS
    Installs a version-pinned Flutter source checkout on Windows.
.PARAMETER Version
    Stable Flutter release version, such as 3.47.8.
.PARAMETER GitRevision
    Official 40-character commit hash for the release.
.PARAMETER InstallPath
    Absolute destination for the Flutter SDK.
#>
param(
    [Parameter(Mandatory)][string]$Version,
    [Parameter(Mandatory)][string]$GitRevision,
    [Parameter(Mandatory)][string]$InstallPath
)

$ErrorActionPreference = 'Stop'

if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    throw "Flutter version must be a stable x.y.z version: $Version"
}

if ($GitRevision -notmatch '^[0-9a-f]{40}$') {
    throw "Flutter Git revision must be a 40-character lowercase SHA: $GitRevision"
}

if (-not [IO.Path]::IsPathFullyQualified($InstallPath)) {
    throw "Flutter install path must be absolute: $InstallPath"
}

if (Test-Path -LiteralPath $InstallPath) {
    $currentRevision = git -C $InstallPath rev-parse HEAD 2>$null
    $flutterExecutable = Join-Path $InstallPath 'bin\flutter.bat'
    $hasFlutterExecutable = Test-Path -LiteralPath $flutterExecutable -PathType Leaf
    if ($LASTEXITCODE -eq 0 -and $currentRevision.Trim() -eq $GitRevision -and $hasFlutterExecutable) {
        Write-Host "Flutter $Version is already installed at $InstallPath"
        return
    }

    throw "Flutter install path already exists and does not match the requested revision: $InstallPath"
}

$installParent = Split-Path -Parent $InstallPath
New-Item -ItemType Directory -Path $installParent -Force | Out-Null
$temporaryDirectory = Join-Path $env:TEMP "flutter-install-$([guid]::NewGuid().ToString('N'))"
New-Item -ItemType Directory -Path $temporaryDirectory | Out-Null

try {
    $archivePath = Join-Path $temporaryDirectory 'flutter.tar.gz'
    $sourcePath = Join-Path $temporaryDirectory 'source'
    New-Item -ItemType Directory -Path $sourcePath | Out-Null
    $archiveUri = "https://codeload.github.com/flutter/flutter/tar.gz/refs/tags/$Version"
    Invoke-WebRequest -Uri $archiveUri -OutFile $archivePath

    tar.exe -xzf $archivePath --strip-components=1 -C $sourcePath
    if ($LASTEXITCODE -ne 0) {
        throw "Flutter source archive extraction failed with exit code $LASTEXITCODE."
    }

    if (-not (Test-Path -LiteralPath (Join-Path $sourcePath 'bin\flutter.bat') -PathType Leaf)) {
        throw 'Flutter source archive did not contain bin/flutter.bat.'
    }

    git -C $sourcePath init --initial-branch=stable
    if ($LASTEXITCODE -ne 0) { throw 'Could not initialize the Flutter Git checkout.' }
    git -C $sourcePath remote add origin https://github.com/flutter/flutter.git
    if ($LASTEXITCODE -ne 0) { throw 'Could not configure the Flutter Git remote.' }
    git -C $sourcePath config core.autocrlf false
    if ($LASTEXITCODE -ne 0) { throw 'Could not configure the Flutter Git checkout.' }
    git -C $sourcePath fetch --depth=1 --filter=blob:none origin "refs/tags/${Version}:refs/tags/${Version}"
    if ($LASTEXITCODE -ne 0) { throw 'Could not fetch Flutter release metadata.' }

    $actualRevision = (git -C $sourcePath rev-parse "refs/tags/${Version}^{commit}").Trim()
    if ($LASTEXITCODE -ne 0) { throw 'Could not resolve the Flutter release commit.' }
    if ($actualRevision -ne $GitRevision) {
        throw "Flutter $Version resolved to $actualRevision; expected $GitRevision."
    }

    git -C $sourcePath update-ref refs/heads/stable $actualRevision
    if ($LASTEXITCODE -ne 0) { throw 'Could not set the Flutter release branch.' }
    git -C $sourcePath read-tree $actualRevision
    if ($LASTEXITCODE -ne 0) { throw 'Could not index the Flutter release source.' }
    Move-Item -LiteralPath $sourcePath -Destination $InstallPath
    Write-Host "Installed Flutter $Version ($actualRevision) at $InstallPath"
}
finally {
    $resolvedTemporaryDirectory = (Resolve-Path -LiteralPath $temporaryDirectory).Path
    $temporaryRoot = (Resolve-Path -LiteralPath $env:TEMP).Path.TrimEnd('\') + '\'
    if ($resolvedTemporaryDirectory.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase)) {
        Remove-Item -LiteralPath $resolvedTemporaryDirectory -Recurse -Force
    }
}
