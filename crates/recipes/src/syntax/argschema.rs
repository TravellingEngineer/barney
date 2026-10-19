// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Argument schema parsing

use std::collections::HashMap;

use kdl::KdlNode;

use crate::syntax::{ArgSpec, NodeDescent, NodeName, NodeSpec, process::process_kdl_node};

/// Argument schema parsing, ie dynamic NodeSpecish
/// rules generated from the module action arguments
/// Essentially this is reliant on a single layer of nodes
/// within a root, each node having an explicit annotation type
/// ie as understood within TaggedValue
#[derive(Debug)]
pub struct ArgSchema<'a> {
    ruleset: HashMap<DynamicRuleID, NodeSpec<'a, DynamicRuleID>>,
    _last_id: usize,
}

/// newtyping for NodeSchema rules to permit dynamic evaluation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct DynamicRuleID(usize);

impl From<DynamicRuleID> for usize {
    fn from(val: DynamicRuleID) -> Self {
        val.0
    }
}

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

impl<'a> ArgSchema<'a> {
    pub fn from_kdl_node(root: &KdlNode) -> Result<Self, super::Error> {
        let mut rules = vec![&RULES];
        let processed = process_kdl_node(root, &mut rules)?;
        assert_eq!(processed.identity, ArgSchemaID::Root);

        let ruleset = HashMap::new();

        Ok(Self {
            ruleset,
            _last_id: 0,
        })
    }

    /// Yield as a set of "static" rule types to integrate with the existing rule loops
    /// explicitly only permitted for executor calls inside script blocks
    pub fn rules(&self) -> impl Iterator<Item = &NodeSpec<'a, DynamicRuleID>> {
        self.ruleset.values()
    }
}
