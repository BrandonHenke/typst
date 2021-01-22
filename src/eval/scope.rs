use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::iter;

use super::Value;

/// A hierarchy of scopes.
#[derive(Debug, Clone, PartialEq)]
pub struct Scopes<'a> {
    /// The active scope.
    top: Scope,
    /// The stack of lower scopes.
    scopes: Vec<Scope>,
    /// The base scope.
    base: &'a Scope,
}

impl<'a> Scopes<'a> {
    /// Create a new hierarchy of scopes.
    pub fn new(base: &'a Scope) -> Self {
        Self { top: Scope::new(), scopes: vec![], base }
    }

    /// Define a variable in the active scope.
    pub fn define(&mut self, var: impl Into<String>, value: impl Into<Value>) {
        self.top.define(var, value);
    }

    /// Look up the value of a variable.
    pub fn get(&self, var: &str) -> Option<&Value> {
        iter::once(&self.top)
            .chain(&self.scopes)
            .chain(iter::once(self.base))
            .find_map(|scope| scope.get(var))
    }

    /// Get a mutable reference to a variable.
    pub fn get_mut(&mut self, var: &str) -> Option<&mut Value> {
        iter::once(&mut self.top)
            .chain(&mut self.scopes)
            .find_map(|scope| scope.get_mut(var))
    }

    /// Return whether the variable is constant (not writable).
    ///
    /// Defaults to `false` if the variable does not exist.
    pub fn is_const(&self, var: &str) -> bool {
        self.base.get(var).is_some()
    }
}

/// A map from variable names to values.
#[derive(Default, Clone, PartialEq)]
pub struct Scope {
    values: HashMap<String, Value>,
}

impl Scope {
    /// Create a new empty scope.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a new variable.
    pub fn define(&mut self, var: impl Into<String>, value: impl Into<Value>) {
        self.values.insert(var.into(), value.into());
    }

    /// Look up the value of a variable.
    pub fn get(&self, var: &str) -> Option<&Value> {
        self.values.get(var)
    }

    /// Get a mutable reference to a variable.
    pub fn get_mut(&mut self, var: &str) -> Option<&mut Value> {
        self.values.get_mut(var)
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.values.fmt(f)
    }
}
