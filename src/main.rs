use std::io::{self, BufRead, Write};
use std::{env, fs, path};

const HTML_TEMPLATE: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\t<meta charset=\"UTF-8\">\n\t<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\t<title>{{title}}</title>\n</head>\n<body>\n\t<p>\n";
const VERSION: &str = "rost_gen version 0.1";
const DEFAULT_OUTPUT_DIR: &str = "./dist";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify an option. Run rost_gen [-h | --help] for a list of options");
        return;
    }

    let option_arg = &args[1];

    if option_arg == "-v" || option_arg == "--version" {
        println!("{VERSION}");
    } else if option_arg == "-h" || option_arg == "--help" {
        print_help_message();
    } else if option_arg == "-i" || option_arg == "--input" {
        handle_conversion(&args);
    } else {
        println!("Invalid option. Run rost_gen [-h | --help] for a list of options");
    }
}

fn print_help_message() {
    println!("Usage: ./rost_gen[.exe] [OPTIONS]");
    println!("Options:");
    println!("\t-v, --version\t\t\tPrint tool name and version");
    println!("\t-h, --help\t\t\tPrint help message");
    println!("\t-i, --input [PATH]\n\t\tProvided a txt file path, will generate an html file");
    println!("\t\tProvided a directory path, will generate html files based on txt files in the directory");
    println!("\t\tWARNING: By default, will output to ./dist directory and will delete all existing contents if it already exists");
    println!("\n\t\tOptional: Use -o, --output [PATH] to output to a specific directory:");
    println!("\t\t\t-i, --input [INPUT_PATH] -o, --output [OUTPUT_PATH]");
    println!("\t\t This will not delete existing contents but will create the directory if it doesn't exist");
}

fn handle_conversion(args: &Vec<String>) {
    // Handle no input path or
    // input path with spaces which is not enclosed in quotes
    if args.len() < 3 || (args.len() > 3 && (args[3] != "-o" && args[3] != "--output")) {
        println!("Please provide a file or folder path. Enclose paths with spaces in quotes.");
        return;
    }

    let input_path = &args[2];
    let path = path::Path::new(input_path);

    if !path.exists() {
        println!("Invalid path: No file or directory found at '{input_path}'");
        return;
    }
    
    let mut output_dir_path = DEFAULT_OUTPUT_DIR.to_string();
    create_output_directory(args, &mut output_dir_path);

    if path.is_dir() {
        let dir = fs::read_dir(input_path).expect("Read input directory");
        convert_files_in_directory(dir, &output_dir_path);
    }

    if path.is_file() {
        if path.extension().unwrap().to_str().unwrap() == "txt" {
            convert_file(input_path, path, &output_dir_path);
        } else {
            println!("Only .txt files are accepted");
            return;
        }
    }
}

fn create_output_directory(args: &Vec<String>, output_dir_path: &mut String) {
    if args.len() == 4 && (args[3] == "-o"|| args[3] == "--output") {
        println!("WARNING: Output option specified but no output path specified. Defaulting to {DEFAULT_OUTPUT_DIR}");
    } else if args.len() == 5 {
        *output_dir_path = args[4].clone();
    } else if args.len() > 5 {
        println!("Invalid output path. Enclose output path in quotes if it contains spaces. Defaulting to {DEFAULT_OUTPUT_DIR}")
    }

    if output_dir_path == DEFAULT_OUTPUT_DIR && path::Path::new(DEFAULT_OUTPUT_DIR).exists() {
        fs::remove_dir_all(DEFAULT_OUTPUT_DIR).expect("Delete existing output directory")
    }
    fs::create_dir_all(output_dir_path).expect("Create output directory");
}

fn convert_files_in_directory(dir: fs::ReadDir, output_dir_path: &String) {
    for entry in dir {
        let path_string = &entry
            .expect("Read directory files")
            .path()
            .to_str()
            .unwrap()
            .to_string();
        let path = path::Path::new(path_string);
        convert_file(path_string, path, &output_dir_path);
    }
}

fn convert_file(path_string: &String, path: &path::Path, output_dir_path: &String) {
    if path.extension().unwrap().to_str().unwrap() != "txt" {
        return;
    }

    let in_file = fs::File::open(path_string).expect(&format!("Open file at {path_string}"));
    let mut buf_reader = io::BufReader::new(in_file);
    let html_file_name = path.file_stem().unwrap().to_str().unwrap();
    let mut read_buffer = String::new();
    let mut read_bytes: u64 = 0;
    let out_file_path = path::PathBuf::from(output_dir_path).join(format!("{html_file_name}.html"));
    let mut out_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(out_file_path)
        .expect("Generate html file");

    write!(
        out_file,
        "{}",
        HTML_TEMPLATE.replace("{{title}}", html_file_name)
    )
    .expect("Generate html file");

    while read_bytes < fs::metadata(path_string).expect("Read input file").len() {
        read_buffer.clear();
        read_bytes += buf_reader
            .read_line(&mut read_buffer)
            .expect("Read input file") as u64;
        if read_buffer.eq("\n") || read_buffer.eq("\r\n") {
            write!(out_file, "\t</p>\n\t<p>").expect("Generate html file");
        }
        write!(out_file, "\t\t{}", read_buffer.clone()).expect("Generate html file");
    }
    writeln!(out_file, "\n\t</p>\n</body>\n</html>").expect("Generate html file");
}
