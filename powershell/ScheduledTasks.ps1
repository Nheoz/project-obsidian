<#
.SYNOPSIS
    Project Obsidian - Scheduled Tasks Administration Layer
.DESCRIPTION
    Provides zero-trust, safe, and fully reversible Windows Scheduled Tasks manipulation.
    Strictly prohibits task deletion; enforces state toggling with full state capture.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Get-ScheduledTaskStateSafe {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$TaskPath,

        [Parameter(Mandatory = $true)]
        [string]$TaskName
    )

    try {
        $task = Get-ScheduledTask -TaskPath $TaskPath -TaskName $TaskName -ErrorAction SilentlyContinue
        if ($null -eq $task) {
            return @{
                Exists = $false
                Path   = $TaskPath
                Name   = $TaskName
                State  = 'NotPresent'
            }
        }

        return @{
            Exists = $true
            Path   = $TaskPath
            Name   = $task.TaskName
            State  = $task.State.ToString()
        }
    } catch {
        return @{
            Exists = $false
            Path   = $TaskPath
            Name   = $TaskName
            State  = 'NotPresent'
        }
    }
}

function Set-ScheduledTaskStateSafe {
    [CmdletBinding(SupportsShouldProcess = $true)]
    param(
        [Parameter(Mandatory = $true)]
        [string]$TaskPath,

        [Parameter(Mandatory = $true)]
        [string]$TaskName,

        [Parameter(Mandatory = $true)]
        [ValidateSet('Enabled', 'Disabled')]
        [string]$TargetState
    )

    $current = Get-ScheduledTaskStateSafe -TaskPath $TaskPath -TaskName $TaskName
    $result = [PSCustomObject]@{
        TaskPath      = $TaskPath
        TaskName      = $TaskName
        Exists        = $current.Exists
        PreviousState = $current.State
        TargetState   = $TargetState
        Status        = 'Unchanged'
        Success       = $true
        Error         = $null
    }

    if (-not $current.Exists) {
        $result.Status = 'NotPresent'
        return $result
    }

    if ($current.State -eq $TargetState) {
        $result.Status = 'AlreadyCompliant'
        return $result
    }

    if ($PSCmdlet.ShouldProcess("$TaskPath$TaskName", "Set state to $TargetState")) {
        try {
            if ($TargetState -eq 'Disabled') {
                Disable-ScheduledTask -TaskPath $TaskPath -TaskName $TaskName -ErrorAction Stop | Out-Null
            } else {
                Enable-ScheduledTask -TaskPath $TaskPath -TaskName $TaskName -ErrorAction Stop | Out-Null
            }

            $verify = Get-ScheduledTaskStateSafe -TaskPath $TaskPath -TaskName $TaskName
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
