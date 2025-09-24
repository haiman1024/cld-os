//! CLD file parser module
//!
//! This module handles parsing of .cld files using pest grammar.

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar/cld.pest"]
pub struct CLDParser;

use crate::citizens::*;
use pest::Parser as PestParser;

impl CLDParser {
    /// Parse a CLD file content into citizen objects
    pub fn parse_cld(content: &str) -> Result<Vec<Citizen>, Box<dyn std::error::Error>> {
        let pairs = CLDParser::parse(Rule::cld_file, content)?;
        let mut citizens = Vec::new();

        for pair in pairs {
            // Handle the top-level cld_file rule
            if pair.as_rule() == Rule::cld_file {
                // Iterate through the children of cld_file
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::origin => {
                            citizens.push(Citizen::Origin(parse_origin(inner_pair)?));
                        }
                        Rule::timeline => {
                            citizens.push(Citizen::Timeline(parse_timeline(inner_pair)?));
                        }
                        Rule::event => {
                            citizens.push(Citizen::Event(parse_event(inner_pair)?));
                        }
                        Rule::core_event => {
                            citizens.push(Citizen::CoreEvent(parse_core_event(inner_pair)?));
                        }
                        Rule::niche => {
                            citizens.push(Citizen::Niche(parse_niche(inner_pair)?));
                        }
                        Rule::era => {
                            citizens.push(Citizen::Era(parse_era(inner_pair)?));
                        }
                        Rule::generator => {
                            citizens.push(Citizen::Generator(parse_generator(inner_pair)?));
                        }
                        Rule::memory => {
                            citizens.push(Citizen::Memory(parse_memory(inner_pair)?));
                        }
                        Rule::immune => {
                            citizens.push(Citizen::Immune(parse_immune(inner_pair)?));
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(citizens)
    }
}

#[derive(Debug)]
pub enum Citizen {
    Origin(Origin),
    Timeline(Timeline),
    Event(Event),
    CoreEvent(CoreEvent),
    Niche(Niche),
    Era(Era),
    Generator(Generator),
    Memory(Memory),
    Immune(Immune),
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::string => {
            // Remove quotes from string value
            let str_content = pair.as_str();
            if str_content.starts_with("\"\"\"") && str_content.ends_with("\"\"\"") {
                // Handle triple-quoted strings
                Ok(Value::String(str_content[3..str_content.len()-3].to_string()))
            } else if str_content.starts_with("\"") && str_content.ends_with("\"") {
                // Handle regular quoted strings
                Ok(Value::String(str_content[1..str_content.len()-1].to_string()))
            } else {
                Ok(Value::String(str_content.to_string()))
            }
        }
        Rule::number => {
            let num_str = pair.as_str();
            let number = num_str.parse::<f64>()?;
            Ok(Value::Number(number))
        }
        Rule::boolean => {
            let bool_str = pair.as_str();
            match bool_str {
                "true" => Ok(Value::Boolean(true)),
                "false" => Ok(Value::Boolean(false)),
                _ => Err(format!("Invalid boolean value: {}", bool_str).into()),
            }
        }
        Rule::list => {
            let mut values = Vec::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::value {
                    values.push(parse_value(inner_pair)?);
                }
            }
            Ok(Value::List(values))
        }
        Rule::identifier => {
            Ok(Value::Identifier(pair.as_str().to_string()))
        }
        _ => Err(format!("Unexpected value rule: {:?}, text: {:?}", pair.as_rule(), pair.as_str()).into()),
    }
}

fn parse_origin(pair: pest::iterators::Pair<Rule>) -> Result<Origin, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing origin name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::origin_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Origin { name, fields })
}

fn parse_timeline(pair: pest::iterators::Pair<Rule>) -> Result<Timeline, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing timeline name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::timeline_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Timeline { name, fields })
}

fn parse_event(pair: pest::iterators::Pair<Rule>) -> Result<Event, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing event name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::event_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Event { name, fields })
}

fn parse_core_event(pair: pest::iterators::Pair<Rule>) -> Result<CoreEvent, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing core event name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::event_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(CoreEvent { name, fields })
}

fn parse_niche(pair: pest::iterators::Pair<Rule>) -> Result<Niche, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing niche name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::niche_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Niche { name, fields })
}

fn parse_era(pair: pest::iterators::Pair<Rule>) -> Result<Era, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing era name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::era_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Era { name, fields })
}

fn parse_generator(pair: pest::iterators::Pair<Rule>) -> Result<Generator, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing generator name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::generator_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Generator { name, fields })
}

fn parse_memory(pair: pest::iterators::Pair<Rule>) -> Result<Memory, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing memory name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::memory_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Memory { name, fields })
}

fn parse_immune(pair: pest::iterators::Pair<Rule>) -> Result<Immune, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing immune name")?.as_str().to_string();
    let mut fields = std::collections::HashMap::new();

    for field in inner {
        if field.as_rule() == Rule::immune_field {
            let mut field_inner = field.into_inner();
            let key = field_inner.next().ok_or("Missing field key")?.as_str().to_string();
            let value_pair = field_inner.next().ok_or("Missing field value")?;
            let value = parse_value(value_pair)?;
            fields.insert(key, value);
        }
    }

    Ok(Immune { name, fields })
}