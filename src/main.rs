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
    let res = match parser {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(res) => res,
    };
    println!("{:#?}", res);
    let code = generate(res, "javascript");
    let src = match code {
        Err(err) => {println!("{:?}", err);"".to_string()},
        Ok(res) => res
    };
    println!("{}", src);
}
