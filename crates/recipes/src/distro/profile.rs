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
pub enum Error {
    #[error("Profiles can have one name only")]
    ExactlyOne {
        #[label("set a single name for this profile")]
        span: SourceSpan,
    },

    #[error("not yet implemented")]
    #[diagnostic()]
    Unimplemented {
        #[label("not yet implemented!")]
        span: SourceSpan,
    },
}

impl Profile {
    pub(super) fn from_node(node: &KdlNode) -> Result<Self, Error> {
        let attr = node
            .entries()
            .iter()
            .exactly_one()
            .map_err(|_| Error::ExactlyOne { span: node.span() })?;

        // TODO: ParserError::UnexpectedProperty
        if attr.name().is_some() {
            return Err(Error::Unimplemented { span: node.span() });
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
