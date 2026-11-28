use crate::transaction::Transaction;
use std::fmt::Write;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn calc_hash(i: u64, tns: &[Transaction], prev: &str, val: &str, ts: u64) -> String {
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

pub fn test_calc_hash_same_input() -> bool {
    let index: u64 = 1;
    let tns: Vec<Transaction> = vec![Transaction { sender: String::from("system"), receiver: String::from("test1"), amount: 1000, currency: String::from("NXC") }];
    let prev: &str = "0";
    let val: &str = "init";
    let ts: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let first = calc_hash(index, &tns, prev, val, ts);
    let second = calc_hash(index, &tns, prev, val, ts);
    if first != second { false; }
    true
}

pub fn test_calc_hash_diff_input() -> bool {
    let (i1, i2): (u64, u64) = (1, 2);
    let (tns1, tns2): (Vec<Transaction>, Vec<Transaction>) = (vec![Transaction { sender: String::from("system"), receiver: String::from("test1"), amount: 1000, currency: String::from("NXC") }], vec![Transaction { sender: String::from("system"), receiver: String::from("test2"), amount: 2000, currency: String::from("NXC") }]);
    let (p1, p2): (&str, &str) = ("0", "1");
    let (v1, v2): (&str, &str) = ("init", "server");
    let (t1, t2): (u64, u64) = ((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 1).try_into().unwrap(), (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()).try_into().unwrap());
    let first = calc_hash(i1, &tns1, p1, v1, t1);
    let second = calc_hash(i2, &tns2, p2, v2, t2);
    if first == second { false; }
    true
}