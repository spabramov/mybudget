use chrono::{DateTime, Local, Utc};

pub enum AppEvent {
    Quit,
    Resize,
    Accept,
    Cancel,
    Up,
    Down,
    Rigth,
    Left,
    KeyEvent(char),
}

#[derive(Debug)]
pub struct Transaction {
    pub timestamp: DateTime<Local>,
    pub amount: f32,
    pub description: String,
}

impl Transaction {
    pub fn new(timestamp: DateTime<Utc>, amount: f32, description: &str) -> Transaction {
        Self {
            timestamp: timestamp.into(),
            amount,
            description: String::from(description),
        }
    }
}
