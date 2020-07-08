mod lib;

use lib::Parser;
use lib::JSGenerator;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./src/example.tp").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    let parser = Parser::parse(contents);
    match parser {
        Err(err) => println!("{}", err),
        Ok(res) => {
            println!("Functions: {:?}", res.functions);
            println!("Globals: {:?}", res.global_vars);
            // For now, we generare JS code to test that everything works
            match JSGenerator::generate(res) {
                Err(err) => println!("{}", err),
                Ok(res) => {
                    println!("Javascript:");
                    println!("{}", res.src);
                }
            }
        }
    }
}
