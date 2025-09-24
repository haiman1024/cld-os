//! Main entry point for the cld-os CLI

use std::env;
use std::fs;
use cld_os::parser::CLDParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <cld-file>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("Parsing CLD file: {}", file_path);
            parse_cld_content(&content);
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}

fn parse_cld_content(content: &str) {
    println!("Content length: {} characters", content.len());
    
    match CLDParser::parse_cld(content) {
        Ok(citizens) => {
            println!("Successfully parsed {} citizens", citizens.len());
            for citizen in citizens {
                match citizen {
                    cld_os::parser::Citizen::Origin(origin) => {
                        println!("Origin: {} with {} fields", origin.name, origin.fields.len());
                    },
                    cld_os::parser::Citizen::Timeline(timeline) => {
                        println!("Timeline: {} with {} fields", timeline.name, timeline.fields.len());
                    },
                    cld_os::parser::Citizen::Event(event) => {
                        println!("Event: {} with {} fields", event.name, event.fields.len());
                    },
                    cld_os::parser::Citizen::CoreEvent(core_event) => {
                        println!("CoreEvent: {} with {} fields", core_event.name, core_event.fields.len());
                    },
                    cld_os::parser::Citizen::Niche(niche) => {
                        println!("Niche: {} with {} fields", niche.name, niche.fields.len());
                    },
                    cld_os::parser::Citizen::Era(era) => {
                        println!("Era: {} with {} fields", era.name, era.fields.len());
                    },
                    cld_os::parser::Citizen::Generator(generator) => {
                        println!("Generator: {} with {} fields", generator.name, generator.fields.len());
                    },
                    cld_os::parser::Citizen::Memory(memory) => {
                        println!("Memory: {} with {} fields", memory.name, memory.fields.len());
                    },
                    cld_os::parser::Citizen::Immune(immune) => {
                        println!("Immune: {} with {} fields", immune.name, immune.fields.len());
                    },
                }
            }
        },
        Err(e) => {
            eprintln!("Error parsing CLD content: {}", e);
            std::process::exit(1);
        }
    }
}