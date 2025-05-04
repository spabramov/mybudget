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
    pub account_id: usize,
    pub timestamp: DateTime<Local>,
    pub category: String,
    pub amount: f32,
    pub description: String,
}

impl Transaction {
    pub fn new(
        account_id: usize,
        timestamp: DateTime<Utc>,
        amount: f32,
        category: &str,
        description: &str,
    ) -> Transaction {
        Self {
            account_id,
            timestamp: timestamp.into(),
            amount,
            category: String::from(category),
            description: String::from(description),
        }
    }
}
