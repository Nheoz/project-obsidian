# Privacy Hardening Architecture

## Philosophy: Honest Privacy vs. Placebo Claims
Unlike aggressive debloaters that claim "100% telemetry eradication" while silently breaking Windows components, **Project Obsidian** takes a transparent, enterprise-aligned stance.

Microsoft categorizes operating system telemetry into four distinct tiers:
1. **Optional Diagnostic Data**: Detailed browser habits, typing samples, crash dumps with memory excerpts, and behavioral profiling. (Obsidian **disables** this).
2. **Advertising & Promotional Telemetry**: Advertising ID, Windows Spotlight ads, suggestions in Settings and Start Menu. (Obsidian **disables** this).
3. **Activity Tracking & Keylogging**: Implicit ink and typing data harvesting, Activity Feed timeline sync. (Obsidian **disables** this).
4. **Required / Security Diagnostic Data**: Minimal telemetry required to verify device security, hardware architecture, and driver update distribution via Windows Update. (Obsidian sets this to `0 - Security` where supported, or `1 - Required`).

## Targeted Subsystems

### 1. Group Policy / Registry Nodes
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection\AllowTelemetry` = `0`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection\DoNotShowFeedbackNotifications` = `1`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo\DisabledByGroupPolicy` = `1`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI\DisableAIDataAnalysis` = `1` (Windows Recall)
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot\TurnOffWindowsCopilot` = `1`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search\DisableWebSearch` = `1`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search\ConnectedSearchUseWeb` = `0`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search\AllowCloudSearch` = `0`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent\DisableWindowsConsumerFeatures` = `1`
- `HKLM:\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization\DODownloadMode` = `0`

### 2. Services
- **`DiagTrack`** (*Connected User Experiences and Telemetry*): Set to `Disabled` and stopped.
- **`WerSvc`** (*Windows Error Reporting*): Set to `Disabled` and stopped.
- **`MapsBroker`** (*Downloaded Maps Manager*): Set to `Disabled` and stopped.
- **`RetailDemo`** (*Retail Demo Service*): Set to `Disabled` and stopped.

### 3. Scheduled Tasks
- `\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser` (`CompatTelRunner.exe` background scan).
- `\Microsoft\Windows\Application Experience\ProgramDataUpdater`.
- `\Microsoft\Windows\Customer Experience Improvement Program\Consolidator`.
- `\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip`.
