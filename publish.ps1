<#
.SYNOPSIS
Validates and publishes probably-bool to crates.io.

.DESCRIPTION
Runs the same checks expected before a release, then either packages the crate
without publishing (`-DryRun`) or publishes it to crates.io. Authentication is
handled by Cargo: use `cargo login` or set `CARGO_REGISTRY_TOKEN` before a real
publish.

.EXAMPLE
.\publish.ps1 -DryRun

.EXAMPLE
.\publish.ps1
#>
[CmdletBinding()]
param(
    [switch]$DryRun,
    [switch]$SkipChecks
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Invoke-Cargo {
    param(
        [Parameter(Mandatory)]
        [string[]]$Arguments
    )

    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($Arguments -join ' ') failed with exit code $LASTEXITCODE."
    }
}

$projectRoot = Split-Path -Parent $PSCommandPath
Push-Location $projectRoot

try {
    if (-not $SkipChecks) {
        Invoke-Cargo -Arguments @('fmt', '--check')
        Invoke-Cargo -Arguments @('test', '--locked')
        Invoke-Cargo -Arguments @('clippy', '--all-targets', '--locked', '--', '-D', 'warnings')
    }

    if ($DryRun) {
        Invoke-Cargo -Arguments @('publish', '--dry-run', '--locked')
    }
    else {
        Invoke-Cargo -Arguments @('publish', '--locked')
    }
}
finally {
    Pop-Location
}
