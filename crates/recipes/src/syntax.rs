// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Syntax helpers / errors

use std::{fmt::Debug, hash::Hash};

use kdl::KdlDocument;
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
pub struct NodeSpec<'a, I>
where
    I: Into<usize> + Eq + PartialEq + PartialOrd + Hash + Debug,
{
    /// The matching name for the node, ie `NodeName::Static("variables")`
    pub name: NodeName,

    /// Identity for the node to retain context in processing
    pub identity: I,

    /// Supported argument kinds
    pub args: ArgSpec,

    /// When not empty, limit the allowed properties
    pub props: &'a [PropSpec],

    /// Children of the node, if permitted
    pub children: &'a [NodeSpec<'a, I>],
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

/// Process KDL according to the given rule set
pub(super) fn process_kdl<'a, I>(
    _document: &KdlDocument,
    _rules: &[&NodeSpec<'a, I>],
) -> Result<(), Error>
where
    I: Into<usize> + Eq + PartialEq + PartialOrd + Hash + Debug,
{
    unimplemented!("Whoops!")
}
