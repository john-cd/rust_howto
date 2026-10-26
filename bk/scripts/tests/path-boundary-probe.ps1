param([Parameter(Mandatory)][string]$Value)

if ($Value -ceq '__boundary_exit_23__') {
    exit 23
}

if ($Value -cne $env:JUST_PATH_BOUNDARY_EXPECTED) {
    [Console]::Error.WriteLine('The path argument changed while crossing the just/PowerShell boundary.')
    exit 42
}

exit 0
