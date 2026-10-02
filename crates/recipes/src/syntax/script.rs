// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Basic "script" functionality
//!
//! unlike traditional shell scripts we want verifiable
//! call chains, custody, hashing, etc, without arbitrary
//! crap polluting the planet.

use kdl::KdlNode;
use tracing::trace;

use crate::syntax::{
    ArgSpec, NodeDescent, NodeName, NodeSpec, TaggedValue, process::process_kdl_node,
};

/// A Script is simply a list of processed execution statements
/// that are in turn flow-guided by incredibly simple boolean
/// conditions (`with`) or list-based iterations (`foreach`)
/// Anything that is NOT a `with` or `foreach` is considered an
/// execution statement, ie invokes a functor
#[derive(Debug)]
pub enum Statement {
    /// A boolean test
    WithCondition {
        reference: String,
        statements: Vec<Statement>,
    },

    ForEachLoop {
        /// Name of the local argument
        arg: String,

        /// Name of the list to loop
        target: String,

        /// Child statements
        statements: Vec<Statement>,
    },

    Execute {
        /// The keyword / function call
        keyword: String,

        /// A set of arguments
        arguments: Vec<TaggedValue>,

        /// A set of properties
        properties: Vec<TaggedValue>,
    },
}

/// A script contains one or more statements,
/// any comment AST processing happened in fancy
/// DSL atop KDL land.
pub struct Script {
    _statements: Vec<Statement>,
}

/// Parsing flow for statements
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum StatementID {
    /// Dynamic, we may be an execute or explicit step
    Root,
    WithCondition,
    ForEachLoop,
    Execute,
}

impl From<StatementID> for usize {
    fn from(val: StatementID) -> Self {
        val as usize
    }
}

// compiled rules to recurse statements and process them
// accordingly.
static RULES: NodeSpec<'static, StatementID> = NodeSpec {
    name: NodeName::Dynamic,
    identity: StatementID::Root,
    args: ArgSpec::Variable, // ignored
    props: &[],              // ignored
    children: &[
        // a with condition
        NodeSpec {
            name: NodeName::Static("with"),
            identity: StatementID::WithCondition,
            // one argument only
            args: ArgSpec::Exactly(1),
            props: &[],    // ignored
            children: &[], // ignored
            descent: NodeDescent::Never,
        },
        // a foreach
        NodeSpec {
            name: NodeName::Static("foreach"),
            identity: StatementID::ForEachLoop,
            args: ArgSpec::Exactly(2),
            props: &[],    // ignored
            children: &[], // ignored
            descent: NodeDescent::Never,
        },
        // an executor
        NodeSpec {
            name: NodeName::Dynamic,
            identity: StatementID::Execute,
            args: ArgSpec::Variable,
            props: &[],    // dynamic
            children: &[], // not allowed
            descent: NodeDescent::Normal,
        },
    ],
    descent: NodeDescent::Normal,
};

impl<'a> Statement {
    /// Generate a Statement from a given node
    pub fn from_kdl_node(node: &'a KdlNode) -> Result<(), super::Error> {
        let mut rules = vec![&RULES];
        let node = process_kdl_node(node, &mut rules)?;
        trace!(id = ?node.identity, "Should be a root id");
        for child in node.children {
            trace!(id = ?&child.identity, "Got a step");
        }
        Ok(())
    }
}
