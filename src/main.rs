use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self};

mod lexer;
mod parser;
mod token;

fn main() -> Result<(), io::Error> {
    // get first args
    match std::env::args().nth(1) {
        Some(file_path) => {
            let mut _lines = match fs::read_to_string(file_path) {
                Ok(lines) => lines,
                Err(e) => return Err(e),
            };

            // add \n
            _lines = _lines + "\n";

            // lexer lines
            let lexer = lexer::lexer::Lexer::new(_lines.as_str());
            let mut p = parser::parser::Parser::new(lexer);

            let parsed_html = p.get_html();

            // print html
            println!("{}", parsed_html);

            println!("-! a.html saved in the same directory");

            let path = "a.html";
            let mut output = File::create(path)?;
            write!(output, "{}", parsed_html).unwrap();

            Ok(())
        }
        None => Err(io::Error::new(io::ErrorKind::Other, "no args!")),
    }
}
