<#
.SYNOPSIS
    Project Obsidian - Windows Policies Administration Layer
.DESCRIPTION
    Manages Enterprise Local Group Policy registry nodes for Privacy, Search, Copilot,
    Delivery Optimization, and Advertising. Interacts via Registry.ps1.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
. (Join-Path $scriptDir 'Registry.ps1')

function Set-ObsidianPrivacyPolicies {
    [CmdletBinding(SupportsShouldProcess = $true)]
    param(
        [Parameter(Mandatory = $false)]
        [switch]$Revert
    )

    $results = [System.Collections.Generic.List[object]]::new()

    if ($Revert) {
        # Restore standard Windows defaults by removing forced restrictions
        $policiesToRemove = @(
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'; Name = 'AllowTelemetry' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'; Name = 'MaxTelemetryAllowed' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'; Name = 'DoNotShowFeedbackNotifications' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo'; Name = 'DisabledByGroupPolicy' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI'; Name = 'DisableAIDataAnalysis' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot'; Name = 'TurnOffWindowsCopilot' },
            @{ Path = 'HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot'; Name = 'TurnOffWindowsCopilot' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'DisableWebSearch' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'ConnectedSearchUseWeb' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'AllowCloudSearch' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'EnableDynamicContentInWSB' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent'; Name = 'DisableWindowsConsumerFeatures' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization'; Name = 'DODownloadMode' }
        )

        foreach ($p in $policiesToRemove) {
            $results.Add((Remove-RegistryValueSafe -Path $p.Path -Name $p.Name))
        }
    } else {
        # Apply strict enterprise privacy baseline
        $policiesToApply = @(
            # Data Collection & Telemetry (0 = Security/Minimal)
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'; Name = 'AllowTelemetry'; Value = 0; Type = 'DWord' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'; Name = 'MaxTelemetryAllowed'; Value = 0; Type = 'DWord' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'; Name = 'DoNotShowFeedbackNotifications'; Value = 1; Type = 'DWord' },

            # Advertising ID
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo'; Name = 'DisabledByGroupPolicy'; Value = 1; Type = 'DWord' },

            # Windows AI / Recall (Windows 11 24H2+ snapshot analysis)
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI'; Name = 'DisableAIDataAnalysis'; Value = 1; Type = 'DWord' },

            # Windows Copilot
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot'; Name = 'TurnOffWindowsCopilot'; Value = 1; Type = 'DWord' },
            @{ Path = 'HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot'; Name = 'TurnOffWindowsCopilot'; Value = 1; Type = 'DWord' },

            # Search Web Integration & Highlights
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'DisableWebSearch'; Value = 1; Type = 'DWord' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'ConnectedSearchUseWeb'; Value = 0; Type = 'DWord' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'AllowCloudSearch'; Value = 0; Type = 'DWord' },
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name = 'EnableDynamicContentInWSB'; Value = 0; Type = 'DWord' },

            # Consumer Bloatware & Automatic Promotion Downloads
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent'; Name = 'DisableWindowsConsumerFeatures'; Value = 1; Type = 'DWord' },

            # Delivery Optimization (0 = HTTP only, disables P2P internet uploads)
            @{ Path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization'; Name = 'DODownloadMode'; Value = 0; Type = 'DWord' }
        )

        foreach ($p in $policiesToApply) {
            $results.Add((Set-RegistryValueSafe -Path $p.Path -Name $p.Name -Value $p.Value -PropertyType $p.Type))
        }
    }

    return $results
}
