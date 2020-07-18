mod compiler;

use compiler::{
    AnilizedTokens, CodeLocation, Compiler, CompilerProps, Lang, LocationError, Options,
};

struct CLI {
    warnings: usize,
    errors: usize,
    options: Options,
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
    fn debug_formatted_tokens(&mut self, _: CodeLocation, tokens: AnilizedTokens) {
        println!("Debug output:");
        println!("{:#?}", tokens);
    }
    fn debug_parsed_output(&mut self, _: CodeLocation, src: String) {
        println!("Parse output:");
        println!("{}", src);
    }
}

fn main() {
    let mut cli = CLI::new(Options {
        lang: Some(Lang::JS),
        debug: true,
    });
    Compiler::start(&mut cli);

    if cli.errors > 0 {
        println!("Unable to compile file, {} errors occurred", cli.errors);
        std::process::exit(1);
    }

    if cli.warnings == 0 {
        println!("Successfull compiled code");
    } else {
        println!("Successfull compiled code with {} warnings", cli.warnings);
    }
}
