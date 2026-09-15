// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Brogstrapa profiles

use itertools::Itertools;
use kdl::KdlNode;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug)]
pub struct Profile {}

#[derive(Debug, Error, Diagnostic)]
pub enum SyntaxError {
    #[error("unexpected property")]
    UnexpectedProperty {
        #[label("This should not be a property")]
        span: SourceSpan,
    },
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    /// Error with the profiles name
    #[error("Profiles must have a valid name")]
    ProfileName {
        #[label("set a single name for this profile")]
        span: SourceSpan,
    },

    /// TODO: Set fire to this crap
    #[error("not yet implemented")]
    #[diagnostic()]
    Unimplemented {
        #[label("not yet implemented!")]
        span: SourceSpan,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Syntax(#[from] SyntaxError),
}

impl Profile {
    /// Build a distro::Profile from a KdlNode
    pub(super) fn from_node(node: &KdlNode) -> Result<Self, Error> {
        let attr = node
            .entries()
            .iter()
            .exactly_one()
            .map_err(|_| Error::ProfileName { span: node.span() })?;

        // TODO: ParserError::UnexpectedProperty
        if attr.name().is_some() {
            return Err(SyntaxError::UnexpectedProperty { span: node.span() })?;
        }

        // TODO: ParserError::UnexpectedType
        if !attr.value().is_string() {
            return Err(Error::Unimplemented { span: node.span() });
        }

        let _a = attr
            .value()
            .as_string()
            .ok_or(Error::Unimplemented { span: node.span() })?;

        Err(Error::Unimplemented { span: node.span() })
    }
}
