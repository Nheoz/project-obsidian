# Windows Services Classification Matrix

Project Obsidian maintains a strict, zero-trust classification of all Windows services.

| Service Name | Display Name | Classification | Action | Technical Justification |
| :--- | :--- | :--- | :--- | :--- |
| **`wuauserv`** | Windows Update | **DO NOT TOUCH** | Preserved | Essential for security patches, driver delivery, and OS integrity. |
| **`WinDefend`** | Microsoft Defender | **DO NOT TOUCH** | Preserved | Anti-malware protection and kernel security hooks. |
| **`RpcSs`** | Remote Procedure Call | **DO NOT TOUCH** | Preserved | System backbone; disabling results in unbootable/crashing OS. |
| **`DcomLaunch`** | DCOM Server Launcher | **DO NOT TOUCH** | Preserved | Required for COM/DCOM object activation across Windows. |
| **`Winmgmt`** | WMI Service | **DO NOT TOUCH** | Preserved | System management, driver interfaces, hardware monitoring. |
| **`BITS`** | Background Intelligent Transfer | **DO NOT TOUCH** | Preserved | Reliable asynchronous file transfer for Store and updates. |
| **`CryptSvc`** | Cryptographic Services | **DO NOT TOUCH** | Preserved | Certificate validation for game binaries, drivers, and TLS. |
| **`EventLog`** | Windows Event Log | **DO NOT TOUCH** | Preserved | Core OS diagnostics, crash logs, system auditing. |
| **`Dnscache`** | DNS Client | **DO NOT TOUCH** | Preserved | Network name resolution. |
| **`DiagTrack`** | Connected User Experiences | **SAFE CANDIDATE** | Disabled | Collects telemetry and diagnostic data for Microsoft servers. |
| **`WerSvc`** | Windows Error Reporting | **SAFE CANDIDATE** | Disabled | Generates and transmits crash dump data externally. |
| **`MapsBroker`** | Downloaded Maps Manager | **SAFE CANDIDATE** | Disabled | Unnecessary background resource usage for offline maps. |
| **`RetailDemo`** | Retail Demo Service | **SAFE CANDIDATE** | Disabled | Store showroom demo mode service; completely superfluous. |
| **`wisvc`** | Windows Insider Service | **SAFE CANDIDATE** | Disabled | Flighting and beta diagnostic data transmission. |
