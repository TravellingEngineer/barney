// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Argument schema parsing

use kdl::KdlNode;

use crate::syntax::{ArgSpec, NodeDescent, NodeName, NodeSpec, process::process_kdl_node};

/// Argument schema parsing, ie dynamic NodeSpecish
/// rules generated from the module action arguments
/// Essentially this is reliant on a single layer of nodes
/// within a root, each node having an explicit annotation type
/// ie as understood within TaggedValue
#[derive(Debug)]
pub struct ArgSchema {}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum ArgSchemaID {
    /// ie "arguments"
    Root,
    //Entry,

    //EntryRule,
}

impl From<ArgSchemaID> for usize {
    fn from(val: ArgSchemaID) -> Self {
        val as usize
    }
}

// Rules for any argument schema block
static RULES: NodeSpec<'static, ArgSchemaID> = NodeSpec {
    name: NodeName::Dynamic,
    identity: ArgSchemaID::Root,
    args: ArgSpec::Variable,
    props: &[],
    children: &[],
    descent: NodeDescent::Never,
};

impl<'a> ArgSchema {
    pub fn from_kdl_node(root: &'a KdlNode) -> Result<Self, super::Error> {
        let mut rules = vec![&RULES];
        let processed = process_kdl_node(root, &mut rules)?;
        assert_eq!(processed.identity, ArgSchemaID::Root);

        Ok(Self {})
    }
}
