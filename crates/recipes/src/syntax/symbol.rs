// SPDX-FileCopyrightText: 2026 Ikey Doherty
// SPDX-License-Identifier: MPL-2.0

//! Creates strong symbols based on namespace and ID

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("namespace contains special prefix chars")]
    NamespaceContainsPrefixChars,

    #[error("id contains special prefix chars")]
    IdContainsPrefixChars,
}

/// A Symbol is essentially an encoding of symbol and namespace
/// ie `core::emit` or `autotools::configure`
/// As a special case the `core` domain is "always visible" and any
/// unqualified functor is assumed to be in `core` to emulate an import
/// pattern without the problems associated with them (shadowing etc)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Symbol {
    // The namespace the symbol belongs to, i.e. `autotools`
    namespace: String,

    // The specific symbol ID, ie `configure`
    id: String,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{}", &self.namespace, &self.id))
    }
}

impl Symbol {
    /// Create a new symbol
    pub fn new(namespace: &str, id: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            id: id.to_string(),
        }
    }

    /// Return reference to the namespace
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return a reference to the ID
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// A SymbolTable is basically an ordered map of symbols
/// that can be enumerated by prefix scans
#[derive(Default, Debug)]
pub struct SymbolTable {
    mapping: BTreeMap<String, Symbol>,

    // lazy cache for all known namespaces / prefixes
    namespaces: BTreeSet<String>,
}

impl<'a> SymbolTable {
    /// Construct a new SymbolTable
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert and take ownership of a symbol
    pub fn insert(&mut self, symbol: Symbol) -> Result<(), Error> {
        if symbol.namespace().contains("::") {
            Err(Error::NamespaceContainsPrefixChars)
        } else if symbol.id().contains("::") {
            Err(Error::IdContainsPrefixChars)
        } else {
            let fqdn = symbol.to_string();
            self.namespaces.insert(symbol.namespace.to_owned());
            self.mapping.insert(fqdn, symbol);
            Ok(())
        }
    }

    /// Allow symbol enumeration by way of a fixed prefix
    pub fn namespace_symbols(&'a self, namespace: &str) -> impl Iterator<Item = &'a Symbol> {
        let prefixed = format!("{}::", namespace);
        let namespace_cz = namespace.to_string();
        self.mapping
            .range(prefixed..)
            .take_while(move |(_, s)| s.namespace == namespace_cz)
            .map(|(_, s)| s)
    }

    /// Read only references to namespaces
    pub fn namespaces(&'a self) -> impl Iterator<Item = &'a str> {
        self.namespaces.iter().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_prefixes() {
        let mut tab = SymbolTable::new();
        tab.insert(Symbol {
            namespace: "autotools".to_string(),
            id: "configure".to_string(),
        })
        .expect("good symbol should work");
        tab.insert(Symbol {
            namespace: "autotools".to_string(),
            id: "make".to_string(),
        })
        .expect("good symbol should work");
        tab.insert(Symbol {
            namespace: "core".to_string(),
            id: "emit".to_string(),
        })
        .expect("good symbol should work");
        assert_eq!(tab.namespace_symbols("").collect_vec().len(), 0);
        assert_eq!(tab.namespace_symbols("autotools").collect_vec().len(), 2);
        assert_eq!(tab.namespace_symbols("core").collect_vec().len(), 1);
        assert_eq!(tab.namespaces.iter().collect_vec().len(), 2);
    }
}
