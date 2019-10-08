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

pub fn iterate() {
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
    for (key, value) in database.iter(ReadOptions::new()) {
        println!("Key: {}", key);
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
        println!("value: {:?}", decoded);
    }
}

pub fn get_last_key() -> u32 {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    database.get(read_opts, 1).unwrap(); // POR QUE KEY DA ERROR SI COMENTO ESTO????
    let (key, _value) = database.iter(ReadOptions::new()).last().unwrap();
    key as u32
    // for (key, value) in database.iter(ReadOptions::new()) {
    //     println!("Key: {}", key);
    //     println!("value: {:?}", value);
    // }
}

pub fn get_last_value() -> Vec<u8> {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    database.get(read_opts, 1).unwrap(); // POR QUE KEY DA ERROR SI COMENTO ESTO????
    let (_key, value) = database.iter(ReadOptions::new()).last().unwrap();
    value
    // for (key, value) in database.iter(ReadOptions::new()) {
    //     println!("Key: {}", key);
    //     println!("value: {:?}", value);
    // }
}

pub fn store_data(index: u32, data: &[u8]) {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
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

