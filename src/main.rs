use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

struct Block {
    index: u64,
    data: String,
    prev_hash: String,
    hash: String,
    timestamp: u64
}

struct BC {
    chain: Vec<Block>,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let hash = calc_hash(index, &data, &prev_hash, timestamp);
        Self {
            index,
            data,
            hash,
            prev_hash,
            timestamp
        }
    }  
}

impl BC {
    fn init() -> Self {
        let init_block = Block::new(0, String::from("init"), String::from("0"));
        Self {
            chain: vec![init_block],
        }
    }

    fn add(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let index = prev_block.index + 1;
        let prev_hash = prev_block.hash.clone();
        let block = Block::new(index, data, prev_hash);
        self.chain.push(block);
    }

    fn is_valid(&self) -> (bool, u8) {
        let mut valid: bool = true;
        let mut status_code: u8 = 0;
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let current = &self.chain[i];
            if calc_hash(current.index, &current.data, &current.prev_hash, current.timestamp) != current.hash {
                valid = false;
                status_code += 1;
            }
            if prev.hash != current.prev_hash {
                valid = false;
                status_code += 2;
            }
            if current.timestamp <= prev.timestamp {
                valid = false;
                status_code += 3;
            }
        }
        (valid, status_code)
    }
}

fn main() {
    let mut bc = BC::init();
    bc.add(String::from("first block"));
    bc.add(String::from("second block"));

    for block in &bc.chain {
        println!("prev_hash: {}\n{} {}\nhash: {}\n", block.prev_hash, block.index, block.data, block.hash);
    }
}

fn calc_hash(i: u64, data: &str, prev: &str, ts: u64) -> String {
    let form = format!("{}{}{}{}", i, data, prev, ts);
    let mut hasher = Sha256::new();
    hasher.update(form.as_bytes());
    let result = hasher.finalize();
    return format!("{}", hex::encode(result))
}
