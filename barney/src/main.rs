// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use recipes::distro::Brogstrappa;

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

fn main() -> miette::Result<()> {
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
