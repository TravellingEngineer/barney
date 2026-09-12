// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Brogstrapa profiles

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
}

impl Profile {
    pub(super) fn from_node(_node: &KdlNode) -> Result<Self, Error> {
        Err(Error::Unimplemented)
    }
}
