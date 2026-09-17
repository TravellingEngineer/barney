// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use barney::build;
use clap::{Parser, Subcommand};
use tokio::runtime;
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

/// Guarded main to enable future workers without bricking the
/// architecture.
fn main() {
    // TODO: Slightly more graceful init of tokio please
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Run internal async enabled runtime
    rt.block_on(async move { priv_async_main().await });
}

/// Main entry point
async fn priv_async_main() {
    init_registry();
    let cli = CliEntry::parse();

    let exit_code = match &cli.command {
        // handle build command
        Commands::Build { path } => {
            let path = path.clone().unwrap_or(PathBuf::from("bootstrap.kdl"));
            build::run_command(&path).await
        }
    };

    std::process::exit(exit_code as i32)
}
