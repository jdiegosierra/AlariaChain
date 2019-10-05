// modules
mod lib;

use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};

use serde::{Deserialize, Serialize};

use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use leveldb::iterator::{Iterator, KeyIterator, ValueIterator};
use leveldb::database::iterator::Iterable;
use std::path::Path;

use leveldb::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
// Creamos estructura publica de bloque, para que pueda ser accesible desde los padres.
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String, // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: [String; 1],
    pub hash: Vec<u8>,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

// static targetBits: u16 = 24;

impl Block {
    fn new(index: u32, timestamp: u128, data: String, prev: [String; 1]) -> Self {
        let tmp = &data; // se puede hacer sin esto?

        Block {
            index,
            timestamp,
            data: tmp.clone(),
            prev,
            hash: get_hash(tmp).to_vec(),
        }
    }

    // fn get_hash(mut data: [String; 1]) -> GenericArray<u8, U32> {
    //     let mut hasher = Sha256::new();
    //     hasher.input(&mut data[0]);
    //     let result = hasher.result();
    //     result
    // }
}

// Función que obtiene el hash de un bloque
pub fn get_hash(data: &String) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.input(&data);
    let result = hasher.result();
    result
}

impl Blockchain {
    fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block.clone());
    }
}

// pub fn Serialize {

// }

// pub fn BlockchainIterator(index: u8) {

// }

fn main() {
    println!("Welcome to AlariaChain!");

    println!("Creating genesis Block...");
    let genesis_block = Block::new(
        1,
        lib::get_timestamp(),
        String::from("This is the genesis block"),
        [String::from("00000000")],
    );

    println!("El primer bloque es: ");
    println!("{:#?}", genesis_block);

    let mut blockchain = Blockchain::new();
    println!("Adding genesis Block...");
    let encoded = bincode::serialize(&genesis_block).unwrap();

    let mut options = Options::new();
    options.create_if_missing = true;

    let path = Path::new("./db/blockchain");

    let mut database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    // let write_opts = WriteOptions::new();
    // match database.put(write_opts, 1, &encoded) {
    //     Ok(_) => (),
    //     Err(e) => panic!("failed to write to database: {:?}", e),
    // };

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 1);
    match res {
        Ok(data) => {
            // assert!(data.is_some());
            // assert_eq!(data, Some(vec![1]));
            // println!("Los datos son");
            // println!("{:#?}", data);
            // let decoded = bincode::deserialize(&data[..]).unwrap();
            println!("the bytecode is");
            match data {
                Some(inner) => {
                    let decoded: Block = bincode::deserialize(&inner).unwrap();
                    println!("{:?}", decoded);
                },
                None => println!("No gift? Oh well."),
            }
        }
        Err(e) => panic!("failed reading data: {:?}", e),
    }
    println!("LETS ITERATE");

    for (key, value) in database.iter(ReadOptions::new()) {
        println!("Key: {}", key);
        println!("value: {:?}", value);
    }


    println!("FIN!")
    // let decoded = bincode::deserialize(&encoded[..]).unwrap();
    // println!("the bytecode is {:#?}", encoded);
    // blockchain.add_block(genesisblock);
}
