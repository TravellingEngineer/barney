// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Distro / OS definition cruft

use std::{fs, io, path::Path, str::FromStr};

use kdl::{KdlDocument, KdlError};
use thiserror::Error;

mod profile;
pub use profile::Profile;

/// A Brogstrappa definition is taken from a brogstrappa.kdl
/// You can blame Arjan van de Ven for this name
///
#[derive(Debug)]
pub struct Brogstrappa {
    _profiles: Vec<Profile>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    Parsing(#[from] KdlError),

    #[error("profile")]
    Profile(#[from] profile::Error),

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

    /// Load a Brogstrappa definition (into AST) from a valid KDL document
    /// using the correct procedural lingo.
    pub fn new(doc: &KdlDocument) -> Result<Self, Error> {
        let mut nodes = vec![];
        for node in doc.nodes() {
            eprintln!("node: {}", node.name().value());
            match node.name().value() {
                "module" => {}
                "profile" => {
                    let node = Profile::from_node(node)?;
                    nodes.push(node);
                }
                _ => {}
            }
        }

        Err(Error::Unimplemented)
    }
}
