mod lib;

use lib::Parser;
use lib::languages::generate;
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
            println!("{:#?}", res);
            // Stage 2 goes here
            // Stage 3: Generate code
            // use javascript for now (change to arg in future)
            let code = generate(res, "javascript");
            match code {
                Err(err) => println!("{:?}", err),
                Ok(res) => {
                    println!("{}", res);
                }
            }
        },
    }
}
