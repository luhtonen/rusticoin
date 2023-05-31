use crate::{Block, Transaction};

struct Blockchain {
    pub chain: Vec<Block>,
    pub uncommitted_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let chain = vec![Block::first(None)];
        Blockchain {
            chain,
            uncommitted_transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.uncommitted_transactions.push(transaction);
    }
}
