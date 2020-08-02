mod compiler;

use compiler::{AnilizedTokens, Compiler, CompilerProps, Lang, LocationError, Options};
use std::cell::RefCell;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Clone)]
struct CLI {
    warnings: usize,
    errors: usize,
    options: Options,
}

impl Deref for CLI {
    type Target = CLI;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl DerefMut for CLI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl CLI {
    fn new(options: Options) -> Self {
        Self {
            options,
            warnings: 0,
            errors: 0,
        }
    }
}

impl CompilerProps for CLI {
    fn open_file(&mut self, file_name: &str) -> Result<Vec<u8>, String> {
        match fs::read(file_name) {
            Err(err) => Err(format!("{}", err)),
            Ok(c) => Ok(c),
        }
    }
    fn get_options(&self) -> Options {
        self.options.clone()
    }
    fn warning(&mut self, error: LocationError) {
        self.warnings += 1;
        println!("Error:\n{:?}", error);
    }
    fn error(&mut self, warning: LocationError) {
        self.errors += 1;
        println!("Warning:\n{:?}", warning);
    }
    fn debug_formatted_tokens(&mut self, _: String, tokens: AnilizedTokens) {
        println!("Debug output:");
        println!("{:#?}", tokens);
    }
    fn debug_parsed_output(&mut self, _: String, src: String) {
        println!("Parse output:");
        println!("{}", src);
    }
}

fn main() {
    let cli = Rc::new(RefCell::new(CLI::new(Options {
        lang: Some(Lang::Go),
        debug: true,
    })));
    let cli_clone = Rc::clone(&cli);
    Compiler::start("example.tp", cli);

    let errors = cli_clone.borrow().errors;
    if errors > 0 {
        println!("Unable to compile file, {} errors occurred", errors);
        std::process::exit(1);
    }

    let warnings = cli_clone.borrow().warnings;
    if warnings == 0 {
        println!("Successfully compiled code");
    } else {
        println!("Successfully compiled code with {} warnings", warnings);
    }
}
