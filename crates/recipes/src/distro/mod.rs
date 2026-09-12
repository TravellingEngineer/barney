// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Distro / OS definition cruft

use std::{fs, io, path::Path, str::FromStr};

use kdl::{KdlDocument, KdlError};
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

    #[error(transparent)]
    Parsing(#[from] KdlError),

    #[error("unimplemented")]
    Unimplemented,
}

impl FromStr for Brogstrappa {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kdl_doc = KdlDocument::parse_v2(s)?;
        Brogstrappa::new(&kdl_doc)
    }
}

impl Brogstrappa {
    pub fn from_path(whence: &impl AsRef<Path>) -> Result<Self, Error> {
        let contents = fs::read_to_string(whence)?;
        let brog = contents.parse::<Brogstrappa>()?;
        Ok(brog)
    }

    pub fn new(_doc: &KdlDocument) -> Result<Self, Error> {
        Err(Error::Unimplemented)
    }
}
