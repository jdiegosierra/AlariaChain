use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256 // Digest permite que se pueda hacer el new() por que?
};

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use merkle::{Hashable, MerkleTree, Proof};
use super::transaction;
use ring::digest::{Algorithm, Context, SHA512};

// #[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

pub fn get_hash(data: &String) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let result = hasher.result();
    result
}

pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}

pub fn get_timestamp() -> u128 {
    let tmp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap(); // Leer Error Handling

    tmp.as_secs() as u128
}

pub fn get_merkle(transactions: Vec<transaction::Transaction>) -> Vec<u8> {
    let tmp = MerkleTree::from_vec(digest, transactions);
    println!("{:?}", tmp);
    // let encoded = bincode::serialize(&transaction).unwrap_or_default(); // No se puede hacer en una sola línea let encoded: &[u8] = bincode::serialize(&block).unwrap();
    // let c: &[u8] = &encoded;
    Vec::new()
}

pub fn print_merkle(transactions: Vec<transaction::Transaction>) {
    let tmp = MerkleTree::from_vec(digest, transactions);
    println!("err merkle");
    println!("{:#?}", tmp);
    // let encoded = bincode::serialize(&transaction).unwrap_or_default(); // No se puede hacer en una sola línea let encoded: &[u8] = bincode::serialize(&block).unwrap();
    // let c: &[u8] = &encoded;
}

// pub fn generate_keypair() {
//     let mut csprng: OsRng = OsRng::new().unwrap();
//     let keypair: Keypair = Keypair::generate(&mut csprng);
//     println!("Keypair: {}", keypair);
// }