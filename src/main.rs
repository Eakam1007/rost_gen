use std::io::{self, BufRead, Seek, Write};
use std::{env, fs, path};

const HTML_TEMPLATE: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\t<meta charset=\"UTF-8\">\n\t<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\t<title>\n\t\t{{title}}\n\t</title>\n</head>\n<body>\n";
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
    println!("\t-i, --input [PATH]\n\t\tProvided a txt or md file path, will generate an html file");
    println!("\t\tProvided a directory path, will generate html files based on txt or md files in the directory");
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
        if path.extension().unwrap().to_str().unwrap() == "txt" ||  path.extension().unwrap().to_str().unwrap() == "md" {
            convert_file(input_path, path, &output_dir_path);
        } else {
            println!("Only .txt or .md files are accepted");
            return;
        }
    }
}

fn create_output_directory(args: &Vec<String>, output_dir_path: &mut String) {
    // Warn if -o or --output flag was specified by a path was not
    // Warn if more than one output path was specified or if output path has spaces
    if args.len() == 4 && (args[3] == "-o" || args[3] == "--output") {
        println!("WARNING: Output option specified but no output path specified. Defaulting to {DEFAULT_OUTPUT_DIR}");
    } else if args.len() > 5 {
        println!("WARNING: Invalid output path. Enclose output path in quotes if it contains spaces. Defaulting to {DEFAULT_OUTPUT_DIR}")
    } else if args.len() == 5 {
        *output_dir_path = args[4].clone();
    }

    // Delete output dir and its contents if it is the default output dir
    if output_dir_path == DEFAULT_OUTPUT_DIR && path::Path::new(DEFAULT_OUTPUT_DIR).exists() {
        fs::remove_dir_all(DEFAULT_OUTPUT_DIR).expect("Delete existing output directory")
    }
    fs::create_dir_all(output_dir_path).expect("Create output directory");
}

fn convert_files_in_directory(dir: fs::ReadDir, output_dir_path: &String) {
    // Iterate over each file in directory, calling the convert file function
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
    // We only want to convert .txt files
    if path.extension().unwrap().to_str().unwrap() != "txt" && path.extension().unwrap().to_str().unwrap() != "md" {
        return;
    }

    // Variables to read input file
    let in_file = fs::File::open(path_string).expect(&format!("Open file at {path_string}"));
    let mut buf_reader = io::BufReader::new(in_file);
    let mut read_buffer = String::new();

    // Try to find title
    // Title is first line followed by two blank lines
    let mut title = String::new();
    let mut read_bytes: u64 = parse_title_from_file(path_string, &mut title);

    // Variables to create and write to output html file
    let html_file_name = path.file_stem().unwrap().to_str().unwrap();
    let out_file_path = path::PathBuf::from(output_dir_path).join(format!("{html_file_name}.html"));
    let mut out_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(out_file_path)
        .expect("Generate html file");

    // Write the html template, replacing title with the found title or the file name
    write!(
        out_file,
        "{}",
        HTML_TEMPLATE.replace(
            "{{title}}",
            if title.is_empty() {
                html_file_name
            } else {
                title.as_str()
            }
        )
    )
    .expect("Generate html file");

    // Write the title if found
    if title.is_empty() {
        write!(out_file, "\t<p>\n").expect("Generate html file");
    } else {
        write!(out_file, "\t<h1>\n\t\t{title}\t</h1>\n\t<p>\n").expect("Generate html file");
        // Skip the title bytes (first three lines) to prevent printing title twice
        buf_reader
            .seek(io::SeekFrom::Start(read_bytes))
            .expect("Read input file");
    }

    // Write the rest of the contents
    while read_bytes < fs::metadata(path_string).expect("Read input file").len() {
        read_buffer.clear();
        read_bytes += buf_reader
            .read_line(&mut read_buffer)
            .expect("Read input file") as u64;
    
        // Add paragraph tags if line is an empty line
        // Empty line indicate end of current paragraph and start of next paragraph
        if path.extension().unwrap().to_str().unwrap() == "txt"{
        if read_buffer == "\n" || read_buffer == "\r\n" {
            write!(out_file, "\t</p>\n\t<p>").expect("Generate html file");
        }else{
            write!(out_file, "\t\t{}", read_buffer.clone()).expect("Generate html file");
        }
        }
        if path.extension().unwrap().to_str().unwrap() == "md"{
        if read_buffer.starts_with("# ") {
            read_buffer.remove(0);
            read_buffer.remove(0);
            write!(out_file, "<h1>\n{read_buffer}</h1>\n").expect("Generate html file");

        }else{
        write!(out_file, "\t\t{}", read_buffer.clone()).expect("Generate html file");
    }

}
        
    
    }

    // Write closing tags for html file
    writeln!(out_file, "\n\t</p>\n</body>\n</html>").expect("Generate html file");
}

// returns no of bytes to skip if title is found, otherwise 0
fn parse_title_from_file(path_string: &String, title: &mut String) -> u64 {
    let mut buf_reader = io::BufReader::new(
        fs::File::open(path_string).expect(&format!("Open file at {path_string}")),
    );
    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    let lines = [&mut line1, &mut line2, &mut line3];
    let mut read_bytes: u64 = 0;

    for line in lines {
        match buf_reader.read_line(line) {
            Ok(n) => read_bytes += n as u64,
            Err(_) => return 0,
        }
    }

    // Check if line 1 is not empty and not a blank line, and is followed by two empty lines
    if (!line1.is_empty() && line1 != "\n" && line1 != "\r\n")
        && (line2 == "\n" || line2 == "\r\n")
        && (line3 == "\n" || line3 == "\r\n")
    {
        *title = line1;
        return read_bytes;
    } else {
        return 0;
    }
}
