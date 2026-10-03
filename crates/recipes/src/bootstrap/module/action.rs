// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap configuration `module` actions

/// An Action as defined in the modules exported by various
/// KDL files.
/// These actions provide a modern alternative to macro-rich
/// string soup approaches of the past, providing a dynamic
/// lookup table of functors with Rust-style namespace resolution.
///
/// Unlike other action approaches this all boils down to shell script
/// emission with chain of custody and hashing as the central design
/// tenants.
#[derive(Debug)]
pub struct Action {
    pub(super) id: String,
    // args:
    // execute:
}
