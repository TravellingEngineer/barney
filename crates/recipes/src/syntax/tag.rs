// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Strong types can have tags associated with them
//! which also permit a referencing system
//! Internally they're still strings

use std::{fmt::Display, str::FromStr};

use thiserror::Error;

/// Arguments and indeed properties may be decorated with
/// a type, i.e a tag, to help control the the intended type
/// or indeed reference another argument or variable by the FQDN
/// accessor method.
#[derive(Debug)]
pub enum Tag {
    /// Local argument `(arg)` reference
    Argument,

    /// Qualified variable `(var)` reference
    Variable,

    Type(Type),
}

#[derive(Debug, Error)]
pub enum TagError {
    #[error("unknown tag: {0}")]
    UnknownTag(String),
}

impl FromStr for Tag {
    type Err = TagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arg" => Ok(Tag::Argument),
            "var" => Ok(Tag::Variable),
            "string" => Ok(Tag::Type(Type::String)),
            "list" => Ok(Tag::Type(Type::List)),
            _ => Err(TagError::UnknownTag(s.to_string())),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            Tag::Argument => "arg",
            Tag::Variable => "var",
            Tag::Type(t) => match t {
                Type::String => "string",
                Type::List => "list",
            },
        };
        f.write_str(r)
    }
}

/// Explicitly set type tag
#[derive(Debug)]
pub enum Type {
    String,
    List,
}
