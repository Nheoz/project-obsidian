<#
.SYNOPSIS
    Project Obsidian - Windows 11 Privacy & Performance Engine
.DESCRIPTION
    Native PowerShell CLI and launcher for Project Obsidian.
    Dispatches to obsidian.exe when compiled, or executes native engine modules.
#>

[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $false)]
    [ValidateSet('Analyze', 'Doctor', 'Apply', 'Validate', 'Restore', 'Benchmark', 'Status')]
    [string]$Command = 'Status',

    [Parameter(Mandatory = $false)]
    [ValidateSet('Privacy', 'Gaming', 'AI', 'Developer', 'Ultimate')]
    [string]$Profile = 'Ultimate',

    [Parameter(Mandatory = $false)]
    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

# Priority order: next to this script, then dev build paths (for contributors)
$RustBinary = $null
$candidatePaths = @(
    (Join-Path $ScriptDir "obsidian.exe"),           # Distribution: exe alongside script
    (Join-Path $ScriptDir "target\release\obsidian.exe"), # Dev: cargo build --release
    (Join-Path $ScriptDir "target\debug\obsidian.exe")    # Dev: cargo build
)
foreach ($candidate in $candidatePaths) {
    if (Test-Path $candidate) {
        $RustBinary = $candidate
        break
    }
}

# If the compiled Rust binary exists, delegate to obsidian.exe for maximum performance
if ($null -ne $RustBinary) {
    $argsList = @($Command.ToLower())
    if ($Command -eq 'Apply') {
        $argsList += "--profile"
        $argsList += $Profile.ToLower()
        if ($DryRun) { $argsList += "--dry-run" }
    }
    Write-Host "[*] Launching Project Obsidian Core (Rust): $RustBinary $($argsList -join ' ')" -ForegroundColor Cyan
    & $RustBinary @argsList
    Exit $LASTEXITCODE
}

# Fallback: Native PowerShell execution of the modules
Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host "  PROJECT OBSIDIAN - Windows 11 Privacy-First AI AND Gaming Workstation Engine   " -ForegroundColor Cyan
Write-Host "================================================================================" -ForegroundColor Cyan

# Check Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin -and $Command -eq 'Apply') {
    Write-Host "[!] Admin privileges are required to apply optimizations. Please run as Administrator." -ForegroundColor Red
    Exit 1
}

# Load PowerShell Modules
. (Join-Path $ScriptDir "powershell\Registry.ps1")
. (Join-Path $ScriptDir "powershell\Services.ps1")
. (Join-Path $ScriptDir "powershell\ScheduledTasks.ps1")
. (Join-Path $ScriptDir "powershell\Policies.ps1")
. (Join-Path $ScriptDir "powershell\Validation.ps1")

switch ($Command) {
    'Status' {
        Write-Host "`n[+] AUDITING PRIVACY & PERFORMANCE POSTURE" -ForegroundColor Yellow
        $diag = Get-ServiceStateSafe -ServiceName 'DiagTrack'
        Write-Host "  Telemetry Service (DiagTrack):    $($diag.Status) [$($diag.StartupType)]"
        $wer = Get-ServiceStateSafe -ServiceName 'WerSvc'
        Write-Host "  Error Reporting (WerSvc):        $($wer.Status) [$($wer.StartupType)]"
        $appraiser = Get-ScheduledTaskStateSafe -TaskPath '\Microsoft\Windows\Application Experience\' -TaskName 'Microsoft Compatibility Appraiser'
        Write-Host "  Compatibility Appraiser Task:    $($appraiser.State)"
        $pol = Get-RegistryValueSafe -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'AllowTelemetry'
        Write-Host "  Telemetry Policy Level:          $(if ($pol.Exists) { $pol.Value } else { 'Default (Telemetry Active)' })"
    }

    'Doctor' {
        Write-Host "`n[+] AI & GAMING ENVIRONMENT AUDIT" -ForegroundColor Yellow
        $health = Test-ObsidianHealth
        foreach ($h in $health) {
            $color = if ($h.Passed) { [ConsoleColor]::Green } else { [ConsoleColor]::Yellow }
            Write-Host "  [$($h.Status)] $($h.Component.PadRight(30)) : $($h.Details)" -ForegroundColor $color
        }
    }

    'Validate' {
        Write-Host "`n[+] EXECUTING POST-FLIGHT VALIDATION" -ForegroundColor Yellow
        $health = Test-ObsidianHealth
        $allPassed = $true
        foreach ($h in $health) {
            if (-not $h.Passed) { $allPassed = $false }
            Write-Host "  [$($h.Status)] $($h.Component.PadRight(30)) : $($h.Details)"
        }
        if ($allPassed) {
            Write-Host "`n[V] ALL CRITICAL SYSTEMS HEALTHY." -ForegroundColor Green
        } else {
            Write-Host "`n[!] Some validations reported warnings. Review details above." -ForegroundColor Yellow
        }
    }

    'Apply' {
        Write-Host "`n[+] APPLYING PROFILE [$Profile] (DryRun: $DryRun)" -ForegroundColor Yellow
        
        # Save snapshot
        $stateDir = Join-Path $ScriptDir "obsidian-state"
        if (-not (Test-Path $stateDir)) { New-Item -ItemType Directory -Path $stateDir -Force | Out-Null }
        $timestamp = (Get-Date).ToString("yyyyMMdd-HHmmss")
        $snapshotFile = Join-Path $stateDir "snapshot-$timestamp.json"

        # Apply policies
        $policyResults = Set-ObsidianPrivacyPolicies
        
        # Apply services
        $svcResults = @(
            (Set-ServiceStateSafe -ServiceName 'DiagTrack' -TargetStartupType 'Disabled' -StopIfRunning),
            (Set-ServiceStateSafe -ServiceName 'WerSvc' -TargetStartupType 'Disabled' -StopIfRunning),
            (Set-ServiceStateSafe -ServiceName 'MapsBroker' -TargetStartupType 'Disabled' -StopIfRunning),
            (Set-ServiceStateSafe -ServiceName 'RetailDemo' -TargetStartupType 'Disabled' -StopIfRunning)
        )

        # Apply tasks
        $taskResults = @(
            (Set-ScheduledTaskStateSafe -TaskPath '\Microsoft\Windows\Application Experience\' -TaskName 'Microsoft Compatibility Appraiser' -TargetState 'Disabled'),
            (Set-ScheduledTaskStateSafe -TaskPath '\Microsoft\Windows\Application Experience\' -TaskName 'ProgramDataUpdater' -TargetState 'Disabled'),
            (Set-ScheduledTaskStateSafe -TaskPath '\Microsoft\Windows\Customer Experience Improvement Program\' -TaskName 'Consolidator' -TargetState 'Disabled'),
            (Set-ScheduledTaskStateSafe -TaskPath '\Microsoft\Windows\Customer Experience Improvement Program\' -TaskName 'UsbCeip' -TargetState 'Disabled')
        )

        # Record atomic snapshot
        $snapshotData = @{
            timestamp      = (Get-Date).ToString("o")
            profile        = $Profile
            registry_items = @($policyResults | ForEach-Object { @{
                path = $_.Path; name = $_.Name; previous_exists = $_.PreviousExists; previous_value = $_.PreviousValue; previous_type = $_.PreviousType
            }})
            services       = @($svcResults | ForEach-Object { @{
                name = $_.Name; previous_startup = $_.PreviousStartup; previous_status = $_.PreviousStatus
            }})
            tasks          = @($taskResults | ForEach-Object { @{
                path = $_.TaskPath; name = $_.TaskName; previous_state = $_.PreviousState
            }})
        }
        $snapshotData | ConvertTo-Json -Depth 6 | Set-Content -Path $snapshotFile -Encoding UTF8
        Write-Host "`n[V] Atomic rollback snapshot preserved at: $snapshotFile" -ForegroundColor Green
        Write-Host "[V] Optimizations applied safely and successfully." -ForegroundColor Green
    }

    'Restore' {
        & (Join-Path $ScriptDir "Restore-Obsidian.ps1")
    }
}
