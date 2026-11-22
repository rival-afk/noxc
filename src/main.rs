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

impl BC {
    fn init() -> Self {
        let index: u64 = 0;
        let data: String = String::from("init");
        let prev_hash: String = String::from("0");
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let hash: String = calc_hash(index, &data, &prev_hash, timestamp);
        let mut chain: Vec<Block> = vec![];
        let init_block: Block = Block {
            index,
            data,
            hash,
            prev_hash,
            timestamp
        };
        chain.push(init_block);
        Self {
            chain
        }
    }
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

fn main() {
    let mut idk: Vec<Block> = vec![];
    idk.push(Block::new(
        1,
        String::from("test1"),
        String::from("0")
    ));
    idk.push(Block::new(
        2,
        String::from("test2"),
        String::from(idk[0].hash.clone()),
        
    ));
    for block in &idk {
        println!("index: {}\ndata: {}", block.index, block.data);
    }
}

fn calc_hash(i: u64, data: &str, prev: &str, ts: u64) -> String {
    let form = format!("{}{}{}{}", i, data, prev, ts);
    let mut hasher = Sha256::new();
    hasher.update(form.as_bytes());
    let result = hasher.finalize();
    return format!("{}", hex::encode(result))
}
