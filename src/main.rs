// modules
mod lib;

use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
// Creamos estructura publica de bloque, para que pueda ser accesible desde los padres.
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String, // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: [String; 1],
    pub hash: GenericArray<u8, U32>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Test {
    pub index: u32,
    pub timestamp: u128,
    pub data: String, // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: [String; 1]
}


pub struct Blockchain {
    pub blocks: Vec<Block>
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
            hash: get_hash(tmp)
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
        Blockchain {
            blocks: vec![]
        }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block.clone());
    }
}

// pub fn Serialize {

// }

fn main() {
    
    println!("Welcome to AlariaChain!");
    let world = Test {
        index: 1,
        timestamp: lib::get_timestamp(),
        data: String::from("This is the genesis block"), // TODO: Aumentar número de transacciones. Usar bytes.
        prev: [String::from("00000000")]
    };

    let encoded = bincode::serialize(&world).unwrap();

    println!("the bytecode is {:#?}", encoded);

    let decoded = bincode::deserialize(&encoded[..]).unwrap();

    assert_eq!(world, decoded, "we are testing addition with {:#?} and {:#?}", world, decoded);
    // println!("Creating genesis Block...");
    // let genesisblock = Block::new(1, lib::get_timestamp(), String::from("This is the genesis block"), [String::from("00000000")]);

    // // println!("El primer bloque es: ");
    // // println!("{:#?}", genesisblock);

    // let mut blockchain = Blockchain::new();
    // println!("Adding genesis Block...");
    // blockchain.add_block(genesisblock);
}
