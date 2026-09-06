// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliEntry {}

fn main() {
    let _ = CliEntry::parse();
}
