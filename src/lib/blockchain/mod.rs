// Dependencies
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};
use serde::{Deserialize, Serialize};
// use sha2::{Sha256, Digest}; // Digest permite que se pueda hacer el new() por que?

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
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(
            1,
            lib::get_timestamp(),
            String::from("This is the genesis block"),
            [String::from("00000000")],
        );

        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block.clone());
    }
}

    let encoded = bincode::serialize(&genesis_block).unwrap();

pub fn get_timestamp() -> u128 {
    let tmp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap(); // Leer Error Handling

    tmp.as_secs() as u128
}

// pub fn show_hash ()  {
//     // create a Sha256 object
//     let mut hasher = Sha256::new();
//     hasher.input(b"hholilil");
//     let result = hasher.result();
//     println!("{:x}", result) // con {} daba fallo, por qué?
//     // result.to_vec()

// }

// pub fn Serialize {

// }

// pub fn BlockchainIterator(index: u8) {

// }

    // let decoded = bincode::deserialize(&encoded[..]).unwrap();
    // println!("the bytecode is {:#?}", encoded);
    // blockchain.add_block(genesisblock);

    // Función que obtiene el hash de un bloque
pub fn get_hash(data: &String) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.input(&data);
    let result = hasher.result();
    result
}



pub fn get_hash(data: &String) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let result = hasher.result();
    result
}