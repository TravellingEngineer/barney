// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use recipes::distro::BootstrapSpec;
use tracing::{error, info};
use tracing_indicatif::{IndicatifLayer, suspend_tracing_indicatif};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliEntry {
    #[command(subcommand)]
    command: Commands,
}

// TODO: Add proper codes for our app xD
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExitCode {
    Normal = 0,
    Abnormal,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the distro
    Build { path: Option<PathBuf> },
}

// Initialisation of the tracing registry
fn init_registry() {
    let ind = IndicatifLayer::new();
    let env = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));

    // TODO: Use some kind of timer
    tracing_subscriber::registry()
        .with(env)
        .with(
            fmt::layer()
                .with_writer(ind.get_stderr_writer())
                .pretty()
                .compact()
                .with_file(false)
                .without_time()
                .with_line_number(false),
        )
        .with(ind)
        .init();
}

/// Main entry point
#[tokio::main]
async fn main() {
    init_registry();
    let cli = CliEntry::parse();

    let exit_code = match &cli.command {
        // handle build command
        Commands::Build { path } => {
            let path = path.clone().unwrap_or(PathBuf::from("bootstrap.kdl"));
            command_build(&path).await
        }
    };

    std::process::exit(exit_code as i32)
}

// Load bootstrap config, handle emission of miette report if needed
async fn command_build(path: &Path) -> ExitCode {
    // Ensure we have a real path firstly!
    let path = match path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            error!(path = ?&path, error = ?e, "Failed to canonicalize config path");
            return ExitCode::Abnormal;
        }
    };

    // Load the bootstrap configuration
    let _distro = match BootstrapSpec::from_path(&path) {
        Ok(d) => d,
        Err(e) => {
            let report = miette::Report::new(e);
            error!(config = ?path, "Failed to load bootstrap configuration");
            suspend_tracing_indicatif(|| {
                eprintln!("{report:?}");
            });
            return ExitCode::Abnormal;
        }
    };
    info!(config = ?path, "Loaded distro configuration");
    ExitCode::Normal
}
