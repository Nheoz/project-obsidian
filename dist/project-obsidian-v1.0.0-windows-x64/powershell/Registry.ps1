<#
.SYNOPSIS
    Project Obsidian - Registry Administration Layer
.DESCRIPTION
    Provides zero-trust, typed, auditable, and fully reversible Windows Registry manipulation.
    Complies with CIS Benchmarks, Microsoft Security Baselines, and Enterprise PowerShell Standards.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Get-RegistryValueSafe {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$Name,

        [Parameter(Mandatory = $false)]
        [object]$DefaultValue = $null
    )

    try {
        if (Test-Path -LiteralPath $Path) {
            $item = Get-ItemProperty -LiteralPath $Path -Name $Name -ErrorAction SilentlyContinue
            if ($null -ne $item -and ($item.PSObject.Properties[$Name])) {
                return @{
                    Exists = $true
                    Value  = $item.$Name
                    Type   = (Get-Item -LiteralPath $Path).GetValueKind($Name).ToString()
                }
            }
        }
        return @{
            Exists = $false
            Value  = $DefaultValue
            Type   = $null
        }
    } catch {
        Write-Warning "Failed to read registry property [$Path\$Name]: $($_.Exception.Message)"
        return @{
            Exists = $false
            Value  = $DefaultValue
            Type   = $null
        }
    }
}

function Set-RegistryValueSafe {
    [CmdletBinding(SupportsShouldProcess = $true)]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$Name,

        [Parameter(Mandatory = $true)]
        [object]$Value,

        [Parameter(Mandatory = $false)]
        [ValidateSet('DWord', 'QWord', 'String', 'ExpandString', 'MultiString', 'Binary')]
        [string]$PropertyType = 'DWord'
    )

    $current = Get-RegistryValueSafe -Path $Path -Name $Name

    $result = [PSCustomObject]@{
        Path           = $Path
        Name           = $Name
        PreviousExists = $current.Exists
        PreviousValue  = $current.Value
        PreviousType   = $current.Type
        TargetValue    = $Value
        TargetType     = $PropertyType
        Status         = 'Unchanged'
        Success        = $true
        Error          = $null
    }

    # Idempotency check: if current value and type match target, do not write
    if ($current.Exists -and ($current.Value -eq $Value) -and ($current.Type -eq $PropertyType)) {
        $result.Status = 'AlreadyCompliant'
        return $result
    }

    if ($PSCmdlet.ShouldProcess("$Path\$Name", "Set value to $Value ($PropertyType)")) {
        try {
            if (-not (Test-Path -LiteralPath $Path)) {
                New-Item -Path $Path -Force | Out-Null
            }

            New-ItemProperty -LiteralPath $Path -Name $Name -Value $Value -PropertyType $PropertyType -Force | Out-Null
            
            # Post-write verification
            $verify = Get-RegistryValueSafe -Path $Path -Name $Name
            if ($verify.Exists -and ($verify.Value -eq $Value)) {
                $result.Status  = 'Applied'
                $result.Success = $true
            } else {
                $result.Status  = 'VerificationFailed'
                $result.Success = $false
                $result.Error   = 'Value mismatch after write operation'
            }
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

function Remove-RegistryValueSafe {
    [CmdletBinding(SupportsShouldProcess = $true)]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$Name
    )

    $current = Get-RegistryValueSafe -Path $Path -Name $Name

    $result = [PSCustomObject]@{
        Path           = $Path
        Name           = $Name
        PreviousExists = $current.Exists
        PreviousValue  = $current.Value
        Status         = 'Unchanged'
        Success        = $true
        Error          = $null
    }

    if (-not $current.Exists) {
        $result.Status = 'AlreadyAbsent'
        return $result
    }

    if ($PSCmdlet.ShouldProcess("$Path\$Name", "Remove property")) {
        try {
            Remove-ItemProperty -LiteralPath $Path -Name $Name -Force -ErrorAction Stop | Out-Null
            $result.Status  = 'Removed'
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
