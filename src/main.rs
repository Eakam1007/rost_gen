use std::{env, fs};

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
    } else if option_arg == "-i" || option_arg == "--input" {
        let file_path = &args[2];
        // let contents = fs::read_to_string(file_path).expect("Failed to read file");

        // fs::remove_dir_all("./dist").expect("Failed to create output directory");
        // fs::create_dir_all("./dist").expect("Failed to create output directory");

        let html_file_name: String;
        match file_path.rfind('/') {
            None => html_file_name = file_path.chars().take(file_path.chars().count() - 4).collect(),
            Some(n) => html_file_name = file_path.chars().skip(n + 1).take(file_path.chars().count() - (n + 5)).collect(),
        };
        
        println!("New file name: {html_file_name}.html");
        println!("New file path: ./dist/{html_file_name}.html");
        // fs::write("./dist/{html_file_name}.html", contents).expect("Failed to generate html file");
    }
}
