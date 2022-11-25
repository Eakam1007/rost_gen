# rost_gen
A simple static site generator that generates basic .html files from .txt files.

# Getting started
This **project** requires that you have [Rust and Cargo](https://www.rust-lang.org/learn/get-started) installed on your device.
Run the following to install in your desired directory:

```
cargo install rost_gen --root <path_to_directory>
```

For example: 
```
cargo install rost_gen --root /my_dir
```
This will install the executable inside the ./my_dir/bin directory.
Run the project by changing into the install directory:  
``` 
cd /my_dir/bin 
```

# Usage
```
./rost_gen[.exe] [OPTION]
```
| Option  | Description |
| ------------- | ------------- |
| -v, --version  | Print tool name and version  |
| -h, --help  | Print help message with a list of options  |
| -i, --input [PATH] | Provided a path to a text(.txt) or Markdown (.md) file, generate an html file <br> Provided a path to a directory, generate html files for all text(.txt) and Markdown (.md) files in that directory<br><strong>Warning: will output generated html files to the ./dist directory, replacing any existing content</strong>|
| -o, --output [PATH] | Optional: Use to specify an output directory:<br> ``-i, --input [INPUT_PATH] -o, --output  [OUTPUT_PATH]``<br><br>This will not delete any existing content in the specified directory. If the directory doesn't exist, it will be created|
| -l, --lang [LANG] | Optional: Use to specify the language (lang attribute of the html tag) of html file. Defaults to "en-CA" |
|-c, --config [PATH]| Flags accept a file path to a JSON config file.|

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
  
# Markdown (.md) File Features
- ### Header detection
  ```
  # This line is header
  ```
  will be converted to
  ```
  <h1>
    This line is header
  </h1>
  ```
  
- ### Thematic Break detection (horizontal rule)
  ```
  ---
  ```
  will be converted into
  ```html
  <hr />
  ```
  This is only supported for lines that just have the above markdown text
  
- ### Link Markdown
  ```
  [This is a link](https://www.example.com)
  ```
  will be converted to 
  ```html
  <a href="https://www.example.com">This is a link</a>
  ```
  Any text before or after the markdown in the same line will be preserved as is

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
- ### Specify language for generated html:
  ```
  ./rost_gen -i ./file_to_convert.txt -l fr
  ```
  This will update the lang tag of the html tag in the generated html file:
  ```html
  <html lang="fr">
  ...
  ```

  ## Running config JSON files
  ```
  ./rost_gen -c ./ssg-config.json

  ```

