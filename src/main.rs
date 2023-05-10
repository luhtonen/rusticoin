use rusticoin::Block;

fn main() {
    let block = Block::new(0, String::from("data"), String::from("block"));
    println!("hash: {}", block.current_hash);
}
