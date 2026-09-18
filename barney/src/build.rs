// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Build queue entry point for Barney CLI

use std::path::Path;

use recipes::bootstrap::BootstrapSpec;
use tracing::{error, info};
use tracing_indicatif::suspend_tracing_indicatif;

use crate::ExitCode;

// Load bootstrap config, handle emission of miette report if needed
pub async fn run_command(path: &Path) -> ExitCode {
    // Ensure we have a real path firstly!
    let path = match path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            error!(path = ?&path, error = ?e, "Failed to canonicalize config path");
            return ExitCode::Abnormal;
        }
    };

    // Load the bootstrap configuration
    let distro = match BootstrapSpec::from_path(&path) {
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
    info!(?distro, config = ?path, "Loaded distro configuration");
    ExitCode::Normal
}
