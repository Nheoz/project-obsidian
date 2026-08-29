<#
.SYNOPSIS
    Project Obsidian - Windows Features & Subsystems Layer
.DESCRIPTION
    Audits and manages optional Windows features pertinent to AI development and gaming
    (WSL2, Hyper-V, Virtual Machine Platform). Never forces destructive removals.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Get-ObsidianFeatureStatus {
    [CmdletBinding()]
    param()

    $featuresToCheck = @(
        'Microsoft-Windows-Subsystem-Linux',
        'VirtualMachinePlatform',
        'Containers-DisposableClientVM', # Windows Sandbox
        'HypervisorPlatform'
    )

    $statusList = [System.Collections.Generic.List[object]]::new()

    foreach ($feat in $featuresToCheck) {
        $info = Get-WindowsOptionalFeature -Online -FeatureName $feat -ErrorAction SilentlyContinue
        if ($null -ne $info) {
            $statusList.Add([PSCustomObject]@{
                FeatureName = $info.FeatureName
                State       = $info.State.ToString()
                RestartRequired = $info.RestartNeeded
            })
        } else {
            $statusList.Add([PSCustomObject]@{
                FeatureName = $feat
                State       = 'NotAvailable'
                RestartRequired = $false
            })
        }
    }

    return $statusList
}
