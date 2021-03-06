use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self};
mod lib;

fn main() -> Result<(), io::Error> {
    // get first args
    match std::env::args().nth(1) {
        Some(file_path) => {
            let mut _lines = match fs::read_to_string(file_path) {
                Ok(lines) => lines,
                Err(e) => return Err(e),
            };
            _lines = _lines + "\n";
            // render md
            let html = lib::render_md(_lines);

            // print html
            println!("{}", html);
            println!("-! a.html saved in the same directory");
            let path = "a.html";
            let mut output = File::create(path)?;
            write!(output, "{}", html).unwrap();

            Ok(())
        }
        None => Err(io::Error::new(io::ErrorKind::Other, "no args!")),
    }
}
