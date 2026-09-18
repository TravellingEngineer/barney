// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Distro / OS definition cruft

use std::{fs, io, path::Path};

use kdl::{KdlDocument, KdlError};
use miette::{Diagnostic, NamedSource};
use thiserror::Error;

mod phase;
pub use phase::Phase;

use crate::syntax::{self, NodeSpec};

/// A distro definition is taken from a bootstrap.kdl
///
#[derive(Debug)]
pub struct BootstrapSpec {
    phases: Vec<Phase>,
}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub(crate) enum SpecIdentity {
    /// Root phase
    Phase,
    /// The `variables` block itself
    PhaseVariables,
    /// Some variable in the phase variables list
    PhaseVariable,
}

impl From<SpecIdentity> for usize {
    fn from(value: SpecIdentity) -> Self {
        value as usize
    }
}

// Our entire schema is a composite of loader rules by way of NodeSpec sets
static RULES: &[&NodeSpec<'static, SpecIdentity>] = &[&phase::RULES];

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

    #[error("Syntax parsing")]
    #[diagnostic()]
    Syntax {
        #[source_code]
        src: NamedSource<String>,

        #[diagnostic_source(transparent)]
        source: syntax::Error,
    },
}

impl BootstrapSpec {
    /// Load a bootstrap spec from the given path
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
        // TODO: Pivot to process_kdl API
        let nodes = syntax::process_kdl(doc, RULES).map_err(|e| Error::Syntax {
            src: source.clone(),
            source: e,
        })?;

        let mut phases = vec![];
        for node in nodes.into_iter() {
            match node.identity {
                SpecIdentity::Phase => {
                    phases.push(Phase::new(node));
                }
                _ => panic!("unsupported descent"),
            }
        }

        Ok(Self { phases })
    }

    /// Access the underlying phases
    pub fn phases(&self) -> &[Phase] {
        self.phases.as_slice()
    }
}
