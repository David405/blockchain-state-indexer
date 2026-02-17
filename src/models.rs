use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },
}