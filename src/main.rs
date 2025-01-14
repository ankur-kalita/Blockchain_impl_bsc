use core::hash;
use std::fmt::format;
use sha256::digest;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]

struct Blockchain {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
    fn starting_block(&mut self) {
        let genesis_block: Block = Block {
            id: 1,
            data: String::from("I am a first or genesis block"),
            previous_hash: String::from(
                "0000000000000000000000000000000000000000000000000000000000000000",
            ),
            nonce: 11316,
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
            timestamp: Utc::now().timestamp(),
        };
        self.blocks.push(genesis_block);
    }
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now: DateTime<Utc> = Utc::now();
        let now_timestamp: i64 = now.timestamp();
        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data);
        Self {
            id, 
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce
        }
    }
    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("mining block...");
        let mut nonce= 1;
        loop {
            let block_string: String = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            let hash: String = digest(block_string);
            if hash.starts_with("0000") {
                println!("mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash)
            }
            nonce += 1;  
        }
    }
}
fn main() {
    let mut new_BC: Blockchain = Blockchain::new();
    new_BC.starting_block();

    println!("{:?}", new_BC);
    let new_block: Block = Block::new( 2,  new_BC.blocks[0].hash.to_owned(),"Ankur".to_string());
}
