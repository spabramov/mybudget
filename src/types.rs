use chrono::{DateTime, Local};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ScreenMode {
    #[default]
    Browsing,
    Editing,
}

#[derive(Debug)]
pub enum AppEvent {
    Notifiction(String),
}

#[derive(Debug)]
pub enum NavEvent {
    Left,
    Rigth,
    Up,
    Down,
    Cancel,
    Interact,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub transaction_id: Option<isize>,
    pub credit_acc_id: Option<u8>,
    pub debit_acc_id: Option<u8>,
    pub timestamp: DateTime<Local>,
    pub category: Option<String>,
    pub amount: i64,
    pub description: Option<String>,
}

// impl Transaction {
//     pub fn new(timestamp: DateTime<Local>, amount: i64) -> Self {
//         Self {
//             transaction_id: None,
//             credit_acc_id: None,
//             debit_acc_id: None,
//             timestamp,
//             category: None,
//             amount,
//             description: None,
//         }
//     }
// }
