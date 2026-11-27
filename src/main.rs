mod transaction;
mod block;
mod blockchain;
mod utils;

use transaction::Transaction;
use blockchain::BC;
use utils::calc_hash;

fn main() {
    let mut bc = BC::init();
    bc.add(vec![Transaction { sender: String::from("test1"), receiver: String::from("test2"), amount: 50, currency: String::from("NXC")}], "server");
    bc.add(vec![Transaction { sender: String::from("test2"), receiver: String::from("test1"), amount: 50, currency: String::from("NXC")}], "server");
    let (is_valid, invalid_index) = bc.is_valid();
    let empty = bc.is_empty();
    let same = utils::test_calc_hash_same_input();
    let diff = utils::test_calc_hash_diff_input();
    
    println!("[BC] authorities: {}", bc.authorities.join(", "));
    for block in &bc.chain {
        println!("[BLC] {}: prev_hash: {}, validator: {}\n", block.index, block.prev_hash, block.validator);
        for (i, trn) in block.transactions.iter().enumerate() {
            println!("[TRN] {}: sender: {}, receiver: {}, amount: {}, currency: {}\n", i, trn.sender, trn.receiver, trn.amount, trn.currency)
        }
        println!("[BLC] hash: {}\n", block.hash);
    }
    println!("[BC] valid: {}, last invalid block index (-1 if chain is valid): {}", is_valid, invalid_index);
    println!("[BC] empty: {} same_hashes_are_the_same: {} different_hashes_are_different: {}", empty, same, diff)
}