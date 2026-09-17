// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap phase

use kdl::KdlNode;
use miette::Diagnostic;
use thiserror::Error;
use tracing::trace;

use crate::distro::syntax;

#[derive(Debug)]
pub struct Phase {
    id: String,
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Syntax(#[from] syntax::Error),
}

impl Phase {
    /// Build a distro::Phase from a KdlNode
    pub(super) fn from_node(node: &KdlNode) -> Result<Self, Error> {
        let name = syntax::get_node_id(node)?;
        trace!(name = name, "parsing bootstrap phase");

        Ok(Self { id: name })
    }

    /// Returns the phase ID
    pub fn id(&self) -> &str {
        &self.id
    }
}
