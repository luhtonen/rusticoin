use data_encoding::HEXUPPER;
use ring::digest;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub current_hash: String,
    pub previous_hash: String,
    pub timestamp: SystemTime,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now();
        let current_hash = hash(index, &data, &previous_hash, timestamp);
        Block {
            index,
            data,
            current_hash: current_hash.clone(),
            previous_hash,
            timestamp,
        }
    }

    pub fn first(data: Option<String>) -> Block {
        Block::new(
            0,
            data.unwrap_or(String::from("Genesis Block")),
            String::from("0"),
        )
    }

    pub fn next(previous_block: &Block) -> Block {
        Block::new(
            previous_block.index + 1,
            format!("Transaction data number ({})", previous_block.index + 1),
            previous_block.current_hash.clone(),
        )
    }
}

fn hash(index: u64, data: &str, previous_hash: &str, timestamp: SystemTime) -> String {
    let value = format!("{}{}{}{}", index, timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(), data, previous_hash);
    HEXUPPER.encode(digest::digest(&digest::SHA256, value.as_bytes()).as_ref())
}
