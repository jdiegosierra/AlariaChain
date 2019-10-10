// DEPENDENCIES
use serde::{Deserialize, Serialize};
use std::fs;
// use serde_json::{Result, Value};
// use serde_json::json;

mod transaction;

// STRUCTS
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub transactions: Vec<transaction::Transaction>, // TODO: Aumentar número de transacciones. Usar bytes.
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
    // fn new(index: u32, timestamp: u128, data: Vec<transactions::Transaction>, prev: Vec<u8>) -> Self {

    //     Block {
    //         index,
    //         timestamp,
    //         transactions: data,
    //         prev,
    //         hash: utils::get_hash(tmp).to_vec(),
    //     }
    // }
}

impl Blockchain {
    // pub fn new(genesis_file: String, _config_file: String) -> Self {
    //     let file_content: String = utils::read_file(&genesis_file);
    //     Blockchain::add_block(&file_content);
    //     Blockchain{}
    // }

    // pub fn add_block(data: &String) {
    //     let index = db::get_last_key();
    //     let value = db::get_last_value();
    //     let decoded: Block = bincode::deserialize(&value).unwrap();
    //     println!("El anterior Bloque es {:#?}", decoded);

    //     let block = Block::new(
    //         index+1,
    //         utils::get_timestamp(),
    //         data.clone(),
    //         decoded.hash
    //     );

    //     println!("El nuevo Bloque es {:?}", block);

    //     let encoded = bincode::serialize(&block).unwrap(); // No se puede hacer en una sola línea let encoded: &[u8] = bincode::serialize(&block).unwrap();
    //     let c: &[u8] = &encoded;
    //     db::store_data(index+1, c);
    // }

    pub fn print_genesis(genesis_file: String) {
        let file_content: String = utils::read_file(&genesis_file);
        let my_obj: Genesis = serde_json::from_str(&file_content).unwrap();
        println!("{:#?}", my_obj);
    }

    pub fn print_chain() {
        let blocks = db::iterate();
         println!("value: {:?}", blocks);
    }

    pub fn drop_chain() {
        fs::remove_dir_all("./db").unwrap();
    }

    // pub fn find_uspent_transactions() {
    //     let blocks = db::iterate();
    // }

    pub fn add_transaction(to: String, data: String) {
        let path = String::from("./db/transactions");
        let index = match db::get_last_key(path.clone()) {
            None => 0 as u32,   
            Some(index) => index as u32
        };
        // let index = 0;
        let transaction = transactions::Transaction::new(to, data);
        let encoded = bincode::serialize(&transaction).unwrap(); // No se puede hacer en una sola línea let encoded: &[u8] = bincode::serialize(&block).unwrap();
        let c: &[u8] = &encoded;
        db::store_data(path, index+1, c);
    }

    pub fn print_transactions() {
        let transactions = db::iterate_transactions(String::from("./db/transactions"));
        println!("value: {:?}", blocks);        
    }
}
