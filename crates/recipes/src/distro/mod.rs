// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Distro / OS definition cruft

use std::{io, path::Path};

use thiserror::Error;

/// A Brogstrappa definition is taken from a brogstrappa.kdl
/// You can blame Arjan van de Ven for this name
///
#[derive(Debug)]
pub struct Brogstrappa {}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("unimplemented")]
    Unimplemented,
}

impl Brogstrappa {
    pub fn from_path(_whence: &impl AsRef<Path>) -> Result<Self, Error> {
        Err(Error::Unimplemented)
    }
}
