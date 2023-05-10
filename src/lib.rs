use data_encoding::HEXUPPER;
use ring::digest;
use std::time::SystemTime;

pub struct Block {
    pub index: u64,
    pub data: String,
    pub current_hash: String,
    pub previous_hash: String,
    pub timestamp: SystemTime,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Block {
        let current_hash = hash(&data);
        Block {
            index,
            data,
            current_hash: current_hash.clone(),
            previous_hash,
            timestamp: SystemTime::now(),
        }
    }
}

fn hash(data: &str) -> String {
    HEXUPPER.encode(digest::digest(&digest::SHA256, &data.as_bytes()).as_ref())
}
