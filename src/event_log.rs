use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub message: String,
    pub color: ColorPair,
}

pub struct EventLog {
    pub messages: Vec<LogEntry>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
}
