# Windows 11 Telemetry & Diagnostic Technical Reference

## Architectural Layers of Windows Telemetry
Windows 11 gathers diagnostic events through the **Unified Telemetry Client (UTC)** architecture.

```text
[Applications / Shell / Subsystems]
             │
             ▼
[ETW Providers (Event Tracing for Windows)]
             │
             ▼
[Connected User Experiences and Telemetry (DiagTrack Service)]
             │
             ▼
[Diagnostic Data Store (%ProgramData%\Microsoft\Diagnosis)]
             │
             ▼
[Encrypted Transmission to vortex.data.microsoft.com]
```

## How Obsidian Neutralizes Telemetry Safely
1. **At the Registry/Policy Tier**:
   - Forces `AllowTelemetry = 0` (Security/Minimal level).
   - Silences `DoNotShowFeedbackNotifications = 1`.
2. **At the Service Tier**:
   - Stops and sets `DiagTrack` startup type to `Disabled`.
   - Prevents the UTC daemon from spawning listener threads.
3. **At the Scheduled Tasks Tier**:
   - Disables `Microsoft Compatibility Appraiser` (`CompatTelRunner.exe`), preventing background disk thrashing and hardware inventory scanning.
   - Disables `ProgramDataUpdater` and `Consolidator` (CEIP).
4. **At the Shell Search Tier**:
   - Binds Start Menu search queries to local Windows Search Indexer only (`DisableWebSearch = 1`), preventing user keystrokes from being streamed to Bing telemetry endpoints.
