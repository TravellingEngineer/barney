// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Brogstrapa profiles

use itertools::Itertools;
use kdl::KdlNode;
use thiserror::Error;

#[derive(Debug)]
pub struct Profile {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("not yet implemented")]
    Unimplemented,

    #[error("missing name")]
    MissingName,

    #[error("expected one entry only")]
    ExactlyOne,
}

impl Profile {
    pub(super) fn from_node(node: &KdlNode) -> Result<Self, Error> {
        let attr = node
            .entries()
            .iter()
            .exactly_one()
            .map_err(|_| Error::ExactlyOne)?;

        // TODO: ParserError::UnexpectedProperty
        if attr.name().is_some() {
            return Err(Error::MissingName);
        }

        // TODO: ParserError::UnexpectedType
        if !attr.value().is_string() {
            return Err(Error::MissingName);
        }

        let a = attr.value().as_string().ok_or(Error::MissingName)?;
        eprintln!("Got a profile \"{}\"", a);

        Err(Error::Unimplemented)
    }
}
