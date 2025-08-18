#![allow(dead_code)]
use std::collections::{HashMap, hash_map::Keys};

type BoxedCallBack = Box<dyn Fn(HashMap<String, String>)>;
pub struct Parser {
    commands: HashMap<String, BoxedCallBack>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            commands: HashMap::new(),
        }
    }
    pub fn add_command(&mut self, name: String, func: BoxedCallBack) {
        self.commands.insert(name, func);
    }
    pub fn execute_command(&self, name: String, arg: HashMap<String, String>) {
        match self.commands.get(&name) {
            None => panic!("Unkown Command!"),
            Some(callback) => callback(arg),
        }
    }
    pub fn list_command(&self) -> Keys<'_, String, BoxedCallBack> {
        self.commands.keys()
    }
}
