[CmdletBinding()]
param(
    [switch]$SkipWasm,
    [string]$ExpectedVersion,
    [switch]$RequireClean
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot

function Invoke-Gate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [scriptblock]$Command
    )

    Write-Host "`n==> $Name" -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

try {
    $branch = (git branch --show-current).Trim()
    $head = (git rev-parse --short HEAD).Trim()
    Write-Host "Checking branch $branch at $head" -ForegroundColor DarkGray

    if ($RequireClean) {
        $changes = @(git status --porcelain)
        if ($changes.Count -gt 0) {
            throw "Release checks require a clean tree. Commit or stash all tracked and untracked changes first."
        }
    }

    if ($ExpectedVersion) {
        $manifest = Get-Content "Cargo.toml" -Raw
        if ($manifest -notmatch "(?m)^version = `"$([regex]::Escape($ExpectedVersion))`"$") {
            throw "Cargo.toml does not declare version $ExpectedVersion."
        }

        $lockfile = Get-Content "Cargo.lock" -Raw
        $lockPattern = "(?ms)\[\[package\]\]\r?\nname = `"algobuddy`"\r?\nversion = `"$([regex]::Escape($ExpectedVersion))`""
        if ($lockfile -notmatch $lockPattern) {
            throw "Cargo.lock does not contain the AlgoBuddy package at version $ExpectedVersion."
        }

        $changelog = Get-Content "CHANGELOG.md" -Raw
        if ($changelog -notmatch "(?m)^## \[$([regex]::Escape($ExpectedVersion))\]") {
            throw "CHANGELOG.md does not contain a release heading for $ExpectedVersion."
        }
    }

    Invoke-Gate "Formatting" { cargo fmt --all -- --check }
    Invoke-Gate "Clippy" { cargo clippy --all-targets -- -D warnings }
    Invoke-Gate "Tests" { cargo test --all }
    Invoke-Gate "Diff hygiene" { git diff --check }
    Invoke-Gate "Native release build" { cargo build --release }

    if (-not $SkipWasm) {
        if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
            throw "Trunk is required for the WebAssembly release gate. Install it or rerun with -SkipWasm."
        }
        Invoke-Gate "WebAssembly release build" {
            trunk build --release --public-url ./
        }
    }

    if ($SkipWasm) {
        Write-Warning "WebAssembly was skipped. Contributor gates passed, but this is not a complete release check."
    }
    else {
        Write-Host "`nAll release gates passed." -ForegroundColor Green
    }
}
finally {
    Pop-Location
}
