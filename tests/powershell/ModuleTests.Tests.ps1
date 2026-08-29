<#
.SYNOPSIS
    Pester Integration & Safety Tests for Project Obsidian PowerShell Layer
#>

BeforeAll {
    $script:PSScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
    $script:RepoRoot = Resolve-Path (Join-Path $script:PSScriptRoot "..\..")
    $script:PsDir = Join-Path $script:RepoRoot "powershell"

    . (Join-Path $script:PsDir "Registry.ps1")
    . (Join-Path $script:PsDir "Services.ps1")
    . (Join-Path $script:PsDir "ScheduledTasks.ps1")
    . (Join-Path $script:PsDir "Policies.ps1")
    . (Join-Path $script:PsDir "Validation.ps1")
}

Describe "Project Obsidian - Security & Core OS Protection Tests" {
    Context "Critical Service Immutability Protection" {
        It "Must permanently forbid modification of Windows Update (wuauserv)" {
            (Test-IsServiceProtected -ServiceName 'wuauserv') | Should -Be $true
            { Set-ServiceStateSafe -ServiceName 'wuauserv' -TargetStartupType 'Disabled' } | Should -Throw
        }

        It "Must permanently forbid modification of Microsoft Defender (WinDefend)" {
            (Test-IsServiceProtected -ServiceName 'WinDefend') | Should -Be $true
            { Set-ServiceStateSafe -ServiceName 'WinDefend' -TargetStartupType 'Disabled' } | Should -Throw
        }

        It "Must permanently forbid modification of RPC and WMI core services" {
            (Test-IsServiceProtected -ServiceName 'RpcSs') | Should -Be $true
            (Test-IsServiceProtected -ServiceName 'Winmgmt') | Should -Be $true
            (Test-IsServiceProtected -ServiceName 'CryptSvc') | Should -Be $true
        }
    }

    Context "Registry Layer Safe Verification" {
        It "Should safely query non-existent keys without throwing" {
            $val = Get-RegistryValueSafe -Path "HKLM:\SOFTWARE\ObsidianNonExistentTest" -Name "DummyVal"
            $val.Exists | Should -Be $false
            $val.Value | Should -BeNullOrEmpty
        }

        It "Should support dry-run -WhatIf semantics without altering state" {
            $res = Set-RegistryValueSafe -Path "HKCU:\Software\ObsidianTestKey" -Name "TestVal" -Value 1 -WhatIf
            $res.Status | Should -Be "DryRun"
        }
    }

    Context "Scheduled Tasks Safe Verification" {
        It "Should return not-present safely for non-existent tasks" {
            $task = Get-ScheduledTaskStateSafe -TaskPath "\Obsidian\" -TaskName "NonExistentTask"
            $task.Exists | Should -Be $false
            $task.State | Should -Be "NotPresent"
        }
    }
}
