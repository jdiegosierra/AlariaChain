// modules
mod transaction;
mod blockchain;
mod db;
mod utils;

extern crate clap;
use clap::{App, Arg};

fn main() {

    let matches = App::new("AlariaChain")
        .version("1.0")
        .author("J. Diego Sierra F. <jdiego.sierra@hotmail.com>")
        .about("The Blockchain")
        .subcommand(
            App::new("init")
                .about("Start Blockchain with genesis block")
        )
        .subcommand(
            App::new("addblock")
                .about("DEPRECATED: Use mineblock command")
                .arg(Arg::with_name("data")
                    .help("Data to store into the Blockchain")
                    .takes_value(true)
                    .required(true)
                )
        )
        .subcommand(
            App::new("printchain")
                .about("Print Blockchain")
        )
        .subcommand(
            App::new("printgenesis")
                .about("Print genesis file")
        )
        .subcommand(
            App::new("dropchain")
                .about("Remove the blockchain")
        )
        .subcommand(
            App::new("keypair")
                .about("Generate key pair")
        )
        .subcommand(
            App::new("addtransaction")
                .about("Add a transaction")
                .arg(Arg::with_name("to")
                    .long("to")
                    .help("ID of the destinatary")
                    .takes_value(true)
                    .required(true)
        )
                .arg(Arg::with_name("data")
                    .long("data")
                    .help("Data to store into the Blockchain")
                    .takes_value(true)
                )
        )
        .subcommand(
            App::new("printtransactions")
                .about("Add a transaction")
        )
        .subcommand(
            App::new("mineblock")
                .about("Mine all transactions")
        )
        .get_matches();

    // if let Some(_matches) = matches.subcommand_matches("init") {
    //     lib::blockchain::Blockchain::new(String::from("./config/genesis.json"), String::from("./config/genesis.json"));
    // }
    if let Some(matches) = matches.subcommand_matches("addblock") {
        println!("DEPRECATED: Use mineblock command");
        // if matches.is_present("data") {
        //    lib::blockchain::Blockchain::add_block(&String::from(matches.value_of("data").unwrap()));
        // }
    }
    if let Some(_matches) = matches.subcommand_matches("printchain") {
        blockchain::Blockchain::print_chain();
    }
    if let Some(_matches) = matches.subcommand_matches("printgenesis") {
        blockchain::Blockchain::print_genesis(String::from("./config/genesis.json"));
    }
    if let Some(_matches) = matches.subcommand_matches("dropchain") {
        blockchain::Blockchain::drop_chain();
    }
    // if let Some(_matches) = matches.subcommand_matches("keypair") {
    //     lib::blockchain::Blockchain::utils::generate_keypair();
    // }
    if let Some(input) = matches.subcommand_matches("addtransaction") {
        blockchain::Blockchain::add_transaction(String::from(input.value_of("to").unwrap()), String::from(input.value_of("data").unwrap_or("")));
    }
    if let Some(input) = matches.subcommand_matches("printtransactions") {
        blockchain::Blockchain::print_transactions();
    }
    if let Some(_matches) = matches.subcommand_matches("mineblock") {
    }
}
