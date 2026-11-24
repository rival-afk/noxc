use crate::transaction::Transaction;
use std::fmt::Write;
use sha2::{Sha256, Digest};

pub fn calc_hash(i: u64, tns: &Vec<Transaction>, prev: &str, val: &str, ts: u64) -> String {
    let mut tns_str = String::new();
    for trn in tns {
        write!(
                tns_str,
                "{}{}{}{}",
                trn.sender, trn.receiver, trn.amount, trn.currency
            ).unwrap();
    }
    let form = format!("{}{}{}{}{}", i, tns_str, prev, val, ts);
    let mut hasher = Sha256::new();
    hasher.update(form.as_bytes());
    let result = hasher.finalize();
    return format!("{}", hex::encode(result))
}
