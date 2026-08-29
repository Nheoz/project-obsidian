<#
.SYNOPSIS
    Project Obsidian - Services Administration Layer
.DESCRIPTION
    Provides zero-trust, dependency-aware, and fully reversible Windows Services manipulation.
    Guarantees strict isolation of critical operating system services.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Immutable classification of Core Services - NEVER TO BE DISABLED
$script:DO_NOT_TOUCH_SERVICES = [System.Collections.Generic.HashSet[string]]::new(
    [string[]]@(
        'wuauserv',    # Windows Update
        'WinDefend',   # Microsoft Defender Antivirus Service
        'WdNisSvc',    # Microsoft Defender Network Inspection
        'RpcSs',       # Remote Procedure Call
        'DcomLaunch',  # DCOM Server Process Launcher
        'RpcEptMapper',# RPC Endpoint Mapper
        'Winmgmt',     # Windows Management Instrumentation
        'BITS',        # Background Intelligent Transfer Service
        'CryptSvc',    # Cryptographic Services
        'EventLog',    # Windows Event Log
        'Dnscache',    # DNS Client
        'Dhcp',        # DHCP Client
        'nlasvc',      # Network Location Awareness
        'PlugPlay',    # Plug and Play
        'LanmanWorkstation', # Workstation
        'ProfSvc',     # User Profile Service
        'gpsvc',       # Group Policy Client
        'SamSs',       # Security Accounts Manager
        'KeyIso',      # CNG Key Isolation
        'BFE',         # Base Filtering Engine
        'mpssvc'       # Windows Defender Firewall
    ),
    [System.StringComparer]::OrdinalIgnoreCase
)

function Test-IsServiceProtected {
    [CmdletBinding()]
    param([string]$ServiceName)
    return $script:DO_NOT_TOUCH_SERVICES.Contains($ServiceName)
}

function Get-ServiceStateSafe {
    [CmdletBinding()]
    param([string]$ServiceName)
    
    $svc = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if ($null -eq $svc) {
        return @{
            Exists      = $false
            Name        = $ServiceName
            DisplayName = $null
            Status      = 'NotPresent'
            StartupType = 'NotPresent'
            DependentServices = @()
        }
    }

    $startup = (Get-CimInstance -ClassName Win32_Service -Filter "Name='$ServiceName'" -ErrorAction SilentlyContinue).StartMode
    if ($null -eq $startup) { $startup = $svc.StartType.ToString() }

    return @{
        Exists            = $true
        Name              = $svc.Name
        DisplayName       = $svc.DisplayName
        Status            = $svc.Status.ToString()
        StartupType       = $startup
        DependentServices = @($svc.DependentServices | Where-Object { $_.Status -eq 'Running' } | Select-Object -ExpandProperty Name)
    }
}

function Set-ServiceStateSafe {
    [CmdletBinding(SupportsShouldProcess = $true)]
    param(
        [Parameter(Mandatory = $true)]
        [string]$ServiceName,

        [Parameter(Mandatory = $true)]
        [ValidateSet('Disabled', 'Manual', 'Automatic')]
        [string]$TargetStartupType,

        [Parameter(Mandatory = $false)]
        [switch]$StopIfRunning,

        [Parameter(Mandatory = $false)]
        [switch]$StartIfStopped
    )

    if (Test-IsServiceProtected -ServiceName $ServiceName) {
        throw [System.Security.SecurityException]::new("BLOCKED BY OBSIDIAN POLICY: Service [$ServiceName] is critical to system integrity and cannot be modified.")
    }

    $current = Get-ServiceStateSafe -ServiceName $ServiceName
    $result = [PSCustomObject]@{
        Name            = $ServiceName
        Exists          = $current.Exists
        PreviousStartup = $current.StartupType
        PreviousStatus  = $current.Status
        TargetStartup   = $TargetStartupType
        Status          = 'Unchanged'
        Success         = $true
        Error           = $null
    }

    if (-not $current.Exists) {
        $result.Status = 'NotPresent'
        return $result
    }

    # Dependency check: do not disable if dependent running services exist
    if ($TargetStartupType -eq 'Disabled' -and $current.DependentServices.Count -gt 0) {
        $depList = $current.DependentServices -join ', '
        $result.Status  = 'SkippedDueToDependencies'
        $result.Success = $false
        $result.Error   = "Active running dependencies detected: [$depList]"
        return $result
    }

    if ($PSCmdlet.ShouldProcess($ServiceName, "Set startup type to $TargetStartupType")) {
        try {
            if ($current.StartupType -ne $TargetStartupType) {
                Set-Service -Name $ServiceName -StartupType $TargetStartupType -ErrorAction Stop
            }

            if ($StopIfRunning -and $current.Status -eq 'Running') {
                Stop-Service -Name $ServiceName -Force -ErrorAction Stop
            } elseif ($StartIfStopped -and $current.Status -ne 'Running') {
                Start-Service -Name $ServiceName -ErrorAction Stop
            }

            $verify = Get-ServiceStateSafe -ServiceName $ServiceName
            $result.Status  = 'Applied'
            $result.Success = $true
        } catch {
            $result.Status  = 'Error'
            $result.Success = $false
            $result.Error   = $_.Exception.Message
        }
    } else {
        $result.Status = 'DryRun'
    }

    return $result
}
