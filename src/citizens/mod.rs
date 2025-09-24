//! Core citizens of the CLD v6 system
//!
//! This module defines the 9 core citizens that form the minimal complete set
//! for any living world in the CLD system.

/// Value enum to represent different types of values in CLD files
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Value>),
    Identifier(String),
}

impl Value {
    /// Get the string representation of the value
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get the number representation of the value
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get the boolean representation of the value
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get the list representation of the value
    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    /// Get the identifier representation of the value
    pub fn as_identifier(&self) -> Option<&String> {
        match self {
            Value::Identifier(i) => Some(i),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Identifier(i) => write!(f, "{}", i),
        }
    }
}

/// The world's initial singularity (time origin, entropy, core contradiction)
#[derive(Debug, Clone)]
pub struct Origin {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// The time manifold (directed causal graph, supports main axis/branch axes)
#[derive(Debug, Clone)]
pub struct Timeline {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// The minimal causal change unit (includes emotional peaks, world entropy change)
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// Immutable historical cornerstone
#[derive(Debug, Clone)]
pub struct CoreEvent {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// Behavioral niche (strategy, stress response)
#[derive(Debug, Clone)]
pub struct Niche {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// Historical semantic layer (rules and capabilities of time periods)
#[derive(Debug, Clone)]
pub struct Era {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// Dynamic content generator (pure function)
#[derive(Debug, Clone)]
pub struct Generator {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// Collective memory bank (layered storage, compression)
#[derive(Debug, Clone)]
pub struct Memory {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

/// World immune system (monitoring, prediction, repair)
#[derive(Debug, Clone)]
pub struct Immune {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}