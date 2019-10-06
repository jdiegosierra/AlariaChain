// modules
// mod lib/blockchain;
extern crate clap;
use clap::App;

fn main() {

    let matches = App::new("Alaria Chain")
        .version("0.0.1")
        .author("J. Diego Sierra Fernández <jdiego.sierra@hotmail.com>")
        .about("The Blockchain")
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .arg("<output> 'Sets an optional output file'")
        .arg("-d... 'Turn debugging information on'")
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg("-l, --list 'lists test values'"),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("d") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    println!("FIN!");
}
