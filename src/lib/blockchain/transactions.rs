extern crate nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
	id: Vec<u8>,
	vin: Vec<TXInput>,
	vout: Vec<TXOutput>
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

pub fn transaction(to: String, mut data: String) -> Transaction {
    if data.is_empty() {
		data = format!("Reward to {}", to);
	}
	let txin = TXInput{txid: vec![], vout: -1, script_sig: data};
	let txout = TXOutput{value: 10, script_pubkey: to};
    let id: Vec<u8> = nanoid::simple().into_bytes();
	Transaction{id: id, vin: vec![txin], vout: vec![txout]}
}