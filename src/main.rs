use std::io::{self, BufRead, Write};
use std::{env, fs, path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify an option. Run rost_gen [-h | --help] for a list of options");
        return;
    }

    let option_arg = &args[1];

    if option_arg == "-v" || option_arg == "--version" {
        println!("rost_gen version 0.1");
    } else if option_arg == "-h" || option_arg == "--help" {
        print_help_message();
    } else if option_arg == "-i" || option_arg == "--input" {
        handle_conversion(&args);
    } else {
        println!("Invalid option. Run rost_gen [-h | --help] for a list of options");
    }
}

fn print_help_message() {
    println!("Usage: rost_gen [OPTIONS]");
    println!("Options:");
    println!("\t-v, --version\t\t\tPrint the tool name and version");
    println!("\t-h, --help\t\t\tPrint help message");
    println!("\t-i, --input [PATH]\n\t\tProvided a txt file path, will generate an html file");
    println!("\t\tProvided a directory path, will generate html files based on txt files in the directory");
    println!("\t\tWARNING: Will output to ./dist directory and will delete all existing contents if it already exists");
}

fn handle_conversion(args: &Vec<String>) {
    if args.len() > 3 {
        println!("Please provide a single file or folder path. Enclose paths with spaces in single quotes");
        return;
    }

    let input_path = &args[2];
    let path = path::Path::new(input_path);

    if !path.exists() {
        println!("Invalid path: No file or directory found at '{input_path}'");
        return;
    }

    if path::Path::new("./dist").exists() {
        fs::remove_dir_all("./dist").expect("Delete existing output directory")
    }
    fs::create_dir_all("./dist").expect("Create output directory");

    if path.is_dir() {
        let dir = fs::read_dir(input_path).expect("Read input directory");
        convert_files_in_directory(dir);
    }

    if path.is_file() {
        if path.extension().unwrap().to_str().unwrap() == "txt" {
            convert_file(input_path, path);
        } else {
            println!("Only .txt files are accepted");
            return;
        }
    }
}

fn convert_files_in_directory(dir: fs::ReadDir) {
    for entry in dir {
        let path_string = &entry
            .expect("Read directory files")
            .path()
            .to_str()
            .unwrap()
            .to_string();
        let path = path::Path::new(path_string);
        convert_file(path_string, path);
    }
}

fn convert_file(path_string: &String, path: &path::Path) {
    if path.extension().unwrap().to_str().unwrap() != "txt" {
        return;
    }

    let in_file = fs::File::open(path_string).expect(&format!("Open file at {path_string}"));
    let mut html_template =
        fs::read_to_string("./output_template.html").expect("Read template file");
    let mut buf_reader = io::BufReader::new(in_file);
    let html_file_name = path.file_stem().unwrap().to_str().unwrap();
    let mut read_buffer = String::new();
    let mut read_bytes: u64 = 0;
    let mut out_file = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(format!("./dist/{html_file_name}.html"))
        .expect("Generate html file");

    html_template = html_template.replace("{{title}}", html_file_name);
    write!(out_file, "{}", html_template).expect("Generate html file");

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
