// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap phase

use kdl::KdlNode;
use miette::Diagnostic;
use thiserror::Error;
use tracing::trace;

use crate::syntax::{self, ArgSpec, NodeName, NodeSpec};

#[derive(Debug)]
pub struct Phase {
    id: String,
}

/// rules for loading phase nodes
pub(super) static RULES: NodeSpec<'static> = NodeSpec {
    name: NodeName::Static("phase"),
    // Single argument: ID
    args: ArgSpec::Exactly(1),
    // No properties permitted
    props: &[],
    // Only allow one child node: `variables`
    children: &[NodeSpec {
        name: NodeName::Static("variables"),
        args: ArgSpec::None,
        props: &[],
        // Arbitrary children due to names
        children: &[NodeSpec {
            // User defined name
            name: NodeName::Dynamic,
            // Variables have one argument, the value
            args: ArgSpec::Exactly(1),
            props: &[],
            children: &[],
        }],
    }],
};

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

        // ensure we have valid children only
        for child in node.iter_children() {
            match child.name().value() {
                "variables" => {}
                _ => return Err(syntax::Error::UnexpectedIdentifier { span: child.span() })?,
            }
        }

        Ok(Self { id: name })
    }

    /// Returns the phase ID
    pub fn id(&self) -> &str {
        &self.id
    }
}
