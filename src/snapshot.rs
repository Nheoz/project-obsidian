use crate::hardware::HardwareInfo;
use crate::windows::WindowsInfo;
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: String,
    pub profile: String,
    pub windows: WindowsInfo,
    pub hardware: HardwareInfo,
    pub registry_items: Vec<SnapshotRegistryItem>,
    pub services: Vec<SnapshotServiceItem>,
    pub tasks: Vec<SnapshotTaskItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotRegistryItem {
    pub path: String,
    pub name: String,
    pub previous_exists: bool,
    pub previous_value: Option<serde_json::Value>,
    pub previous_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotServiceItem {
    pub name: String,
    pub previous_startup: String,
    pub previous_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotTaskItem {
    pub path: String,
    pub name: String,
    pub previous_state: String,
}

impl Snapshot {
    pub fn new(profile: &str, windows: WindowsInfo, hardware: HardwareInfo) -> Self {
        Snapshot {
            timestamp: Utc::now().to_rfc3339(),
            profile: profile.to_string(),
            windows,
            hardware,
            registry_items: Vec::new(),
            services: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn save(&self, directory: &Path) -> Result<PathBuf> {
        if !directory.exists() {
            fs::create_dir_all(directory).context("Failed to create snapshot directory")?;
        }
        let now_str = Utc::now().format("%Y%m%d-%H%M%S").to_string();
        let file_path = directory.join(format!("snapshot-{}.json", now_str));
        let json = serde_json::to_string_pretty(self).context("Failed to serialize snapshot")?;
        fs::write(&file_path, json).context("Failed to write snapshot file")?;
        Ok(file_path)
    }

    pub fn load_latest(directory: &Path) -> Result<Option<(PathBuf, Snapshot)>> {
        if !directory.exists() {
            return Ok(None);
        }
        let mut entries: Vec<PathBuf> = fs::read_dir(directory)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().map_or(false, |ext| ext == "json")
                    && p.file_name()
                        .map_or(false, |n| n.to_string_lossy().starts_with("snapshot-"))
            })
            .collect();

        entries.sort();
        if let Some(latest) = entries.last() {
            let content = fs::read_to_string(latest)?;
            let snap: Snapshot = serde_json::from_str(&content)?;
            Ok(Some((latest.clone(), snap)))
        } else {
            Ok(None)
        }
    }
}
