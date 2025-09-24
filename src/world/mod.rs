//! World module for CLD-OS
//!
//! This module provides the world representation and validation functionality.

use crate::citizens::*;
use crate::parser::Citizen;
use std::collections::HashMap;

/// Represents a complete CLD world with all its citizens
#[derive(Debug)]
pub struct World {
    pub origin: Option<Origin>,
    pub timelines: HashMap<String, Timeline>,
    pub events: HashMap<String, Event>,
    pub core_events: HashMap<String, CoreEvent>,
    pub niches: HashMap<String, Niche>,
    pub eras: HashMap<String, Era>,
    pub generators: HashMap<String, Generator>,
    pub memories: HashMap<String, Memory>,
    pub immunes: HashMap<String, Immune>,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        World {
            origin: None,
            timelines: HashMap::new(),
            events: HashMap::new(),
            core_events: HashMap::new(),
            niches: HashMap::new(),
            eras: HashMap::new(),
            generators: HashMap::new(),
            memories: HashMap::new(),
            immunes: HashMap::new(),
        }
    }

    /// Build a world from a list of citizens
    pub fn from_citizens(citizens: Vec<Citizen>) -> Result<Self, String> {
        let mut world = World::new();
        
        for citizen in citizens {
            match citizen {
                Citizen::Origin(origin) => {
                    if world.origin.is_some() {
                        return Err("Multiple @Origin declarations found".to_string());
                    }
                    world.origin = Some(origin);
                }
                Citizen::Timeline(timeline) => {
                    world.timelines.insert(timeline.name.clone(), timeline);
                }
                Citizen::Event(event) => {
                    world.events.insert(event.name.clone(), event);
                }
                Citizen::CoreEvent(core_event) => {
                    world.core_events.insert(core_event.name.clone(), core_event);
                }
                Citizen::Niche(niche) => {
                    world.niches.insert(niche.name.clone(), niche);
                }
                Citizen::Era(era) => {
                    world.eras.insert(era.name.clone(), era);
                }
                Citizen::Generator(generator) => {
                    world.generators.insert(generator.name.clone(), generator);
                }
                Citizen::Memory(memory) => {
                    world.memories.insert(memory.name.clone(), memory);
                }
                Citizen::Immune(immune) => {
                    world.immunes.insert(immune.name.clone(), immune);
                }
            }
        }
        
        Ok(world)
    }
}

/// Validate a world against CLD v6 rules
pub fn validate_world(world: &World) -> Result<(), String> {
    // Validate !Origin rule - must exist
    validate_origin_exists(world)?;
    
    // Validate !CoreEvent rule - core events in Origin.核心锚点 must be defined
    validate_core_events(world)?;
    
    // TODO: Add more validation rules
    
    Ok(())
}

/// Validate that an Origin exists
fn validate_origin_exists(world: &World) -> Result<(), String> {
    if world.origin.is_none() {
        return Err("!Origin missing: A world must have exactly one @Origin".to_string());
    }
    Ok(())
}

/// Validate that all core events referenced in Origin.核心锚点 are defined
fn validate_core_events(world: &World) -> Result<(), String> {
    let origin = world.origin.as_ref().ok_or("!Origin missing")?;
    
    // Get the 核心锚点 field from Origin
    if let Some(core_anchors_value) = origin.fields.get("核心锚点") {
        if let Some(core_anchors) = core_anchors_value.as_list() {
            // Check that each anchor is defined as a CoreEvent
            for anchor in core_anchors {
                if let Some(identifier) = anchor.as_identifier() {
                    if !world.core_events.contains_key(identifier) {
                        return Err(format!("CoreEvent '{}' referenced in Origin.核心锚点 is not defined", identifier));
                    }
                } else {
                    return Err("Origin.核心锚点 must contain only identifiers".to_string());
                }
            }
        } else {
            return Err("Origin.核心锚点 must be a list".to_string());
        }
    }
    
    Ok(())
}