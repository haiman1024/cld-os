// 使用 pest 生成解析器
// Cargo.toml: pest = "2.7", pest_derive = "2.7"

use crate::ast::*;

#[derive(Parser)]
#[grammar = "parser/cld.pest"]
pub struct CLDParser;

#[derive(Debug)]
pub enum Declaration {
    Origin(Origin),
    Timeline(Timeline),
    Event(Event),
    CoreEvent(Event), // 复用 Event 结构，标记类型
    Niche(Niche),
    Era(Era),
    Generator(Generator),
    Memory(Memory),
    Immune(Immune),
}

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub description: String,
    pub emotional_peak: u8, // 0-100
    pub entropy_delta: i32,
    pub effects: std::collections::HashMap<String, serde_json::Value>,
    pub is_core: bool,
}

// 解析入口
pub fn parse_cld(input: &str) -> Result<Vec<Declaration>, pest::error::Error<Rule>> {
    let pairs = CLDParser::parse(Rule::cld_document, input)?;
    let mut decls = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::origin_decl => decls.push(Declaration::Origin(parse_origin(pair)?)),
            Rule::core_event_decl => {
                let mut event = parse_event(pair)?;
                event.is_core = true;
                decls.push(Declaration::CoreEvent(event));
            }
            Rule::event_decl => decls.push(Declaration::Event(parse_event(pair)?)),
            // ... 其他解析函数
            _ => {}
        }
    }
    Ok(decls)
}

// 解析 Origin 声明
fn parse_origin(pair: pest::iterators::Pair<Rule>) -> Result<Origin, pest::error::Error<Rule>> {
    // TODO: 实现完整的 Origin 解析逻辑
    unimplemented!()
}

// 解析 Event 声明
fn parse_event(pair: pest::iterators::Pair<Rule>) -> Result<Event, pest::error::Error<Rule>> {
    // TODO: 实现完整的 Event 解析逻辑
    unimplemented!()
}
