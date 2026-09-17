// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use recipes::distro::Brogstrappa;
use tracing::{error, info};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliEntry {
    #[command(subcommand)]
    command: Commands,
}

// TODO: Add proper codes for our app xD
#[repr(i32)]
enum ExitCode {
    Normal = 0,
    Abnormal,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the distro
    Build { distro: Option<PathBuf> },
}

// Initialisation of the tracing registry
fn init_registry() {
    let ind = IndicatifLayer::new();
    let env = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

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
        Commands::Build { distro } => {
            let distro = distro.clone().unwrap_or(PathBuf::from("distro.kdl"));
            command_build(&distro)
        }
    };

    std::process::exit(exit_code as i32)
}

// Load distro config, handle emission of miette report if needed
fn command_build(path: &Path) -> ExitCode {
    let _distro = match Brogstrappa::from_path(&path) {
        Ok(d) => d,
        Err(e) => {
            let report = miette::Report::new(e);
            error!(config = ?path, "Failed to load bootstrap configuration");
            eprintln!("{report:?}");
            return ExitCode::Abnormal;
        }
    };
    info!(config = ?path, "Loaded distro configuration");
    ExitCode::Normal
}
