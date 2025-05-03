pub enum AppEvent {
    Quit,
    Resize,
    Accept,
    Cancel,
    KeyEvent(char),
}

#[derive(Debug)]
pub struct Transaction {
    pub timestamp: String,
    pub amount: f32,
    pub description: String,
}

impl Transaction {
    pub fn new(timestamp: &str, amount: f32, description: &str) -> Transaction {
        Self {
            timestamp: String::from(timestamp),
            amount,
            description: String::from(description),
        }
    }
}
