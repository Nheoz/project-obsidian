use crate::cli::ProfileType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationProfile {
    pub profile_type: ProfileType,
    pub name: String,
    pub description: String,
    pub enable_privacy: bool,
    pub enable_gaming: bool,
    pub enable_ai_doctor: bool,
    pub enable_developer: bool,
}

impl OptimizationProfile {
    pub fn from_type(p_type: ProfileType) -> Self {
        match p_type {
            ProfileType::Privacy => OptimizationProfile {
                profile_type: p_type,
                name: "Privacy Hardening".to_string(),
                description: "Maximizes privacy by strictly limiting telemetry, advertising IDs, and telemetry tasks.".to_string(),
                enable_privacy: true,
                enable_gaming: false,
                enable_ai_doctor: false,
                enable_developer: false,
            },
            ProfileType::Gaming => OptimizationProfile {
                profile_type: p_type,
                name: "Gaming Optimization".to_string(),
                description: "Optimizes latency, maintains Game Mode and HAGS, removes background clutter without affecting anticheats.".to_string(),
                enable_privacy: false,
                enable_gaming: true,
                enable_ai_doctor: false,
                enable_developer: false,
            },
            ProfileType::Ai => OptimizationProfile {
                profile_type: p_type,
                name: "AI Workstation".to_string(),
                description: "Audits CUDA, NVIDIA drivers, WSL2, Docker, Python, and local model runtimes.".to_string(),
                enable_privacy: false,
                enable_gaming: false,
                enable_ai_doctor: true,
                enable_developer: true,
            },
            ProfileType::Developer => OptimizationProfile {
                profile_type: p_type,
                name: "Developer Workstation".to_string(),
                description: "Configures developer subsystems (WSL2, Git, Docker, Windows Terminal, Python).".to_string(),
                enable_privacy: true,
                enable_gaming: false,
                enable_ai_doctor: true,
                enable_developer: true,
            },
            ProfileType::Ultimate => OptimizationProfile {
                profile_type: p_type,
                name: "Ultimate Workstation".to_string(),
                description: "Combines privacy, gaming responsiveness, and AI development readiness in a single coherent profile.".to_string(),
                enable_privacy: true,
                enable_gaming: true,
                enable_ai_doctor: true,
                enable_developer: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_configuration_flags() {
        let privacy = OptimizationProfile::from_type(ProfileType::Privacy);
        assert!(privacy.enable_privacy);
        assert!(!privacy.enable_gaming);

        let gaming = OptimizationProfile::from_type(ProfileType::Gaming);
        assert!(!gaming.enable_privacy);
        assert!(gaming.enable_gaming);

        let ultimate = OptimizationProfile::from_type(ProfileType::Ultimate);
        assert!(ultimate.enable_privacy);
        assert!(ultimate.enable_gaming);
        assert!(ultimate.enable_ai_doctor);
        assert!(ultimate.enable_developer);
    }
}
