use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use leveldb::iterator::{Iterator, KeyIterator, ValueIterator};
use leveldb::database::iterator::Iterable;
use std::path::Path;


let mut options = Options::new();
options.create_if_missing = true;

let path = Path::new("./db/blockchain");

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

let mut database = match Database::open(&path, options) {
    Ok(db) => db,
    Err(e) => panic!("failed to open database: {:?}", e),
};

