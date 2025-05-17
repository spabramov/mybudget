use chrono::{DateTime, Local};

pub enum AppEvent {
    Quit,
    Resize,
    Accept,
    Cancel,
    Up,
    Down,
    Rigth,
    Left,
    Key(char),
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub transaction_id: usize,
    pub credit_acc_id: u16,
    pub debit_acc_id: u16,
    pub timestamp: DateTime<Local>,
    pub category: String,
    pub amount: usize,
    pub description: String,
}
