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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("addblock") {
        if matches.is_present("data") {
            
           lib::blockchain::addblock(&String::from("00000000"));
        }
    }
    if let Some(_matches) = matches.subcommand_matches("printchain") {
        lib::blockchain::printchain();
    }

    println!("FIN!");
}
