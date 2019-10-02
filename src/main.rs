// modules
mod lib;

use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};

#[derive(Debug)]
#[derive(Clone)]
// Creamos estructura publica de bloque, para que pueda ser accesible desde los padres.
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String, // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: [String; 1],
    pub hash: GenericArray<u8, U32>
}

pub struct Blockchain {
    pub blocks: Vec<Block>
}

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

fn main() {
    
    println!("Welcome to AlariaChain!");

    println!("Creating genesis Block...");
    let genesisblock = Block::new(1, lib::get_timestamp(), String::from("This is the genesis block"), [String::from("00000000")]);

    // println!("El primer bloque es: ");
    // println!("{:#?}", genesisblock);

    let mut blockchain = Blockchain::new();
    println!("Adding genesis Block...");
    blockchain.add_block(genesisblock);
}
