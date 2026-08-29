# Disaster Recovery & Rollback Engine

## Atomic Snapshot Model
Every modification applied by Project Obsidian is preceded by a snapshot transaction stored in `obsidian-state/snapshot-YYYYMMDD-HHMMSS.json`.

### Snapshot Schema:
```json
{
  "timestamp": "2026-08-29T18:25:00Z",
  "profile": "Ultimate Workstation",
  "windows": { ... },
  "hardware": { ... },
  "registry_items": [
    {
      "path": "HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection",
      "name": "AllowTelemetry",
      "previous_exists": true,
      "previous_value": 1,
      "previous_type": "DWord"
    }
  ],
  "services": [
    {
      "name": "DiagTrack",
      "previous_startup": "Automatic",
      "previous_status": "Running"
    }
  ],
  "tasks": [
    {
      "path": "\\Microsoft\\Windows\\Application Experience\\",
      "name": "Microsoft Compatibility Appraiser",
      "previous_state": "Ready"
    }
  ]
}
```

## How to Restore
### Via Rust CLI:
```powershell
obsidian restore
# or specify an exact snapshot:
obsidian restore --snapshot obsidian-state\snapshot-20260829-182500.json
```

### Via Standalone PowerShell Script (Works even if Rust binary is missing):
```powershell
.\Restore-Obsidian.ps1
```

### Guarantees:
- Restores exact previous startup types and running states of services.
- Restores original registry values or deletes keys that were newly created.
- Re-enables scheduled tasks that were previously active.
