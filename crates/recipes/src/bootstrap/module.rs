// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap configuration `module`

use std::collections::HashMap;

use crate::{
    bootstrap::SpecIdentity,
    syntax::{ArgSpec, NodeName, NodeSpec, ProcessedNode},
};

/// A module within the bootstrap configuration
///
/// It can contain its own variables, public exports,
/// and also actions.
#[derive(Debug)]
pub struct Module {
    id: String,
    vars: HashMap<String, String>,
    exports: HashMap<String, String>,
}

/// Rules for the module nodespec
pub(super) static RULES: NodeSpec<'static, SpecIdentity> = NodeSpec {
    name: NodeName::Static("module"),
    identity: SpecIdentity::Module,
    args: ArgSpec::Exactly(1),
    props: &[],
    children: &[
        // variables
        NodeSpec {
            name: NodeName::Static("variables"),
            identity: SpecIdentity::ModuleVariables,
            args: ArgSpec::None,
            props: &[],
            children: &[NodeSpec {
                name: NodeName::Dynamic,
                identity: SpecIdentity::ModuleVariable,
                args: ArgSpec::Exactly(1),
                props: &[],
                children: &[],
            }],
        },
        // exports
        NodeSpec {
            name: NodeName::Static("exports"),
            identity: SpecIdentity::ModuleExports,
            args: ArgSpec::None,
            props: &[],
            children: &[NodeSpec {
                name: NodeName::Dynamic,
                identity: SpecIdentity::ModuleExport,
                args: ArgSpec::Exactly(1),
                props: &[],
                children: &[],
            }],
        },
    ],
};

impl Module {
    pub(crate) fn new(node: ProcessedNode<SpecIdentity>) -> Self {
        // Pull the ID out
        let id = node.args.into_iter().next().unwrap();
        let mut exports = HashMap::new();
        let mut vars = HashMap::new();

        for child in node.children {
            match child.identity {
                SpecIdentity::ModuleVariables => child
                    .children
                    .into_iter()
                    .filter(|c| c.identity == SpecIdentity::ModuleVariable)
                    .map(|n| (n.name, n.args.into_iter().next().unwrap()))
                    .for_each(|(k, v)| {
                        vars.insert(k, v);
                    }),
                SpecIdentity::ModuleExports => child
                    .children
                    .into_iter()
                    .filter(|c| c.identity == SpecIdentity::ModuleExport)
                    .map(|n| (n.name, n.args.into_iter().next().unwrap()))
                    .for_each(|(k, v)| {
                        exports.insert(k, v);
                    }),
                _ => panic!("derp"),
            }
        }

        Self { id, exports, vars }
    }

    /// Return the module ID
    pub fn id(&self) -> &str {
        &self.id
    }
}
