# rost_gen
A simple static site generator that generates basic .html files from .txt files.

# Getting started
This **project** requires that you have [Rust and Cargo](https://www.rust-lang.org/learn/get-started) installed on your device.  

```
git clone git@github.com:Eakam1007/rost_gen.git
cd rost_gen
cargo build --release
```
This will create the optimized build in the ./target/release directory. Run the project by changing into the release directory:  
``` cd target/release ```

# Usage
```
./rost_gen[.exe] [OPTION]
```
| Option  | Description |
| ------------- | ------------- |
| -v, --version  | Print tool name and version  |
| -h, --help  | Print help message with a list of options  |
| -i, --input [PATH] | Provided a path to a text(.txt) or Markdown (.md) file, generate an html file <br> Provided a path to a directory, generate html files for all text(.txt) and Markdown (.md) files in that directory<br><strong>Warning: will output generated html files to the ./dist directory, replacing any existing content</strong> <br><br> Optionally use with the -o, --output flag to specify an output directory:<br> ``-i, --input [INPUT_PATH] -o, --output  [OUTPUT_PATH]``<br><br>This will not delete any existing content in the specified directory. If the directory doesn't exist, it will be created|

# Features
- ### Specify a title in text and Markdown files  
  Specify a title in the text and Markdown files by leaving two blank lines after the first line:
  ``` 
  This is a title 
  
  
  First line of text
  ```
  The first line will then be used as the title for generated html file, and the generated html body will include a ``<h1>`` tag with the title:
  ```
  <body>
    <h1>
      This is a title
    </h1>
    <p>
      First line of text
    </p>
  ```
- ### Seperate paragraphs with blank lines
  Excluding the title, specify the end of a paragraph and the start of a new paragraph by seperating them with blank lines:
  ```
  This is a title
  
  
  First line of first paragraph
  
  First line of second paragraph
  Second line of second paragraph
  ```
  This will result in two ``<p>`` tags in the generated html:
  ```
  <h1>
    This is a title
  </h1>
  <p>
    First line of first paragraph
  </p>
  <p>
    First line of second paragraph
    Second line of second paragraph
  </p>
  ```
- ### Header detection for .md files  
  ```
  # This line is header
  ```
  will be converted to
  ```
  <h1>
    This line is header
  </h1>
  ```

# Examples
- ### One input file
  To convert the "file_to_convert.txt" in current directory:
  ```
  ./rost_gen --input ./file_to_convert.txt
  ```
  or to convert "file_to_convert.md" in current directory:
    ```
  ./rost_gen --input ./file_to_convert.md
  ```
- ### One or multiple input files in a directory
  To convert all .txt files in the "folder_with_input_files directory" in the current directory:
  ```
  ./rost_gen -i ./folder_with_input_files
  ```
- ### Specify an output directory:
  Output to the "custom_output_dir" in the current directory:
  ```
  ./rost_gen -i ./file_to_convert.txt -o ./custom_output_dir
  ```
