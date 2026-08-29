use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const REGISTRY_PS: &str = include_str!("../powershell/Registry.ps1");
pub const SERVICES_PS: &str = include_str!("../powershell/Services.ps1");
pub const TASKS_PS: &str = include_str!("../powershell/ScheduledTasks.ps1");
pub const POLICIES_PS: &str = include_str!("../powershell/Policies.ps1");
pub const VALIDATION_PS: &str = include_str!("../powershell/Validation.ps1");
pub const FEATURES_PS: &str = include_str!("../powershell/WindowsFeatures.ps1");
pub const RESTORE_PS: &str = include_str!("../Restore-Obsidian.ps1");

/// Ensures that Project Obsidian PowerShell scripts exist on disk,
/// extracting the embedded assets if the user is running the binary standalone.
pub fn get_scripts_root() -> PathBuf {
    // 1. Check current working directory
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
    let temp_root = env::temp_dir().join("ProjectObsidian");
    let ps_dir = temp_root.join("powershell");
    let _ = fs::create_dir_all(&ps_dir);

    let _ = fs::write(ps_dir.join("Registry.ps1"), REGISTRY_PS);
    let _ = fs::write(ps_dir.join("Services.ps1"), SERVICES_PS);
    let _ = fs::write(ps_dir.join("ScheduledTasks.ps1"), TASKS_PS);
    let _ = fs::write(ps_dir.join("Policies.ps1"), POLICIES_PS);
    let _ = fs::write(ps_dir.join("Validation.ps1"), VALIDATION_PS);
    let _ = fs::write(ps_dir.join("WindowsFeatures.ps1"), FEATURES_PS);
    let _ = fs::write(temp_root.join("Restore-Obsidian.ps1"), RESTORE_PS);

    temp_root
}
