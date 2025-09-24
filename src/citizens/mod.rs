//! Core citizens of the CLD v6 system
//!
//! This module defines the 9 core citizens that form the minimal complete set
//! for any living world in the CLD system.

/// The world's initial singularity (time origin, entropy, core contradiction)
#[derive(Debug, Clone)]
pub struct Origin {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// The time manifold (directed causal graph, supports main axis/branch axes)
#[derive(Debug, Clone)]
pub struct Timeline {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// The minimal causal change unit (includes emotional peaks, world entropy change)
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// Immutable historical cornerstone
#[derive(Debug, Clone)]
pub struct CoreEvent {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// Behavioral niche (strategy, stress response)
#[derive(Debug, Clone)]
pub struct Niche {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// Historical semantic layer (rules and capabilities of time periods)
#[derive(Debug, Clone)]
pub struct Era {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// Dynamic content generator (pure function)
#[derive(Debug, Clone)]
pub struct Generator {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// Collective memory bank (layered storage, compression)
#[derive(Debug, Clone)]
pub struct Memory {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// World immune system (monitoring, prediction, repair)
#[derive(Debug, Clone)]
pub struct Immune {
    pub name: String,
    pub fields: std::collections::HashMap<String, String>,
}