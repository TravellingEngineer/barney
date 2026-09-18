// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap phase

use crate::{
    bootstrap::SpecIdentity,
    syntax::{ArgSpec, NodeName, NodeSpec},
};

#[derive(Debug)]
pub struct Phase {
    id: String,
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
}
