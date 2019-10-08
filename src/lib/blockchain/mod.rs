// DEPENDENCIES
mod db;
mod utils;
use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use serde_json::json;


// STRUCTS
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String, // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: Vec<u8>,
    pub hash: Vec<u8>
}

// GENESIS STRUCT
#[derive(Serialize, Deserialize, Debug)]
struct Genesis {
    genesis_time: String,
    chain_id: String,
    validators: Vec<Validator>,
    app_hash: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Validator {
    pub address: String,
    pub pub_key: Key,
    pub power: String, 
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Key {
    pub r#type: String,
    pub value: String
}

// OBJECTS
impl Block {
    fn new(index: u32, timestamp: u128, data: String, prev: Vec<u8>) -> Self {
        let tmp = &data; // se puede hacer sin esto?

        Block {
            index,
            timestamp,
            data: tmp.clone(),
            prev,
            hash: utils::get_hash(tmp).to_vec(),
        }
    }
}

impl Blockchain {
    pub fn new(genesis_file: String, _config_file: String) -> Self {
        let file_content: String = utils::read_file(&genesis_file);
        Blockchain::add_block(&file_content);
        Blockchain{}
    }

    pub fn add_block(data: &String) {
        let index = db::get_last_key();
        let value = db::get_last_value();
        let decoded: Block = bincode::deserialize(&value).unwrap();
        println!("El anterior Bloque es {:#?}", decoded);

        let block = Block::new(
            index+1,
            utils::get_timestamp(),
            data.clone(),
            decoded.hash
        );

        println!("El nuevo Bloque es {:?}", block);

        let encoded = bincode::serialize(&block).unwrap(); // No se puede hacer en una sola línea let encoded: &[u8] = bincode::serialize(&block).unwrap();
        let c: &[u8] = &encoded;
        db::store_data(index+1, c);
    }

    pub fn print_genesis(genesis_file: String) {
        let file_content: String = utils::read_file(&genesis_file);
        let my_obj: Genesis = serde_json::from_str(&file_content).unwrap();
        println!("{:#?}", my_obj);
    }

    pub fn print_chain() {
        db::iterate();
    }
}
