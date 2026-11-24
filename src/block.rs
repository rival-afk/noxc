use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::calc_hash;

pub struct Block {
    pub index: u64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub validator: String,
    pub timestamp: u64
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String, validator: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let hash = calc_hash(index, &transactions, &prev_hash, &validator, timestamp);
        Self {
            index,
            transactions,
            hash,
            prev_hash,
            validator,
            timestamp
        }
    }  
}
