// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use recipes::distro::Brogstrappa;
use tracing::info;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliEntry {
    #[command(subcommand)]
    command: Commands,
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
async fn main() -> miette::Result<()> {
    init_registry();
    info!("Loading registry");

    let cli = CliEntry::parse();

    match &cli.command {
        Commands::Build { distro } => {
            // TODO: Only use miette result reporting for where it matters, ie loading KDL
            let distro = distro.clone().unwrap_or(PathBuf::from("distro.kdl"));
            let distro = Brogstrappa::from_path(&distro)?;
            eprintln!("Have distro: {distro:?}");
        }
    }
    Ok(())
}
