// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Bootstrap configuration `module`

use std::collections::HashMap;

use itertools::Itertools;
use tracing::info;

use crate::{
    bootstrap::SpecIdentity,
    syntax::{ArgSpec, NodeDescent, NodeName, NodeSpec, ProcessedNode, TaggedValue},
};

/// A module within the bootstrap configuration
///
/// It can contain its own variables, public exports,
/// and also actions.
#[derive(Debug)]
pub struct Module {
    id: String,
    // TODO: Disallow any tags but var/arg
    vars: HashMap<String, TaggedValue>,
    exports: HashMap<String, TaggedValue>,
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
                descent: NodeDescent::Normal,
            }],
            descent: NodeDescent::Normal,
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
                descent: NodeDescent::Normal,
            }],
            descent: NodeDescent::Normal,
        },
        NodeSpec {
            name: NodeName::Static("action"),
            identity: SpecIdentity::ModuleAction,
            args: ArgSpec::Exactly(1),
            props: &[],
            children: &[
                NodeSpec {
                    name: NodeName::Static("arguments"),
                    identity: SpecIdentity::ModuleActionArguments,
                    args: ArgSpec::None,
                    props: &[],
                    children: &[],
                    descent: NodeDescent::Never,
                },
                NodeSpec {
                    name: NodeName::Static("execute"),
                    identity: SpecIdentity::ModuleActionExecute,
                    args: ArgSpec::None,
                    props: &[],
                    children: &[],
                    descent: NodeDescent::Never,
                },
            ],
            descent: NodeDescent::Normal,
        },
    ],
    descent: NodeDescent::Normal,
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
                    .map(|n| (n.name().to_owned(), n.args.into_iter().next().unwrap()))
                    .for_each(|(k, v)| {
                        vars.insert(k, v);
                    }),
                SpecIdentity::ModuleExports => child
                    .children
                    .into_iter()
                    .filter(|c| c.identity == SpecIdentity::ModuleExport)
                    .map(|n| (n.name().to_owned(), n.args.into_iter().next().unwrap()))
                    .for_each(|(k, v)| {
                        exports.insert(k, v);
                    }),
                SpecIdentity::ModuleAction => {
                    info!("Got an action: {:?}", child.args.first())
                }
                _ => panic!("derp"),
            }
        }

        Self {
            id: id.to_string(),
            exports,
            vars,
        }
    }

    /// Return the module ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return all export keys
    pub fn exports(&self) -> Vec<String> {
        self.exports.keys().cloned().collect_vec()
    }

    /// Return all variable keys
    pub fn vars(&self) -> Vec<String> {
        self.vars.keys().cloned().collect_vec()
    }
}
