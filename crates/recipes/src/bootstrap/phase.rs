// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap phase

use std::collections::HashMap;

use crate::{
    bootstrap::SpecIdentity,
    syntax::{ArgSpec, NodeName, NodeSpec, ProcessedNode},
};

#[derive(Debug)]
pub struct Phase {
    id: String,
    _vars: HashMap<String, String>,
}

/// rules for loading phase nodes
pub(super) static RULES: NodeSpec<'static, SpecIdentity> = NodeSpec {
    name: NodeName::Static("phase"),
    identity: SpecIdentity::Phase,

    // Single argument: ID
    args: ArgSpec::Exactly(1),
    // No properties permitted
    props: &[],
    // Only allow one child node: `variables`
    children: &[NodeSpec {
        name: NodeName::Static("variables"),
        identity: SpecIdentity::PhaseVariables,
        args: ArgSpec::None,
        props: &[],
        // Arbitrary children due to names
        children: &[NodeSpec {
            // User defined name
            name: NodeName::Dynamic,
            identity: SpecIdentity::PhaseVariable,
            // Variables have one argument, the value
            args: ArgSpec::Exactly(1),
            props: &[],
            children: &[],
        }],
    }],
};

impl Phase {
    /// Returns the phase ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Consume a processed node to generate a phase
    pub(crate) fn new(node: ProcessedNode<SpecIdentity>) -> Self {
        // > phase > variables > key = value
        let vars = if let Some(block) = node
            .children
            .into_iter()
            .find(|i| i.identity == SpecIdentity::PhaseVariables)
        {
            block
                .children
                .into_iter()
                .filter(|f| f.identity == SpecIdentity::PhaseVariable)
                .map(|f| (f.name, f.args.into_iter().next().unwrap()))
                .collect()
        } else {
            HashMap::new()
        };
        Self {
            id: node.args.into_iter().next().unwrap(),
            _vars: vars,
        }
    }
}
