use sha2::{Sha256, Digest};

struct Block {
    index: u64,
    data: String,
    prev_hash: String,
    hash: String
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Self {
        let hash = calc_hash(index, &data, &prev_hash);
        Self {
            index,
            data,
            hash,
            prev_hash,
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
        String::from(idk[0].hash.clone())
    ));
    for block in &idk {
        println!("index: {}\ndata: {}", block.index, block.data);
    }
}

fn calc_hash(i: u64, data: &str, prev: &str) -> String {
    let form = format!("{}{}{}", i, data, prev);
    let mut hasher = Sha256::new();
    hasher.update(form.as_bytes());
    let result = hasher.finalize();
    return format!("{}", hex::encode(result))
}
