# Privacy Hardening Reference

Project Obsidian drastically reduces the telemetry footprint of Windows 11 without breaking Windows Update or Microsoft Store.

## Implemented Optimizations (v2.0)

### 1. Core Telemetry Services
- **Action:** Disables DiagTrack (Connected User Experiences and Telemetry) and WerSvc (Windows Error Reporting).
- **Why:** Stops Windows from collecting kernel ETW traces, crash dumps, and application usage statistics, which saves bandwidth, CPU cycles, and preserves your privacy.

### 2. Consumer Bloat Services
- **Action:** Disables MapsBroker and RetailDemo.
- **Why:** Removes background services intended for store display units and offline maps tracking.

### 3. Telemetry Scheduled Tasks
- **Action:** Disables Microsoft Compatibility Appraiser, ProgramDataUpdater, Consolidator, and UsbCeip.
- **Why:** These tasks routinely wake up the system to scan installed software, connected USB devices, and system configuration to build a demographic profile for Microsoft.

### 4. Enterprise Group Policies
- **Action:** Applies the Set-ObsidianPrivacyPolicies matrix.
- **Why:** Leverages official Microsoft enterprise policies to disable Advertising ID, Tailored Experiences, Activity Feed history, Cortana search integration, and typing data collection.
