//! cld-os: A universal operating system for living, breathing narrative worlds
//! 
//! This crate provides the core functionality for creating and managing
//! "living worlds" that can evolve, remember, and self-heal.

/// The main module for parsing CLD files
pub mod parser;

/// The core citizen definitions
pub mod citizens;

/// The world state management
pub mod world;

/// The timeline and event management
pub mod timeline;

/// The memory system for storing world history
pub mod memory;

/// The immune system for world self-healing
pub mod immune;

/// The generator system for dynamic content creation
pub mod generator;