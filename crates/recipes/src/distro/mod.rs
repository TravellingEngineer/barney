// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Distro / OS definition cruft

use std::{fs, io, path::Path};

use kdl::{KdlDocument, KdlError};
use miette::{Diagnostic, NamedSource};
use thiserror::Error;

mod profile;
pub use profile::Profile;
pub mod syntax;

/// A Brogstrappa definition is taken from a brogstrappa.kdl
/// You can blame Arjan van de Ven for this name
///
#[derive(Debug)]
pub struct Brogstrappa {
    _profiles: Vec<Profile>,
}

#[derive(Diagnostic, Error, Debug)]
#[diagnostic()]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    // KDL raw parser issues
    #[error("Error parsing {}", src.name())]
    #[diagnostic()]
    KDL {
        #[source_code]
        src: NamedSource<String>,

        #[diagnostic_source]
        source: KdlError,
    },

    // Profile DSL
    #[error("Profile parsing")]
    #[diagnostic()]
    Profile {
        #[source_code]
        src: NamedSource<String>,

        #[diagnostic_source(transparent)]
        source: profile::Error,
    },

    // IDK
    #[error("unimplemented")]
    Unimplemented,
}

impl Brogstrappa {
    pub fn from_path(whence: &impl AsRef<Path>) -> Result<Self, Error> {
        let whence_path = whence.as_ref().to_string_lossy().to_string();
        let contents = fs::read_to_string(&whence_path)?;

        let source_code = NamedSource::new(whence_path.clone(), contents);

        let kdl_doc = KdlDocument::parse_v2(source_code.inner()).map_err(|e| Error::KDL {
            src: source_code.clone(),
            source: e,
        })?;
        Brogstrappa::new(&source_code, &kdl_doc)
    }

    /// Load a Brogstrappa definition (into AST) from a valid KDL document
    /// using the correct procedural lingo.
    pub fn new(source: &NamedSource<String>, doc: &KdlDocument) -> Result<Self, Error> {
        let mut nodes = vec![];
        for node in doc.nodes() {
            eprintln!("node: {}", node.name().value());
            match node.name().value() {
                "module" => {}
                "profile" => {
                    let node = Profile::from_node(node).map_err(|e| Error::Profile {
                        src: source.clone(),
                        source: e,
                    })?;
                    nodes.push(node);
                }
                _ => {}
            }
        }

        Err(Error::Unimplemented)
    }
}
