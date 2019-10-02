// Dependencies
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};
// use sha2::{Sha256, Digest}; // Digest permite que se pueda hacer el new() por que?

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



pub fn get_hash(data: &String) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let result = hasher.result();
    result
}