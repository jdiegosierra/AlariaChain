use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use leveldb::database::iterator::Iterable;
use std::path::Path;
use serde::{Deserialize, Serialize};

// let mut options = Options::new();
// options.create_if_missing = true;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String, // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: Vec<u8>,
    pub hash: Vec<u8>,
}

pub fn iterate(path: String) -> Vec<Block> {
    let mut result = Vec::new();
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    let _res = database.get(read_opts, 1); // POR QUE KEY DA ERROR SI COMENTO ESTO????
    // match res {
    //     Ok(data) => {
    //         println!("the bytecode is");
    //         match data {
    //             Some(inner) => {
    //                 let decoded: Block = bincode::deserialize(&inner).unwrap();
    //                 println!("{:?}", decoded);
    //             },
    //             None => println!("No gift? Oh well."),
    //         }
    //     }
    //     Err(e) => panic!("failed reading data: {:?}", e),
    // }
    for (_key, value) in database.iter(ReadOptions::new()) {
        // println!("Key: {}", key);
        let decoded: Block = bincode::deserialize(&value).unwrap();
        // let c: &[u8] = &value;
        // let decoded: Block = bincode::deserialize(c).unwrap(); 
        // match value {
        //     Some(inner) => {
        //         let decoded: Block = bincode::deserialize(&inner).unwrap();
        //         println!("{:?}", decoded);
        //     },
        //     None => println!("No gift? Oh well."),
        // }
        // println!("value: {:?}", decoded);
        result.push(decoded);
    };
    result
}

pub fn iterate_transactions(path: String) -> Vec<transaction::Transaction> {
    let mut result = Vec::new();
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new(path);
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    let _res = database.get(read_opts, 1); // POR QUE KEY DA ERROR SI COMENTO ESTO????

    for (_key, value) in database.iter(ReadOptions::new()) {
        // println!("Key: {}", key);
        let decoded: Block = bincode::deserialize(&value).unwrap();
        // let c: &[u8] = &value;
        // let decoded: Block = bincode::deserialize(c).unwrap(); 
        // match value {
        //     Some(inner) => {
        //         let decoded: Block = bincode::deserialize(&inner).unwrap();
        //         println!("{:?}", decoded);
        //     },
        //     None => println!("No gift? Oh well."),
        // }
        // println!("value: {:?}", decoded);
        result.push(decoded);
    };
    result
}

pub fn get_last_key(path: String) -> Option<u32> {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new(&path);
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };
    // let read_opts = ReadOptions::new();
    // let _res = database.get(read_opts, 1);
    let result = match database.iter(ReadOptions::new()).next() {
        Some(data) => Some(data.0 as u32),
        None => None
    };
    result
}

pub fn get_last_value(path: String) -> Vec<u8> {
    let mut options = Options::new();
    options.create_if_missing = true;
    println!("2,1100");
    let path = Path::new(&path);
    println!("2,111");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };
    println!("2,112");
    let read_opts = ReadOptions::new();
    println!("2,113");
    let res = database.get(read_opts, 1);
    println!("2,114");
    match res {
        Ok(_data) => {
            let (_key, value) = database.iter(ReadOptions::new()).last().unwrap();
            value
        }
        Err(_e) => { 
            Vec::new()
        }
    }

    // for (key, value) in database.iter(ReadOptions::new()) {
    //     println!("Key: {}", key);
    //     println!("value: {:?}", value);
    // }
}

pub fn store_data(path: String, index: u32, data: &[u8]) {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new(&path);
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };
    let read_opts = ReadOptions::new();
    database.get(read_opts, 1).unwrap();
    let write_opts = WriteOptions::new();
    match database.put(write_opts, index as i32, data) {
        Ok(_) => (),
        Err(e) => panic!("failed to write to database: {:?}", e),
    };
}




//     let read_opts = ReadOptions::new();
//     let res = database.get(read_opts, 1);
//     match res {
//         Ok(data) => {
//             // assert!(data.is_some());
//             // assert_eq!(data, Some(vec![1]));
//             // println!("Los datos son");
//             // println!("{:#?}", data);
//             // let decoded = bincode::deserialize(&data[..]).unwrap();
//             println!("the bytecode is");
//             match data {
//                 Some(inner) => {
//                     let decoded: Block = bincode::deserialize(&inner).unwrap();
//                     println!("{:?}", decoded);
//                 },
//                 None => println!("No gift? Oh well."),
//             }
//         }
//         Err(e) => panic!("failed reading data: {:?}", e),
//     }
// let mut database = match Database::open(&path, options) {
//     Ok(db) => db,
//     Err(e) => panic!("failed to open database: {:?}", e),
// };