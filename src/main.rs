//! Main entry point for the cld-os CLI

use std::env;
use std::fs;
use cld_os::parser::CLDParser;
use cld_os::world::{World, validate_world};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        eprintln!("Commands:");
        eprintln!("  parse <cld-file>     Parse a CLD file and output JSON AST");
        eprintln!("  validate <cld-file>  Validate a CLD file against CLD v6 rules");
        std::process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Usage: {} parse <cld-file>", args[0]);
                std::process::exit(1);
            }
            
            let file_path = &args[2];
            parse_cld_file(file_path);
        }
        "validate" => {
            if args.len() < 3 {
                eprintln!("Usage: {} validate <cld-file>", args[0]);
                std::process::exit(1);
            }
            
            let file_path = &args[2];
            validate_cld_file(file_path);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: {} <command> [args...]", args[0]);
            eprintln!("Commands:");
            eprintln!("  parse <cld-file>     Parse a CLD file and output JSON AST");
            eprintln!("  validate <cld-file>  Validate a CLD file against CLD v6 rules");
            std::process::exit(1);
        }
    }
}

fn parse_cld_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match CLDParser::parse_cld(&content) {
                Ok(citizens) => {
                    // Print citizen count for each type
                    let mut origin_count = 0;
                    let mut timeline_count = 0;
                    let mut event_count = 0;
                    let mut core_event_count = 0;
                    let mut niche_count = 0;
                    let mut era_count = 0;
                    let mut generator_count = 0;
                    let mut memory_count = 0;
                    let mut immune_count = 0;
                    
                    for citizen in &citizens {
                        match citizen {
                            cld_os::parser::Citizen::Origin(_) => origin_count += 1,
                            cld_os::parser::Citizen::Timeline(_) => timeline_count += 1,
                            cld_os::parser::Citizen::Event(_) => event_count += 1,
                            cld_os::parser::Citizen::CoreEvent(_) => core_event_count += 1,
                            cld_os::parser::Citizen::Niche(_) => niche_count += 1,
                            cld_os::parser::Citizen::Era(_) => era_count += 1,
                            cld_os::parser::Citizen::Generator(_) => generator_count += 1,
                            cld_os::parser::Citizen::Memory(_) => memory_count += 1,
                            cld_os::parser::Citizen::Immune(_) => immune_count += 1,
                        }
                    }
                    
                    println!("Parsed CLD file: {}", file_path);
                    println!("Citizens parsed:");
                    println!("  Origin: {}", origin_count);
                    println!("  Timeline: {}", timeline_count);
                    println!("  Event: {}", event_count);
                    println!("  CoreEvent: {}", core_event_count);
                    println!("  Niche: {}", niche_count);
                    println!("  Era: {}", era_count);
                    println!("  Generator: {}", generator_count);
                    println!("  Memory: {}", memory_count);
                    println!("  Immune: {}", immune_count);
                    
                    // Try to build world and validate
                    match World::from_citizens(citizens) {
                        Ok(world) => {
                            println!("\nWorld built successfully!");
                            if let Some(origin) = &world.origin {
                                println!("Origin: {} with {} fields", origin.name, origin.fields.len());
                            }
                        }
                        Err(e) => {
                            eprintln!("Error building world: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing CLD content: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}

fn validate_cld_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match CLDParser::parse_cld(&content) {
                Ok(citizens) => {
                    match World::from_citizens(citizens) {
                        Ok(world) => {
                            match validate_world(&world) {
                                Ok(()) => {
                                    println!("Validation successful: {} is a valid CLD v6 file", file_path);
                                }
                                Err(e) => {
                                    eprintln!("Validation failed: {}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error building world: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing CLD content: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}