// modules
mod lib;

#[derive(Debug)]
// Creamos estructura publica de bloque, para que pueda ser accesible desde los padres.
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: [String; 1], // TODO: Aumentar número de transacciones. Usar bytes.
    pub prev: [String; 1],
    pub hash: [String; 1]
}

fn main() {
    println!("Welcome to AlariaChain!");

    let block1 = Block {
        index: 1,
        timestamp: lib::getTimestamp(),
        data: [String::from("Alice => Bob 1BTC")],
        prev: [String::from("previous hash")],
        hash: [String::from("hash of this block")]
    };

    println!("El primer bloque es: ");
    println!("{:#?}", block1);
}
