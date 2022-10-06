use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, Write};
use std::{fs, path};

extern crate serde;
extern crate serde_json;
extern crate serde_with;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
struct Config{
    input: Option<String>,
    output: Option<String>,
    lang: Option<String>
}

const HTML_TEMPLATE: &str = "<!DOCTYPE html>\n<html lang=\"{{lang}}\">\n<head>\n\t<meta charset=\"UTF-8\">\n\t<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\t<title>\n\t\t{{title}}\n\t</title>\n</head>\n<body>\n";
const DEFAULT_OUTPUT_DIR: &str = "./dist";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Take config written in JSON file as argument to parse into system 
    #[arg(short, long, value_name = "CONFIG_PATH")]
    config: Option <String>,

    /// Convert file/files in directory at INPUT_PATH into html files, outputting into ./dist directory by default (deleting existing contents)
    #[arg(short, long, value_name = "INPUT_PATH")]
    input: Option<String>,

    /// Optional: Output generated files to directory at OUTPUT_PATH
    #[arg(short, long, value_name = "OUTPUT_PATH", default_value = DEFAULT_OUTPUT_DIR)]
    output: String,

    /// Optional: Specify lang attribute of html tag
    #[arg(short, long, value_name = "LANG", default_value = "en-CA")]
    lang: String,
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();
    if let Some (input) = args.input.as_deref() {
        handle_conversion(input, &args.output, &args.lang);
    } else if let Some (config) = args.config.as_deref() {
        handle_config(config);
    }
}

fn handle_config(config: &str) {
    let config_path = config.to_string();
    let path = path::Path::new(&config_path);

    if !path.exists() {
        println!("Invalid path: No file or directory found at '{config_path}'");
        return;
    }

    if path.is_file() {
        if path.extension().unwrap().to_str().unwrap() == "json" {
            println!("reading json file at  '{config_path}'");
            let file = File::open(config_path).unwrap();
            let reader = BufReader::new(file);

            let construct: Config = serde_json::from_reader(reader).unwrap();
            let dept_input = construct.input.unwrap_or(" ".to_string());
            let dept_output = construct.output.unwrap_or(DEFAULT_OUTPUT_DIR.to_string());
            let dept_lang = construct.lang.unwrap_or("en-CA".to_string());

            handle_conversion(&dept_input, &dept_output, &dept_lang)

        }
    } else {
        println!("Only .json files are accepted");
        return;
    }
}

fn handle_conversion(input: &str, output_dir_path: &String, html_lang: &String) {
    let input_path = input.to_string();
    let path = path::Path::new(&input_path);

    if !path.exists() {
        println!("Invalid path: No file or directory found at '{input_path}'");
        return;
    }

    create_output_directory(output_dir_path);

    if path.is_dir() {
        let dir = fs::read_dir(&input_path).expect("Read input directory");
        println!("Converting files in directory at {input_path}");
        convert_files_in_directory(dir, output_dir_path, html_lang);
    }

    if path.is_file() {
        if path.extension().unwrap().to_str().unwrap() == "txt"
            || path.extension().unwrap().to_str().unwrap() == "md"
        {
            convert_file(&input_path, path, output_dir_path, html_lang);
        } else {
            println!("Only .txt or .md files are accepted");
            return;
        }
    }

    println!("Conversion successful. Output file(s) placed in directory at {output_dir_path}");
}

fn create_output_directory(output_dir_path: &String) {
    // Delete output dir and its contents if it is the default output dir
    if output_dir_path == DEFAULT_OUTPUT_DIR && path::Path::new(DEFAULT_OUTPUT_DIR).exists() {
        fs::remove_dir_all(DEFAULT_OUTPUT_DIR).expect("Delete existing output directory")
    }
    fs::create_dir_all(output_dir_path).expect("Create output directory");
}

fn convert_files_in_directory(dir: fs::ReadDir, output_dir_path: &String, html_lang: &String) {
    // Iterate over each file in directory, calling the convert file function
    for entry in dir {
        let path_string = &entry
            .expect("Read directory files")
            .path()
            .to_str()
            .unwrap()
            .to_string();
        let path = path::Path::new(path_string);
        convert_file(path_string, path, &output_dir_path, html_lang);
    }
}

fn convert_file(
    path_string: &String,
    path: &path::Path,
    output_dir_path: &String,
    html_lang: &String,
) {
    // We only want to convert .txt files
    if path.extension().unwrap().to_str().unwrap() != "txt"
        && path.extension().unwrap().to_str().unwrap() != "md"
    {
        return;
    }

    println!("Converting file at {path_string}");

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

    // Replace lang and title in the template with appropriate values
    // If title was not found, file name will be used instead
    let html_template = HTML_TEMPLATE.replace("{{lang}}", html_lang).replace(
        "{{title}}",
        if title.is_empty() {
            html_file_name
        } else {
            title.as_str()
        },
    );

    // Write the html template
    write!(out_file, "{}", html_template).expect("Generate html file");

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
        if path.extension().unwrap().to_str().unwrap() == "txt" {
            if read_buffer == "\n" || read_buffer == "\r\n" {
                write!(out_file, "\t</p>\n\t<p>\n").expect("Generate html file");
            } else {
                write!(out_file, "\t\t{}", read_buffer.clone()).expect("Generate html file");
            }
        }
        if path.extension().unwrap().to_str().unwrap() == "md" {
            if read_buffer == "\n" || read_buffer == "\r\n" {
                write!(out_file, "\t</p>\n\t<p>").expect("Generate html file");
            }

            if read_buffer.starts_with("# ") {
                read_buffer.remove(0);
                read_buffer.remove(0);
                write!(out_file, "<h1>\n{read_buffer}</h1>\n").expect("Generate html file");
            } else if read_buffer == "---\n" || read_buffer == "---\r\n" {
                write!(out_file, "<hr />\n").expect("Generate html file");
            } else {
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
