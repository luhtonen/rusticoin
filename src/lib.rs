mod blockchain;

use data_encoding::HEXUPPER;
use ring::digest;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub data: String,
    pub current_hash: String,
    pub previous_hash: String,
    pub timestamp: SystemTime,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now();
        let (nonce, current_hash) = Self::proof_of_work(
            index,
            &data,
            &previous_hash,
            timestamp,
            Some("00".to_string()),
        );
        Block {
            index,
            nonce,
            data,
            current_hash,
            previous_hash,
            timestamp,
            transactions,
        }
    }

    pub fn first(data: Option<String>) -> Block {
        Block::new(
            0,
            data.unwrap_or(String::from("Genesis Block")),
            String::from("0"),
            vec![],
        )
    }

    pub fn next(previous_block: &Block, transactions: Vec<Transaction>) -> Block {
        Block::new(
            previous_block.index + 1,
            format!("Transaction data number ({})", previous_block.index + 1),
            previous_block.current_hash.clone(),
            transactions,
        )
    }
    fn proof_of_work(
        index: u64,
        data: &str,
        previous_hash: &str,
        timestamp: SystemTime,
        difficulty: Option<String>,
    ) -> (u64, String) {
        let difficulty = match difficulty {
            Some(value) => value,
            None => "00".to_string(),
        };
        let mut nonce = 0;
        loop {
            let value = format!(
                "{}{}{}{}{}",
                nonce,
                index,
                timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                data,
                previous_hash
            );
            let hash = HEXUPPER.encode(digest::digest(&digest::SHA256, value.as_bytes()).as_ref());
            if hash.starts_with(&difficulty) {
                return (nonce, hash);
            } else {
                nonce += 1;
            }
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: i32,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: i32) -> Self {
        Transaction { from, to, amount }
    }
}
