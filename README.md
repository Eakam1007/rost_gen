# rost_gen
A simple static site generator that generates basic .html files from .txt files.

# Getting started
This project requires that you have [Rust and Cargo](https://www.rust-lang.org/learn/get-started) installed on your device.  

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
| -i, --input \[PATH\] | Provided a path to a text(.txt) file, generate an html file <br> Provided a path to a directory, generate html files for all text(.txt) files in that directory<br> <strong>Warning: Will output generated html files to the ./dist directory, replacing any existing content</strong> |
