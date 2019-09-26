// Dependencies
use std::time::{SystemTime, UNIX_EPOCH};

pub fn getTimestamp() -> u128 {
    let tmp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap(); // Leer Error Handling

    tmp.as_secs() as u128
}

// fn setHash () {

// }