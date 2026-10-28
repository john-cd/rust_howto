# Purpose: Install and configure Ruby 4.0 for the Rutie FFI example.
# Usage: Run with pwsh -File .devcontainer/install_ruby_4_0.ps1; no arguments are required.

$ErrorActionPreference = 'Stop'

$rubyInstallerVersion = '4.0.7-1'
$rubyVersionPattern = '^ruby 4\.0\.\d+\b'
$rubyRoot = Join-Path $env:SystemDrive 'Ruby40-x64'
$rubyBin = Join-Path $rubyRoot 'bin'
$rubyExecutable = Join-Path $rubyBin 'ruby.exe'
$rubyBuiltinDlls = Join-Path $rubyBin 'ruby_builtin_dlls'

# Prepends installed Ruby directories to the process and user PATH without duplicates.
function Set-RubyPath {
    $rubyDirectories = @($rubyBin, $rubyBuiltinDlls) |
        Where-Object { Test-Path -LiteralPath $_ -PathType Container }
    $normalizedRubyDirectories = @($rubyDirectories | ForEach-Object { $_.TrimEnd('\') })

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $userEntries = @($userPath -split ';' | Where-Object { $_ })
    $remainingUserEntries = @(
        $userEntries | Where-Object { $_.TrimEnd('\') -notin $normalizedRubyDirectories }
    )
    $updatedUserPath = (@($rubyDirectories) + $remainingUserEntries) -join ';'
    [Environment]::SetEnvironmentVariable('Path', $updatedUserPath, 'User')

    $processEntries = @($env:Path -split ';' | Where-Object { $_ })
    $remainingProcessEntries = @(
        $processEntries | Where-Object { $_.TrimEnd('\') -notin $normalizedRubyDirectories }
    )
    $env:Path = (@($rubyDirectories) + $remainingProcessEntries) -join ';'
    return $rubyDirectories
}

# Put existing Ruby DLL directories on PATH before asking ruby.exe for its version.
$rubyDirectories = Set-RubyPath
$rubyVersionIsInstalled = $false
if (Test-Path -LiteralPath $rubyExecutable -PathType Leaf) {
    $rubyVersionOutput = & $rubyExecutable --version
    $rubyVersionIsInstalled = $LASTEXITCODE -eq 0 -and
        $rubyVersionOutput -match $rubyVersionPattern
}

if (-not $rubyVersionIsInstalled) {
    $wingetVerb = if (Test-Path -LiteralPath $rubyExecutable -PathType Leaf) { 'upgrade' } else { 'install' }
    $wingetArguments = @(
        $wingetVerb,
        '--id', 'RubyInstallerTeam.Ruby.4.0',
        '--exact',
        '--version', $rubyInstallerVersion,
        '--source', 'winget',
        '--accept-source-agreements',
        '--accept-package-agreements'
    )
    & winget @wingetArguments
    if ($LASTEXITCODE -notin @(0, 3010)) {
        throw "RubyInstaller $rubyInstallerVersion installation failed with exit code $LASTEXITCODE."
    }
}

if (-not (Test-Path -LiteralPath $rubyExecutable -PathType Leaf)) {
    throw "RubyInstaller did not install ruby.exe at the expected path: $rubyExecutable"
}

$rubyDirectories = Set-RubyPath
$env:RUBY = $rubyExecutable
[Environment]::SetEnvironmentVariable('RUBY', $rubyExecutable, 'User')

$rubyVersionOutput = & $rubyExecutable --version
if ($LASTEXITCODE -ne 0 -or $rubyVersionOutput -notmatch $rubyVersionPattern) {
    throw "Expected a Ruby 4.0.x release at $rubyExecutable, got: $rubyVersionOutput"
}

$rubyHeaderDirectory = & $rubyExecutable -rrbconfig -e 'print RbConfig::CONFIG["rubyhdrdir"]'
if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($rubyHeaderDirectory)) {
    throw 'Could not determine the Ruby 4.0 header directory.'
}

$rubyHeaderPath = Join-Path $rubyHeaderDirectory 'ruby.h'
if (-not (Test-Path -LiteralPath $rubyHeaderPath -PathType Leaf)) {
    throw "Ruby 4.0 headers were not found at: $rubyHeaderDirectory"
}

Write-Host "Configured Ruby 4.0 at $rubyExecutable" -ForegroundColor Green
