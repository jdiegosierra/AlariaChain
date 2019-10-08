// modules
mod lib;
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
                .about("add data to the blockchain")
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
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("init") {
        lib::blockchain::Blockchain::new(String::from("./config/genesis.json"), String::from("./config/genesis.json"));
    }
    if let Some(matches) = matches.subcommand_matches("addblock") {
        if matches.is_present("data") {
           lib::blockchain::Blockchain::add_block(&String::from(matches.value_of("data").unwrap()));
        }
    }
    if let Some(_matches) = matches.subcommand_matches("printchain") {
        lib::blockchain::Blockchain::print_chain();
    }
    if let Some(_matches) = matches.subcommand_matches("printgenesis") {
        lib::blockchain::Blockchain::print_genesis(String::from("./config/genesis.json"));
    }
    if let Some(_matches) = matches.subcommand_matches("dropchain") {
        lib::blockchain::Blockchain::drop_chain();
    }
}
