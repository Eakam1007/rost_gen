use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("WIP");
        return;
    }

    let option_arg = &args[1];

    if option_arg == "-v" || option_arg == "--version" {
        println!("rost_gen version 0.1");
    } else if option_arg == "-h" || option_arg == "--help" {
        println!("Usage: rost_gen [OPTIONS]");
        println!("Options:");
        println!("\t-v, --version\t\t\tPrint the tool name and version");
        println!("\t-h, --help\t\t\tPrint help message");
    } else {
        println!("Invalid option {}", option_arg);
    }
}
