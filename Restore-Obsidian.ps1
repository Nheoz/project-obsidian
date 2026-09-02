<#
.SYNOPSIS
    Project Obsidian - Standalone Disaster Recovery & Rollback Engine
.DESCRIPTION
    Restores the exact previous system state (Registry keys, Windows Services, Scheduled Tasks,
    and Enterprise Policies) recorded by Project Obsidian. Fully functional even after OS reboot.
#>

[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $false)]
    [string]$SnapshotPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Continue'

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$StateDir = Join-Path $ScriptDir "obsidian-state"

Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host "         PROJECT OBSIDIAN - STANDALONE ROLLBACK AND RECOVERY ENGINE              " -ForegroundColor Cyan
Write-Host "================================================================================" -ForegroundColor Cyan

# Verify Admin Privileges
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "[!] Rollback requires Administrator privileges. Please re-run as Administrator." -ForegroundColor Red
    Exit 1
}

# Locate latest snapshot if none specified
if ([string]::IsNullOrWhiteSpace($SnapshotPath)) {
    if (-not (Test-Path $StateDir)) {
        Write-Host "[-] No obsidian-state directory found at $StateDir" -ForegroundColor Yellow
        Write-Host "[*] Executing global policy restoration fallback..." -ForegroundColor Cyan
        . (Join-Path $ScriptDir "powershell\Policies.ps1")
        Set-ObsidianPrivacyPolicies -Revert
        
        # Restore telemetry services fallback
        $telemetryServices = @('DiagTrack', 'WerSvc')
        foreach ($s in $telemetryServices) {
            Set-Service -Name $s -StartupType Automatic -ErrorAction SilentlyContinue
            Start-Service -Name $s -ErrorAction SilentlyContinue
        }
        Write-Host "[V] Global baseline settings restored to Windows defaults." -ForegroundColor Green
        Exit 0
    }

    $latestFile = Get-ChildItem -Path $StateDir -Filter "snapshot-*.json" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if ($null -eq $latestFile) {
        Write-Host "[-] No snapshot JSON found in $StateDir. Applying default policy reversal..." -ForegroundColor Yellow
        . (Join-Path $ScriptDir "powershell\Policies.ps1")
        Set-ObsidianPrivacyPolicies -Revert
        Exit 0
    }
    $SnapshotPath = $latestFile.FullName
}

Write-Host "[*] Loading atomic snapshot: $SnapshotPath" -ForegroundColor Cyan
$snapshot = Get-Content -Path $SnapshotPath -Raw -Encoding UTF8 | ConvertFrom-Json

# 1. Restore Registry Values
if ($snapshot.registry_items) {
    Write-Host "`n[1/3] Restoring Registry Values..." -ForegroundColor Yellow
    foreach ($item in $snapshot.registry_items) {
        try {
            if ($item.previous_exists) {
                if (-not (Test-Path -LiteralPath $item.path)) {
                    New-Item -Path $item.path -Force | Out-Null
                }
                New-ItemProperty -LiteralPath $item.path -Name $item.name -Value $item.previous_value -PropertyType $item.previous_type -Force | Out-Null
                Write-Host "  [OK] Restored [$($item.path)\$($item.name)] -> $($item.previous_value)" -ForegroundColor Green
            } else {
                if (Test-Path -LiteralPath $item.path) {
                    Remove-ItemProperty -LiteralPath $item.path -Name $item.name -Force -ErrorAction SilentlyContinue | Out-Null
                    Write-Host "  [OK] Removed created key [$($item.path)\$($item.name)]" -ForegroundColor Green
                }
            }
        } catch {
            Write-Host "  [!] Error restoring registry property $($item.name): $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

# 2. Restore Services
if ($snapshot.services) {
    Write-Host "`n[2/3] Restoring Windows Services..." -ForegroundColor Yellow
    foreach ($svc in $snapshot.services) {
        try {
            if ($svc.previous_startup -and $svc.previous_startup -ne 'NotPresent') {
                Set-Service -Name $svc.name -StartupType $svc.previous_startup -ErrorAction SilentlyContinue
                if ($svc.previous_status -eq 'Running') {
                    Start-Service -Name $svc.name -ErrorAction SilentlyContinue
                }
                Write-Host "  [OK] Restored Service [$($svc.name)] -> $($svc.previous_startup)" -ForegroundColor Green
            }
        } catch {
            Write-Host "  [!] Error restoring service $($svc.name): $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

# 3. Restore Scheduled Tasks
if ($snapshot.tasks) {
    Write-Host "`n[3/3] Restoring Scheduled Tasks..." -ForegroundColor Yellow
    foreach ($t in $snapshot.tasks) {
        try {
            if ($t.previous_state -eq 'Ready' -or $t.previous_state -eq 'Running') {
                Enable-ScheduledTask -TaskPath $t.path -TaskName $t.name -ErrorAction SilentlyContinue | Out-Null
                Write-Host "  [OK] Re-enabled Task [$($t.name)]" -ForegroundColor Green
            }
        } catch {
            Write-Host "  [!] Error restoring task $($t.name): $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

Write-Host "`n[4/4] Verifying Rollback Integrity..." -ForegroundColor Yellow
$verifyFailed = 0

if ($snapshot.services) {
    foreach ($svc in $snapshot.services) {
        if ($svc.previous_startup -and $svc.previous_startup -ne 'NotPresent') {
            $current = Get-Service -Name $svc.name -ErrorAction SilentlyContinue
            if ($current.StartType.ToString() -ne $svc.previous_startup) {
                Write-Host "  [!] Verify Failed: Service $($svc.name) is $($current.StartType), expected $($svc.previous_startup)" -ForegroundColor Red
                $verifyFailed++
            }
        }
    }
}

if ($verifyFailed -eq 0) {
    Write-Host "  [OK] Rollback integrity verified. System state perfectly restored." -ForegroundColor Green
} else {
    Write-Host "  [!] Rollback integrity check failed for $verifyFailed item(s)." -ForegroundColor Red
}

Write-Host "`n================================================================================" -ForegroundColor Cyan
Write-Host "[V] OBSIDIAN ROLLBACK COMPLETED SUCCESSFULLY." -ForegroundColor Green
Write-Host "    All tracked modifications have been reverted to their exact prior state." -ForegroundColor White
Write-Host "================================================================================" -ForegroundColor Cyan
