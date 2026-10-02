// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Basic "script" functionality
//!
//! unlike traditional shell scripts we want verifiable
//! call chains, custody, hashing, etc, without arbitrary
//! crap polluting the planet.

use crate::syntax::TaggedValue;

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
