#![allow(unused)]
use crate::core::command::Parser;
use std::collections::HashMap;

fn hello(args: HashMap<String, String>) -> String{
    format!("Hello {}", args.get("name").unwrap())
}

#[test]
fn test() {
    let mut p = Parser::new();
    let mut arg = HashMap::new();
    arg.insert("name".to_string(), "world".to_string());
    p.add_command("hello".to_string(), Box::new(hello));
    p.execute_command("hello".to_string(), arg);
}
