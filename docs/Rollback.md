# Rollback & State Recovery

Project Obsidian features an enterprise-grade atomic rollback engine. It never makes a permanent change without first recording the exact previous state of the system.

## How it Works (v2.0)

1. **Pre-flight Capture:** Before modifying a registry key, service, or task, Obsidian queries the live OS using native PowerShell APIs.
2. **Atomic Snapshots:** The precise values (e.g., Value: 1, Type: DWord, Startup: Manual) are serialized into a JSON snapshot located in the obsidian-state/ directory.
3. **Restoration:** When obsidian restore is called, the engine parses the latest JSON and reverses every change.

## Integrity Verification (New in v2.0)
After a rollback is executed, the recovery engine automatically re-queries the system to verify that the target services and tasks actually reverted to their original states. If a service gets stuck, the engine flags it in red in the console output.

## Standalone Fallback
Because the rollback logic is embedded in Restore-Obsidian.ps1, you can restore your system even if the Rust binary is deleted. Simply right-click Restore-Obsidian.ps1 and run with PowerShell.
