// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Brogstrapa profiles

use kdl::KdlNode;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use tracing::trace;

use crate::distro::syntax;

#[derive(Debug)]
pub struct Profile {}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    /// TODO: Set fire to this crap
    #[error("not yet implemented")]
    #[diagnostic()]
    Unimplemented {
        #[label("not yet implemented!")]
        span: SourceSpan,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Syntax(#[from] syntax::Error),
}

impl Profile {
    /// Build a distro::Profile from a KdlNode
    #[tracing::instrument]
    pub(super) fn from_node(node: &KdlNode) -> Result<Self, Error> {
        let name = syntax::get_node_id(node)?;
        trace!(name = name, "loading profile");
        Err(Error::Unimplemented { span: node.span() })
    }
}
