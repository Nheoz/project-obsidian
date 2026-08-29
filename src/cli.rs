use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "obsidian",
    author = "Antigravity Engineering <contact@project-obsidian.dev>",
    version = "1.0.0",
    about = "Forge Windows 11 into a Privacy-First AI & Gaming Workstation",
    long_about = "Project Obsidian is an enterprise-grade hybrid optimization suite that safely configures \
Windows 11 for gaming and AI development with zero placebo tweaks and guaranteed reversibility."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Output in JSON format for automated pipelines
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Verbose diagnostic output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Perform a non-destructive dry-run analysis of the current system
    Analyze,

    /// Audit AI runtimes, CUDA, WSL2, Docker, and gaming prerequisites
    Doctor,

    /// Apply optimization profile to Windows 11
    Apply {
        /// Profile to apply
        #[arg(short, long, value_enum, default_value_t = ProfileType::Ultimate)]
        profile: ProfileType,

        /// Perform a dry-run without writing any changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Measure system performance (RAM, CPU, Processes, Threads, Latency)
    Benchmark {
        /// Label for this benchmark run (e.g. 'before', 'after')
        #[arg(short, long, default_value = "current")]
        label: String,
    },

    /// Execute post-flight validation to ensure zero system breakage
    Validate,

    /// Revert system to exact state captured before Obsidian was applied
    Restore {
        /// Optional path to a specific snapshot JSON file
        #[arg(short, long)]
        snapshot: Option<PathBuf>,
    },

    /// Display current privacy, gaming, and AI readiness status
    Status,

    /// Export comprehensive diagnostic and configuration report
    Export {
        /// Output path for the generated report
        #[arg(short, long, default_value = "report.md")]
        output: PathBuf,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, serde::Serialize, serde::Deserialize)]
pub enum ProfileType {
    /// Strict privacy hardening, telemetry reduction, and ad blocking
    Privacy,
    /// Low-latency gaming optimizations without breaking anticheats
    Gaming,
    /// AI developer acceleration (CUDA, WSL2, Docker, Tensor Cores)
    Ai,
    /// General developer workstation configuration
    Developer,
    /// Unified combination of all safe profiles
    Ultimate,
}
