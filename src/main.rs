use rusticoin::Block;

fn main() {
    let mut blockchain = vec![Block::first(None)];
    for i in 1..5 {
        let previous = &blockchain[i - 1];
        let new_block = Block::next(previous);
        blockchain.push(new_block);
    }
    println!("{:#?}", blockchain);
}
