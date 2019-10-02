// modules
mod lib;

use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};



impl Block {
    fn new(index: u32, timestamp: u128, data: [String; 1], prev: [String; 1]) -> Self {
        Block {
            index,
            timestamp,
            data,
            prev,
            hash: [self.get_hash()] // am I calling correctly?
        }
    }

    fn get_hash(&mut self) -> GenericArray<u8, U32> {
        let mut hasher = Sha256::new();
        hasher.input(self.data[0]);
        let result = hasher.result();
        result
    }
}

fn main() {

    let data = String::from("Alice => Bob 1BTC");

    let block1 = Block::new(1, lib::get_timestamp(), [data.clone()], [String::from("00000000")]); // ERROR: expected 5 parameters
}