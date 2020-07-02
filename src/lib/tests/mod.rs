mod comments;
mod functions;
mod general;
mod variables;

use super::*;

// Parse a string of code
pub fn parse_str(contents: impl Into<String>) -> Parser {
  Parser::parse(contents.into().as_bytes()).unwrap()
}

// Parse a string of code that is meant to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  // Parse the code
  let res = Parser::parse(contents.into().as_bytes());
  // If the code parsed without error
  if let Ok(parsed_content) = res {
    // There is a problem with the parser
    // Output the result in an error (failing the test)
    panic!("{:?}", parsed_content);
  }
}
