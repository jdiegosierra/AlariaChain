use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use leveldb::database::iterator::ValueIterator;
// use leveldb::iterator::{Iterator, KeyIterator, ValueIterator};
use leveldb::database::iterator::Iterable;
use std::path::Path;
// use std::time::{SystemTime, UNIX_EPOCH};
// use sha2::{
//     digest::generic_array::{typenum::U32, GenericArray},
//     Digest, Sha256 // Digest permite que se pueda hacer el new() por que?
// };
// use serde::{Deserialize, Serialize};


// let mut options = Options::new();
// options.create_if_missing = true;


pub fn iterate() {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    database.get(read_opts, 1).unwrap(); // POR QUE KEY DA ERROR SI COMENTO ESTO????

    for (key, value) in database.iter(ReadOptions::new()) {
        println!("Key: {}", key);
        println!("value: {:?}", value);
    }
}

pub fn getLastKey() -> u32 {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    database.get(read_opts, 1).unwrap(); // POR QUE KEY DA ERROR SI COMENTO ESTO????
    let (key, value) = database.iter(ReadOptions::new()).last().unwrap();
    key as u32
    // for (key, value) in database.iter(ReadOptions::new()) {
    //     println!("Key: {}", key);
    //     println!("value: {:?}", value);
    // }
}

pub fn getLastValue() -> Vec<u8> {
    let mut options = Options::new();
    options.create_if_missing = true;
    let path = Path::new("./db");
    let database = match Database::open(&path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let read_opts = ReadOptions::new();
    database.get(read_opts, 1).unwrap(); // POR QUE KEY DA ERROR SI COMENTO ESTO????
    let (key, value) = database.iter(ReadOptions::new()).last().unwrap();
    value
    // for (key, value) in database.iter(ReadOptions::new()) {
    //     println!("Key: {}", key);
    //     println!("value: {:?}", value);
    // }
}


    // let res = database.get(read_opts, 1);
//     // let write_opts = WriteOptions::new();
//     // match database.put(write_opts, 1, &encoded) {
//     //     Ok(_) => (),
//     //     Err(e) => panic!("failed to write to database: {:?}", e),
//     // };

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

