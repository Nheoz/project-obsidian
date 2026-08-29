# Contributing to Project Obsidian

We welcome contributions from systems engineers, kernel researchers, gamers, and AI practitioners!

## Contribution Rules & Standard
1. **Zero Placebo Policy**: We do not accept PRs adding tweaks that lack documented technical benchmarks or official vendor backing (Microsoft Learn, NVIDIA developer, CIS benchmarks).
2. **Never Touch Core Services**: Any PR attempting to disable `wuauserv` (Windows Update), `WinDefend`, `RpcSs`, `BITS`, or `CryptSvc` will be closed immediately.
3. **Atomic Reversibility**: Every registry or policy addition in `powershell/` or `src/` must be paired with corresponding rollback logic in `powershell/Policies.ps1` and `Restore-Obsidian.ps1`.
4. **Code Quality**:
   - Rust code must pass `cargo fmt --check` and `cargo clippy`.
   - PowerShell code must run under `Set-StrictMode -Version Latest`.

## Development Workflow
```bash
# Clone and build
git clone https://github.com/project-obsidian/obsidian.git
cd obsidian
cargo check
cargo test
cargo build --release

# Run dry-run analysis
.\target\release\obsidian.exe analyze
```
