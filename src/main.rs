use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, Write};
use std::{fs, path};
#[derive(Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
struct Config {
  input: Option<String>,
  output: Option<String>,
  lang: Option<String>,
}

const HTML_TEMPLATE: &str = "<!DOCTYPE html>\n<html lang=\"{{lang}}\">\n<head>\n\t<meta charset=\"UTF-8\">\n\t<meta \
                             http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n\t<meta name=\"viewport\" \
                             content=\"width=device-width, \
                             initial-scale=1.0\">\n\t<title>\n\t\t{{title}}\n\t</title>\n</head>\n<body>\n";
const DEFAULT_OUTPUT_DIR: &str = "./dist";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Take config written in JSON file as argument to parse into system
  #[arg(short, long, value_name = "CONFIG_PATH")]
  config: Option<String>,

  /// Convert file/files in directory at INPUT_PATH into html files, outputting
  /// into ./dist directory by default (deleting existing contents)
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

  if let Some(input) = args.input.as_deref() {
    handle_conversion(input, &args.output, &args.lang);
  } else if let Some(config) = args.config.as_deref() {
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

  if path.is_file() && path.extension().unwrap().to_str().unwrap() == "json" {
    println!("reading json file at  '{config_path}'");
    let file = File::open(config_path).unwrap();
    let reader = BufReader::new(file);

    let construct: Config = serde_json::from_reader(reader).unwrap();
    let dept_input = construct.input.unwrap_or_else(|| " ".to_string());
    let dept_output = construct.output.unwrap_or_else(|| DEFAULT_OUTPUT_DIR.to_string());
    let dept_lang = construct.lang.unwrap_or_else(|| "en-CA".to_string());

    handle_conversion(&dept_input, &dept_output, &dept_lang)
  } else {
    println!("Only .json files are accepted");
  }
}

fn handle_conversion(input: &str, output_dir_path: &String, html_lang: &str) {
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

  if path.is_file() && conversion_file_path_valid(path) {
    convert_file(&input_path, path, output_dir_path, html_lang);
  } else {
    println!("Only .txt or .md files are accepted");
    return;
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

fn convert_files_in_directory(dir: fs::ReadDir, output_dir_path: &String, html_lang: &str) {
  // Iterate over each file in directory, calling the convert file function
  for entry in dir {
    let path_string = &entry
      .expect("Read directory files")
      .path()
      .to_str()
      .unwrap()
      .to_string();
    let path = path::Path::new(path_string);
    convert_file(path_string, path, output_dir_path, html_lang);
  }
}

fn convert_file(path_string: &String, path: &path::Path, output_dir_path: &String, html_lang: &str) {
  // We only want to convert .txt files
  if !conversion_file_path_valid(path) {
    return;
  }

  println!("Converting file at {path_string}");

  // Variables to read input file
  let in_file = fs::File::open(path_string).unwrap_or_else(|_| panic!("Open file at {path_string}"));
  let mut buf_reader = io::BufReader::new(in_file);
  let mut read_buffer = String::new();

  // Try to find title
  // Title is first line followed by two blank lines
  let mut title = String::new();
  let mut read_bytes_count: u64 = parse_title_from_file(path_string, &mut title);

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
    writeln!(out_file, "\t<p>").expect("Generate html file");
  } else {
    write!(out_file, "\t<h1>\n\t\t{title}\t</h1>\n\t<p>\n").expect("Generate html file");
    // Skip the title bytes (first three lines) to prevent printing title twice
    buf_reader
      .seek(io::SeekFrom::Start(read_bytes_count))
      .expect("Read input file");
  }

  // Write the rest of the contents
  while read_bytes_count < fs::metadata(path_string).expect("Read input file").len() {
    read_buffer.clear();
    read_bytes_count += buf_reader.read_line(&mut read_buffer).expect("Read input file") as u64;
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

      if let Some(content) = read_buffer.strip_prefix("# ") {
        write!(out_file, "<h1>\n{}</h1>\n", content).expect("Generate html file");
      } else if read_buffer == "---\n" || read_buffer == "---\r\n" {
        writeln!(out_file, "<hr />\n").expect("Generate html file");
      } else {
        let processed_line = process_link_markdown(&read_buffer);
        write!(out_file, "\t\t{}", processed_line.clone()).expect("Generate html file");
      }
    }
  }

  // Write closing tags for html file
  writeln!(out_file, "\n\t</p>\n</body>\n</html>").expect("Generate html file");
}

// returns no of bytes to skip if title is found, otherwise 0
fn parse_title_from_file(path_string: &String, title: &mut String) -> u64 {
  let mut buf_reader =
    io::BufReader::new(fs::File::open(path_string).unwrap_or_else(|_| panic!("Open file at {path_string}")));
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

  // Check if line 1 is not empty and not a blank line, and is followed by two
  // empty lines
  if (!line1.is_empty() && line1 != "\n" && line1 != "\r\n")
    && (line2 == "\n" || line2 == "\r\n")
    && (line3 == "\n" || line3 == "\r\n")
  {
    *title = line1;
    read_bytes
  } else {
    0
  }
}

fn conversion_file_path_valid(path: &path::Path) -> bool {
  let extension = path.extension().unwrap().to_str().unwrap();

  if extension == "txt" || extension == "md" {
    return true;
  }

  false
}

fn process_link_markdown(line: &String) -> String {
  const LINK_HTML_TEMPLATE: &str = "<a href=\"URL\">TEXT</a>";
  let line_bytes = line.as_bytes();
  let mut link_start_found = false;
  let mut link_end_found = false;
  let mut link_url_start_found = false;
  let mut link_url_end_found = false;
  let mut link_start = 0;
  let mut link_end = 0;
  let mut link_url_start = 0;
  let mut link_url_end = 0;

  for (i, &char) in line_bytes.iter().enumerate() {
    if !link_start_found && char == b'[' {
      link_start_found = true;
      link_start = i + 1;
    }

    if link_start_found && !link_end_found && char == b']' {
      link_end_found = true;
      link_end = i;
    }

    if link_end_found {
      if !link_url_start_found && char == b'(' {
        link_url_start_found = true;
        link_url_start = i + 1;
      }

      if link_url_start_found && !link_url_end_found && char == b')' {
        link_url_end_found = true;
        link_url_end = i;
      }
    }
  }

  if link_start_found && link_end_found {
    let link_text = &line[link_start..link_end];
    let mut link_url = "";
    let mut link_html = LINK_HTML_TEMPLATE.replace("TEXT", link_text);

    if link_url_start_found && link_url_end_found {
      link_url = &line[link_url_start..link_url_end];
    }

    link_html = link_html.replace("URL", link_url);

    if link_start > 1 {
      link_html = format!("{}{link_html}", &line[0..(link_start - 1)]);
    }

    if (link_url_end + 1) < line_bytes.len() {
      link_html = format!("{link_html}{}", &line[(link_url_end + 1)..]);
    }

    return link_html;
  }

  line.clone()
}

#[cfg(test)]
mod tests {
  use std::io::Read;

  use crate::*;

  #[test]
  fn converts_txt_files() {
    let input_file_path = path::Path::new("sample.txt");
    assert!(conversion_file_path_valid(input_file_path));
  }

  #[test]
  fn converts_md_files() {
    let input_file_path = path::Path::new("sample.md");
    assert!(conversion_file_path_valid(input_file_path));
  }

  #[test]
  fn does_not_convert_unsupported_file_types() {
    let input_file_path = path::Path::new("sample.exe");
    assert!(!conversion_file_path_valid(input_file_path));
  }

  #[test]
  fn parses_title_when_provided() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_input_path = temp_dir.path().join("parse_title_test.txt");
    let test_input_path_string = test_input_path.as_os_str().to_str().unwrap().to_string();
    let mut test_input_file = File::create(&test_input_path).unwrap();
    writeln!(test_input_file, "test\n\n").expect("Create test input file");
    let mut output_title = String::new();

    parse_title_from_file(&test_input_path_string, &mut output_title);

    assert_eq!(output_title, "test\n");

    drop(test_input_file);
    temp_dir.close().expect("Delete test directory");
  }

  #[test]
  fn returns_title_size_when_found() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_input_path = temp_dir.path().join("parse_title_test.txt");
    let test_input_path_string = test_input_path.as_os_str().to_str().unwrap().to_string();
    let mut test_input_file = File::create(&test_input_path).unwrap();
    writeln!(test_input_file, "test\n\n").expect("Create test input file");
    let mut output_title = String::new();

    let bytes_read = parse_title_from_file(&test_input_path_string, &mut output_title);

    assert_eq!(bytes_read, 7);

    drop(test_input_file);
    temp_dir.close().expect("Delete test directory");
  }

  #[test]
  fn does_not_change_title_arg_when_no_title() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_input_path = temp_dir.path().join("parse_title_test.txt");
    let test_input_path_string = test_input_path.as_os_str().to_str().unwrap().to_string();
    let mut test_input_file = File::create(&test_input_path).unwrap();
    writeln!(test_input_file, "test\n").expect("Create test input file");
    let mut output_title = String::new();

    parse_title_from_file(&test_input_path_string, &mut output_title);

    assert_eq!(output_title, "");

    drop(test_input_file);
    temp_dir.close().expect("Delete test directory");
  }

  #[test]
  fn returns_zero_when_no_title() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_input_path = temp_dir.path().join("parse_title_test.txt");
    let test_input_path_string = test_input_path.as_os_str().to_str().unwrap().to_string();
    let mut test_input_file = File::create(&test_input_path).unwrap();
    writeln!(test_input_file, "test\n").expect("Create test input file");
    let mut output_title = String::new();

    let bytes_read = parse_title_from_file(&test_input_path_string, &mut output_title);

    assert_eq!(bytes_read, 0);

    drop(test_input_file);
    temp_dir.close().expect("Delete test directory");
  }

  #[test]
  fn processes_one_markdown_link() {
    let input_line = String::from("[This is text for a link](www.example.com)");
    let expected_output = "<a href=\"www.example.com\">This is text for a link</a>";
    assert_eq!(process_link_markdown(&input_line), expected_output);
  }

  #[test]
  fn retains_text_before_link() {
    let input_line = String::from("Lorem Ipsum[This is text for a link](www.example.com)");
    let expected_output = "Lorem Ipsum<a href=\"www.example.com\">This is text for a link</a>";
    assert_eq!(process_link_markdown(&input_line), expected_output);
  }

  #[test]
  fn retains_text_after_link() {
    let input_line = String::from("[This is text for a link](www.example.com)Lorem Ipsum");
    let expected_output = "<a href=\"www.example.com\">This is text for a link</a>Lorem Ipsum";
    assert_eq!(process_link_markdown(&input_line), expected_output);
  }

  #[test]
  fn retains_text_around_link() {
    let input_line = String::from("Lorem Ipsum[This is text for a link](www.example.com)Dolor Sit");
    let expected_output = "Lorem Ipsum<a href=\"www.example.com\">This is text for a link</a>Dolor Sit";
    assert_eq!(process_link_markdown(&input_line), expected_output);
  }

  #[test]
  fn does_not_process_invalid_link_markdown() {
    let input_line = String::from("[Invalid markdown[(www.example.com)");
    let expected_output = "[Invalid markdown[(www.example.com)";
    assert_eq!(process_link_markdown(&input_line), expected_output);
  }

  #[test]
  fn image_link_test() {
    let input_line = String::from("[First][Second](www.example.com)");
    let expected_output = "<a href=\"www.example.com\">First</a>";
    assert_eq!(process_link_markdown(&input_line), expected_output);
  }

  #[test]
  fn process_link_markdown_returns_empty_string_arg() {
    assert_eq!(process_link_markdown(&String::from("")), "");
  }

  #[test]
  fn creates_output_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir_path = temp_dir.path().join("test_dir");
    let output_dir_path_string = output_dir_path.as_os_str().to_str().unwrap().to_string();

    create_output_directory(&output_dir_path_string);
    assert!(output_dir_path.is_dir());

    temp_dir.close().expect("Delete test directory");
  }

  #[test]
  fn recreates_default_output_directory() {
    let output_dir_path = path::Path::new(&DEFAULT_OUTPUT_DIR);
    fs::create_dir_all(output_dir_path).expect("Create test default output dir");
    File::create(output_dir_path.join("test_file.txt")).expect("Create test file");

    create_output_directory(&DEFAULT_OUTPUT_DIR.to_string());

    assert!(output_dir_path.is_dir());
    assert!(output_dir_path.read_dir().unwrap().next().is_none());

    fs::remove_dir_all(output_dir_path).expect("Delete test directory");
  }

  #[test]
  fn retains_non_default_output_directory_contents() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir_path = temp_dir.path().join("test_dir");
    let output_dir_path_string = output_dir_path.as_os_str().to_str().unwrap().to_string();
    let existing_file_path = output_dir_path.join("test_file.txt");
    fs::create_dir(&output_dir_path).expect("Create test output dir");
    File::create(&existing_file_path).expect("Create test file");

    create_output_directory(&output_dir_path_string);

    assert!(output_dir_path.is_dir());
    assert_eq!(
      output_dir_path.read_dir().unwrap().next().unwrap().expect("").path(),
      existing_file_path
    )
  }

  #[test]
  fn creates_html_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_input_path = temp_dir.path().join("html_template_test.txt");
    let test_input_path_string = test_input_path.as_os_str().to_str().unwrap().to_string();
    let mut test_input_file = File::create(&test_input_path).unwrap();
    writeln!(test_input_file, "test").expect("Create test input file");

    convert_file(
      &test_input_path_string,
      test_input_path.as_path(),
      &temp_dir.path().to_path_buf().into_os_string().into_string().unwrap(),
      "en",
    );

    let expected_output = HTML_TEMPLATE
      .replace("{{title}}", "html_template_test")
      .replace("{{lang}}", "en")
      + "\t<p>\n\t\ttest\n\n\t</p>\n</body>\n</html>\n";
    let test_output_file_path = temp_dir.path().join("html_template_test.html");
    let mut test_output_file = File::open(&test_output_file_path).unwrap();
    let mut converted_string = String::new();
    test_output_file
      .read_to_string(&mut converted_string)
      .expect("Read test output file");

    assert_eq!(converted_string, expected_output);

    drop(test_input_file);
    temp_dir.close().expect("Delete test directory");
  }

  #[test]
  fn creates_html_in_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_input_path1 = temp_dir.path().join("html_template_test1.txt");
    let test_input_path2 = temp_dir.path().join("html_template_test2.txt");
    let mut test_input_file1 = File::create(&test_input_path1).unwrap();
    let mut test_input_file2 = File::create(&test_input_path2).unwrap();
    writeln!(test_input_file1, "test1").expect("Create test input file");
    writeln!(test_input_file2, "test2").expect("Create test input file");

    let out_dir = "./out".to_owned();
    fs::create_dir_all(&out_dir).expect("Create test output directory");

    let input_dir = fs::read_dir(&temp_dir).expect("Read input directory");
    convert_files_in_directory(input_dir, &out_dir, "en");

    let expected_output1 = HTML_TEMPLATE
      .replace("{{title}}", "html_template_test1")
      .replace("{{lang}}", "en")
      + "\t<p>\n\t\ttest1\n\n\t</p>\n</body>\n</html>\n";
    let expected_output2 = HTML_TEMPLATE
      .replace("{{title}}", "html_template_test2")
      .replace("{{lang}}", "en")
      + "\t<p>\n\t\ttest2\n\n\t</p>\n</body>\n</html>\n";

    let output_file1 = "./out/html_template_test1.html";
    let output_file2 = "./out/html_template_test2.html";
    let mut test_output_file1 = File::open(output_file1).unwrap();
    let mut test_output_file2 = File::open(output_file2).unwrap();
    let mut converted_string1 = String::new();
    let mut converted_string2 = String::new();
    test_output_file1
      .read_to_string(&mut converted_string1)
      .expect("Read test output file");
    test_output_file2
      .read_to_string(&mut converted_string2)
      .expect("Read test output file");

    assert_eq!(converted_string1, expected_output1);
    assert_eq!(converted_string2, expected_output2);

    fs::remove_dir_all(&out_dir).expect("Remove output directory");
    drop(test_input_file1);
    drop(test_input_file2);
    temp_dir.close().expect("Delete test directory");
  }
}
