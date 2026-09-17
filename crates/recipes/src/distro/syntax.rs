// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Syntax helpers / errors

use itertools::Itertools;
use kdl::KdlNode;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Some manner of syntax issue in our DSL atop KDL
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    /// Encountered an unexpected property, should be an ID
    #[error("unexpected property")]
    UnexpectedProperty {
        #[label("This should be an identifier, not a property")]
        span: SourceSpan,
    },

    #[error("node requires an ID")]
    ExpectedID {
        #[label("A valid string identifier needs to be provided")]
        span: SourceSpan,
    },

    #[error("unexpected argument count")]
    WrongArgumentCount {
        #[label("Wrong number of arguments, expected {expected} but got {found}")]
        span: SourceSpan,

        expected: usize,
        found: usize,
    },
}

/// Return a string ID or an error
pub(super) fn get_node_id(node: &KdlNode) -> Result<String, Error> {
    let attr = node
        .entries()
        .iter()
        .exactly_one()
        .map_err(|e| Error::WrongArgumentCount {
            span: node.span(),
            expected: 1,
            found: e.count(),
        })?;

    // make sure its not a property.
    if attr.name().is_some() {
        return Err(Error::UnexpectedProperty { span: attr.span() });
    }

    // make sure its stringy.
    let id = attr
        .value()
        .as_string()
        .ok_or_else(|| Error::ExpectedID { span: attr.span() })?;

    Ok(id.into())
}
