// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Distro / OS definition cruft

use std::{fs, io, path::Path};

use kdl::{KdlDocument, KdlError};
use miette::{Diagnostic, NamedSource};
use thiserror::Error;

mod phase;
pub use phase::Phase;
pub mod syntax;

/// A distro definition is taken from a bootstrap.kdl
///
pub struct BootstrapSpec {
    phases: Vec<Phase>,
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

    // Phase DSL
    #[error("Phase parsing")]
    #[diagnostic()]
    Phase {
        #[source_code]
        src: NamedSource<String>,

        #[diagnostic_source(transparent)]
        source: phase::Error,
    },

    // IDK
    #[error("unimplemented")]
    Unimplemented,
}

impl BootstrapSpec {
    pub fn from_path(whence: &impl AsRef<Path>) -> Result<Self, Error> {
        let whence_path = whence.as_ref().to_string_lossy().to_string();
        let contents = fs::read_to_string(&whence_path)?;

        let source_code = NamedSource::new(whence_path.clone(), contents);

        let kdl_doc = KdlDocument::parse_v2(source_code.inner()).map_err(|e| Error::KDL {
            src: source_code.clone(),
            source: e,
        })?;
        BootstrapSpec::new(&source_code, &kdl_doc)
    }

    /// Load a bootstrap definition (into AST) from a valid KDL document
    /// using the correct procedural lingo.
    pub fn new(source: &NamedSource<String>, doc: &KdlDocument) -> Result<Self, Error> {
        let mut phases = vec![];
        for node in doc.nodes() {
            match node.name().value() {
                "module" => {}
                "phase" => {
                    let node = Phase::from_node(node).map_err(|e| Error::Phase {
                        src: source.clone(),
                        source: e,
                    })?;
                    phases.push(node);
                }
                _ => {}
            }
        }

        Ok(Self { phases })
    }

    /// Access the underlying phases
    pub fn phases(&self) -> &[Phase] {
        self.phases.as_slice()
    }
}
