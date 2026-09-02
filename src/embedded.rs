use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Version embedded at compile time — used to invalidate stale TEMP cache
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REGISTRY_PS: &str = include_str!("../powershell/Registry.ps1");
pub const SERVICES_PS: &str = include_str!("../powershell/Services.ps1");
pub const TASKS_PS: &str = include_str!("../powershell/ScheduledTasks.ps1");
pub const POLICIES_PS: &str = include_str!("../powershell/Policies.ps1");
pub const VALIDATION_PS: &str = include_str!("../powershell/Validation.ps1");
pub const FEATURES_PS: &str = include_str!("../powershell/WindowsFeatures.ps1");
pub const RESTORE_PS: &str = include_str!("../Restore-Obsidian.ps1");

/// Returns the version stamp written inside the TEMP cache directory.
fn cache_version_file(temp_root: &Path) -> PathBuf {
    temp_root.join(".obsidian_version")
}

/// Returns true if the scripts currently cached in TEMP were written by a
/// different binary version (or if no version stamp exists at all).
fn cache_is_stale(temp_root: &Path) -> bool {
    let stamp = cache_version_file(temp_root);
    match fs::read_to_string(&stamp) {
        Ok(v) => v.trim() != VERSION,
        Err(_) => true, // no stamp → definitely stale
    }
}

/// Writes all embedded scripts to temp_root, then stamps the version.
fn extract_to_temp(temp_root: &Path) {
    let ps_dir = temp_root.join("powershell");
    let _ = fs::create_dir_all(&ps_dir);

    let _ = fs::write(ps_dir.join("Registry.ps1"), REGISTRY_PS);
    let _ = fs::write(ps_dir.join("Services.ps1"), SERVICES_PS);
    let _ = fs::write(ps_dir.join("ScheduledTasks.ps1"), TASKS_PS);
    let _ = fs::write(ps_dir.join("Policies.ps1"), POLICIES_PS);
    let _ = fs::write(ps_dir.join("Validation.ps1"), VALIDATION_PS);
    let _ = fs::write(ps_dir.join("WindowsFeatures.ps1"), FEATURES_PS);
    let _ = fs::write(temp_root.join("Restore-Obsidian.ps1"), RESTORE_PS);

    // Stamp the version so next run knows these scripts are fresh
    let _ = fs::write(cache_version_file(temp_root), VERSION);
}

/// Ensures that Project Obsidian PowerShell scripts exist on disk,
/// extracting the embedded assets if the user is running the binary standalone.
/// Automatically re-extracts if the cached version is from an older binary.
pub fn get_scripts_root() -> PathBuf {
    // 1. Check current working directory — dev/distribution mode with scripts alongside exe
    if Path::new("powershell").join("Services.ps1").exists() {
        return PathBuf::from(".");
    }

    // 2. Check directory where the executable resides
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            if parent.join("powershell").join("Services.ps1").exists() {
                return parent.to_path_buf();
            }
        }
    }

    // 3. Fallback: Extract embedded scripts to %TEMP%\ProjectObsidian
    //    Re-extract if the cached scripts are from an older binary version.
    let temp_root = env::temp_dir().join("ProjectObsidian");

    if cache_is_stale(&temp_root) {
        extract_to_temp(&temp_root);
    }

    temp_root
}
