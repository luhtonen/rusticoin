use data_encoding::HEXUPPER;
use ring::digest::{self, Digest};

fn main() {
    println!("hash: {}", HEXUPPER.encode((hash("Hello, world!").as_ref())));
}

fn hash(data: &str) -> Digest {
    digest::digest(&digest::SHA256, &data.as_bytes())
}
