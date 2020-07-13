mod compiler;

use compiler::{Compiler, Lang, Options};

fn main() {
    match Compiler::start(Options { lang: Lang::JS }) {
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
        Ok(_) => {}
    };
}
