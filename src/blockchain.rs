use crate::block::Block;
use crate::transaction::Transaction;
use std::collections::{HashSet};
use crate::calc_hash;

pub struct BC {
    pub chain: Vec<Block>,
    pub authorities: Vec<String>,
    currencies: HashSet<String>
}

impl BC {
    pub fn init() -> Self {
        let init_block = Block::new(0, vec![Transaction { sender: String::from("system"), receiver: String::from("test1"), amount: 1000, currency: String::from("NXC") }], String::from("0"), String::from("init"));
        Self {
            chain: vec![init_block],
            authorities: vec![String::from("server"), String::from("init")],
            currencies: HashSet::from([String::from("NXC")])
        }
    }

    pub fn add(&mut self, tns: Vec<Transaction>, validator: &str) {
        let prev_block = self.chain.last().unwrap();
        let index = prev_block.index + 1;
        let prev_hash = prev_block.hash.clone();
        for trn in &tns {
            let balance = self.get_balance(&trn.sender, &trn.currency).unwrap_or(0);
            if balance < trn.amount {
                println!("[ERR] {} has insufficient funds ({} {}) to complete the transfer: {} {}", trn.sender, balance, trn.currency, trn.amount, trn.currency);
                return
            }
        }
        let block = Block::new(index, tns, prev_hash, String::from(validator));
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> (bool, i64) {
        let genesis = &self.chain[0];
        if calc_hash(genesis.index, &genesis.transactions, &genesis.prev_hash, &genesis.validator, genesis.timestamp) != genesis.hash {
            return (false, 0)
        }
        if !self.authorities.contains(&genesis.validator) {
            return (false, 0)
        }
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let current = &self.chain[i];
            if calc_hash(current.index, &current.transactions, &current.prev_hash, &current.validator,  current.timestamp) != current.hash {
                return (false, i as i64)
            }
            if prev.hash != current.prev_hash {
                return (false, i as i64)
            }
            if current.timestamp < prev.timestamp {
                return (false, i as i64)
            }
            if !self.authorities.contains(&current.validator) {
                return (false, i as i64)
            }
        }
        (true, -1)
    }
    
    fn get_balance(&self, address: &str, currency: &str) -> Option<u128> {
        if !self.currencies.contains(currency) {
            return None
        }
        let mut balance: u128 = 0;
        for block in &self.chain {
            for trn in &block.transactions {
                if trn.currency == currency {
                    if trn.receiver == address {
                        balance += trn.amount;
                    }
                    if trn.sender == address && trn.sender != String::from("system") {
                        if balance >= trn.amount {
                            balance -= trn.amount;
                        } else {
                            return None
                        }
                    }
                }
            }
        }
        Some(balance)
    }
}
