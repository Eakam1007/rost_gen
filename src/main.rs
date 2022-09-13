use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("WIP");
        return;
    }

    let option_arg = &args[1];

    if option_arg == "-v" || option_arg == "--version" {
        println!("Version 0.1");
    } else {
        println!("Invalid option {}", option_arg);
    }
}
