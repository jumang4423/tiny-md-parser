use std::fs;
use std::io::{self};

mod lexer;
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
            let mut lexer = lexer::lexer::Lexer::new(_lines.as_str());
            let mut tok = lexer.next_token();

            loop {
                match &tok {
                    token::token::Token::Eof => break,
                    _ => println!("{:?}", tok),
                }

                tok = lexer.next_token();
            }

            Ok(())
        }
        None => Err(io::Error::new(io::ErrorKind::Other, "no args!")),
    }
}
