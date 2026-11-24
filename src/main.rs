use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write;

struct Block {
    index: u64,
    transactions: Vec<Transaction>,
    prev_hash: String,
    hash: String,
    timestamp: u64
}

struct Transaction {
    sender: String,
    receiver: String,
    amount: u128,
    currency: String
}

struct BC {
    chain: Vec<Block>,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let hash = calc_hash(index, &transactions, &prev_hash, timestamp);
        Self {
            index,
            transactions,
            hash,
            prev_hash,
            timestamp
        }
    }  
}

impl BC {
    fn init() -> Self {
        let init_block = Block::new(0, vec![Transaction { sender: String::from("init"), receiver: String::from("init"), amount: 0, currency: String::from("Undefined") }], String::from("0"));
        Self {
            chain: vec![init_block],
        }
    }

    fn add(&mut self, tns: Vec<Transaction>) {
        let prev_block = self.chain.last().unwrap();
        let index = prev_block.index + 1;
        let prev_hash = prev_block.hash.clone();
        let block = Block::new(index, tns, prev_hash);
        self.chain.push(block);
    }

    fn is_valid(&self) -> (bool, u64) {
        let genesis = &self.chain[0];
        if calc_hash(genesis.index, &genesis.transactions, &genesis.prev_hash, genesis.timestamp) != genesis.hash {
            return (false, 0)
        }
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let current = &self.chain[i];
            if calc_hash(current.index, &current.transactions, &current.prev_hash, current.timestamp) != current.hash {
                return (false, i as u64)
            }
            if prev.hash != current.prev_hash {
                return (false, i as u64)
            }
            if current.timestamp < prev.timestamp {
                return (false, i as u64)
            }
        }
        (true, 0)
    }
}

fn main() {
    let mut bc = BC::init();
    bc.add(vec![Transaction { sender: String::from("test1"), receiver: String::from("test2"), amount: 50, currency: String::from("NXC")}]);
    bc.add(vec![Transaction { sender: String::from("test2"), receiver: String::from("test1"), amount: 50, currency: String::from("NXC")}]);
    let (is_valid, invalid_index) = bc.is_valid();

    for block in &bc.chain {
        println!("[BLC] {}: prev_hash: {}\n", block.index, block.prev_hash);
        for (i, trn) in block.transactions.iter().enumerate() {
            println!("[TRN] {}: sender: {}, receiver: {}, amount: {}, currency: {}\n", i, trn.sender, trn.receiver, trn.amount, trn.currency)
        }
        println!("[BLC] hash: {}\n", block.hash);
    }
    println!("[BC] valid: {}, last invalid block index (0 if chain is valid): {}", is_valid, invalid_index)
}

fn calc_hash(i: u64, tns: &Vec<Transaction>, prev: &str, ts: u64) -> String {
    let mut tns_str = String::new();
    for trn in tns {
        write!(
                tns_str,
                "{}{}{}{}",
                trn.sender, trn.receiver, trn.amount, trn.currency
            ).unwrap();
    }
    let form = format!("{}{}{}{}", i, tns_str, prev, ts);
    let mut hasher = Sha256::new();
    hasher.update(form.as_bytes());
    let result = hasher.finalize();
    return format!("{}", hex::encode(result))
}
