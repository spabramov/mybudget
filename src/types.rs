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
    pub transaction_id: Option<isize>,
    pub credit_acc_id: u8,
    pub debit_acc_id: u8,
    pub timestamp: DateTime<Local>,
    pub category: String,
    pub amount: i64,
    pub description: String,
}
