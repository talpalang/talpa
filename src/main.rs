mod lib;

use lib::Parser;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // the .gpl is a temporary file extension (General Programming Language)
    // the example file should be updated with all working components
    let mut file = File::open("./src/example.gpl").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    // this removes \r as it seems to cause problems during parsing
    for i in 0..contents.len() {
        let c = contents.get(i);
        match c {
            Some(&13) => {contents.remove(i);},
            _ => ()
        }
    }
    match Parser::parse(contents) {
        Err(err) => println!("{}", err),
        Ok(res) => println!("{:?}", res.functions),
    }
}
