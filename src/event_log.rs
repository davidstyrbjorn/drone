use std::collections::LinkedList;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub message: String,
    pub color: ColorPair,
}

pub struct EventLog {
    pub messages: LinkedList<LogEntry>,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            messages: LinkedList::new(),
        }
    }
}
