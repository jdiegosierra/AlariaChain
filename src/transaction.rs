extern crate nanoid;
use serde::{Deserialize, Serialize};
use ring::digest::Context;
use ring::digest::{Algorithm, SHA512};
use std::str;

// #[allow(non_upper_case_globals)]
// static digest: &'static Algorithm = &SHA512;
// pub trait Hashable {
//     fn bytes (&self) -> Vec<u8>;

//     fn hash (transaction: &Transaction) -> Vec<u8> {
//         bincode::serialize(&transaction).unwrap()
//     }
// }

// impl Hashable for Transaction {
//     fn bytes (&self) -> Vec<u8> {
// 		bincode::serialize(&self).unwrap()
//     }
// }


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
	id: Vec<u8>,
	vin: Vec<TXInput>,
	vout: Vec<TXOutput>,
	encoded: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
	value: i32,
	script_pubkey: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
	txid: Vec<u8>,
	vout: i32,
	script_sig: String
}

impl Transaction {
	pub fn new(to: String, mut data: String) -> Transaction {
		if data.is_empty() {
			data = format!("Reward to {}", to);
		}
		let txin = TXInput{txid: vec![], vout: -1, script_sig: data};
		let txout = TXOutput{value: 10, script_pubkey: to};
		let id: Vec<u8> = nanoid::simple().into_bytes();
		Transaction{id: id, vin: vec![txin], vout: vec![txout], encoded: Vec::new()}
	}
	// pub fn as_ref(&self) -> &[u8] {
	// 	let encoded = bincode::serialize(self).unwrap();
	// 	let c: &[u8] = &encoded;
	// 	c		
    //     // str::from_utf8(&encoded).unwrap()
    // }
}

impl AsRef<[u8]> for Transaction {
    fn as_ref(&self) -> &[u8] {
		// let encoded = bincode::serialize(self).unwrap();
		// let c: &[u8] = &encoded;
		// self.tmp = encoded;
		// self.get_serialize()
		&self.encoded
        // str::from_utf8(&encoded).unwrap()
    }
}

// impl Transaction {
// 	fn update_context(&self, context: &mut Context) {
// 		let bytes: Vec<u8> = self.serialize();
// 		context.update(&bytes);
// 	}
// }

impl TXInput {
	pub fn can_unlock_output_with(self, unlocking_data: String) -> bool {
		self.script_sig == unlocking_data
	}
}

impl TXOutput {
	pub fn can_unlock_output_with(self, unlocking_data: String) -> bool {
		self.script_pubkey == unlocking_data
	}
}