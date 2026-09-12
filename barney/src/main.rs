// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

fn main() {
    let cli = CliEntry::parse();

    match &cli.command {
        Commands::Build { distro: _ } => todo!(),
    }
}
