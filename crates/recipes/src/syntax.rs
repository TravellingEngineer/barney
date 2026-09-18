// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Syntax helpers / errors

use std::{collections::HashMap, fmt::Debug, hash::Hash, vec};

use itertools::{Either, Itertools};
use kdl::{KdlDocument, KdlNode};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// A "Baked" node when processed via AST
#[derive(Debug)]
pub struct ProcessedNode<I>
where
    I: Into<usize> + Eq + PartialEq + PartialOrd + Hash + Debug + Clone,
{
    pub identity: I,
    pub name: String,
    // TODO: Use type system with tagging + variable references
    pub args: Vec<String>,
    pub props: HashMap<String, String>,
    pub children: Vec<ProcessedNode<I>>,
}

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
    I: Into<usize> + Eq + PartialEq + PartialOrd + Hash + Debug + Clone,
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

    #[error("unknown property")]
    UnknownProperty {
        #[label("No such property '{name}' exists")]
        span: SourceSpan,

        name: String,
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
        #[label("Encountered an unexpected identifier: {id}")]
        span: SourceSpan,

        id: String,
    },
}

/// Process KDL according to the given rule set
pub(super) fn process_kdl<'a, I>(
    document: &KdlDocument,
    rules: &[&NodeSpec<'a, I>],
) -> Result<Vec<ProcessedNode<I>>, Error>
where
    I: Into<usize> + Eq + PartialEq + PartialOrd + Hash + Debug + Clone,
{
    let mut ruleset = rules.iter().cloned().collect_vec();
    let mut results = vec![];

    // Toplevel descent + entry
    for node in document.nodes() {
        // Recurse children not siblings
        results.push(process_kdl_node(node, &mut ruleset)?);
    }

    Ok(results)
}

/// Process a single KDL node according to rules and if successful, return built
/// nodes according to our DSL requirements
/// If a rule is "spent" in the current context, remove it from the input rules
fn process_kdl_node<'a, 'b, I>(
    node: &KdlNode,
    rules: &'b mut Vec<&NodeSpec<'a, I>>,
) -> Result<ProcessedNode<I>, Error>
where
    I: Into<usize> + Eq + PartialEq + PartialOrd + Hash + Debug + Clone,
{
    let mut rule_index = None;
    for (index, rule) in rules.iter().enumerate() {
        match rule.name {
            // Require explicit name match
            NodeName::Static(n) => {
                if n == node.name().value() {
                    rule_index = Some(index);
                    break;
                }
            }
            // Dynamic means it must match now
            NodeName::Dynamic => {
                rule_index = Some(index);
                break;
            }
        }
    }

    // Is this a legal identifier per nodespec?
    let idx = rule_index.ok_or_else(|| Error::UnexpectedIdentifier {
        span: node.span(),
        id: node.name().to_string(),
    })?;
    let rule = rules.get(idx).unwrap();

    // bake a property map and argument set
    let (properties, args): (HashMap<String, _>, Vec<_>) =
        node.entries().iter().partition_map(|n| {
            if let Some(name) = n.name() {
                Either::Left((name.to_string(), n))
            } else {
                Either::Right(n)
            }
        });

    // Ensure argument policy is sane
    match rule.args {
        // No arguments allowed?
        ArgSpec::None => {
            if !args.is_empty() {
                return Err(Error::WrongArgumentCount {
                    span: node.span(),
                    expected: 0,
                    found: args.len(),
                });
            }
        }
        // Only N args
        ArgSpec::Exactly(n) => {
            if n != args.len() {
                return Err(Error::WrongArgumentCount {
                    span: node.span(),
                    expected: n,
                    found: args.len(),
                });
            }
        }
        ArgSpec::Variable => {}
    }

    // check property sanity
    // TODO: Permit dynamic property matching like for arguments
    // TODO: Handle duplicate rules
    for (id, prop) in properties.iter() {
        let _ = rule
            .props
            .iter()
            .find_position(|r| r.name == id)
            .ok_or_else(|| Error::UnknownProperty {
                span: prop.span(),
                name: id.clone(),
            })?;
    }

    let mut children = vec![];

    // Recurse the child with subset of expendable rules
    let mut child_rules = rule.children.iter().collect_vec();
    for child in node.iter_children() {
        children.push(process_kdl_node(child, &mut child_rules)?);
    }

    // Full baked node

    Ok(ProcessedNode {
        identity: rule.identity.clone(),
        name: node.name().to_string(),
        args: args.into_iter().map(|a| a.value().to_string()).collect(),
        props: properties
            .into_iter()
            .map(|(k, v)| (k, v.value().to_string()))
            .collect(),
        children,
    })
}
