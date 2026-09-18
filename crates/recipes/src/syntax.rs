// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Syntax helpers / errors

use itertools::Itertools;
use kdl::KdlNode;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Node identifier rules
#[derive(Debug)]
pub enum NodeName {
    /// A fixed identifier is required
    Static(&'static str),

    /// Arbitrary name for the node supported
    Dynamic,
}

/// Control evaluation of nodes to enforce schema
#[derive(Debug)]
pub struct NodeSpec<'a> {
    /// The matching name for the node, ie `NodeName::Static("variables")`
    pub name: NodeName,

    /// Supported argument kinds
    pub args: ArgSpec,

    /// When not empty, limit the allowed properties
    pub props: &'a [PropSpec],

    /// Children of the node, if permitted
    pub children: &'a [NodeSpec<'a>],
}

/// Argument spec for blocks
///
/// Determines the policy for processing arguments to a node,
/// for an ID based node this would be `ArgSpec::Exactly(1)`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArgSpec {
    /// Does not support any arguments
    None,

    /// Exact number of arguments
    Exactly(usize),

    /// Any number of arguments.
    Variable,
}

/// Controls property evaluation (which are always built into maps)
/// TODO: Add type filtering here for the value kind
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PropSpec {
    /// The expected name of the property
    pub name: &'static str,
}

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

    #[error("unexpected identifier")]
    UnexpectedIdentifier {
        #[label("Encountered an unexpected identifier or keyword")]
        span: SourceSpan,
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
