use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // the .gpl is a temporary file extension (General Programming Language)
    // the example file should be updated with all working components
    let mut file = File::open("./src/example.gpl").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    match Parser::parse(contents) {
        Err(err) => println!("{}", err),
        Ok(res) => println!("{:?}", res.functions),
    }
}

static CONST_KEYWORD: &'static str = "const";
static LET_KEYWORD: &'static str = "let";
static RETURN_KEYWORD: &'static str = "return";
static VALID_NAME_CHARS: &'static str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

fn legal_name_char(c: char) -> bool {
    VALID_NAME_CHARS.contains(c)
}

#[derive(Debug)]
enum ParsingErrorType {
    IncompletedArgument,
    UnexpectedEOF,
    UnexpectedChar,
    UnexpectedResult,
    InvalidNameChar,
    Custom(&'static str),
}

impl Display for ParsingErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IncompletedArgument => write!(f, "Incompletted argument"),
            Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
            Self::UnexpectedChar => write!(f, "Unexpected char"),
            Self::UnexpectedResult => write!(f, "Unexpected result"),
            Self::InvalidNameChar => write!(f, "Invalid name char"),
            Self::Custom(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug)]
struct CodeLocation {
    file_name: Option<String>,
    x: usize,
    y: usize,
}

struct ParsingError {
    location: CodeLocation,
    error_type: ParsingErrorType,
    prev_line: Option<String>,
    line: String,
    next_line: Option<String>,
}

impl ParsingError {
    fn err(&self) -> String {
        let mut output: Vec<String> = vec![];
        let y = self.location.y;

        if let Some(line) = self.prev_line.clone() {
            output.push(format!("{}: {}", y - 1, line.replace("\t", "  ")));
        }

        let mut spacing = String::from("");
        for _ in 0..self.location.x + y.to_string().len() {
            spacing += " ";
        }
        output.push(format!(
            "{}: {}\n{}^-- {}",
            y,
            self.line.replace("\t", "  "),
            spacing,
            self.error_type,
        ));

        if let Some(line) = self.next_line.clone() {
            output.push(format!("{}: {}", y + 1, line.replace("\t", "  ")));
        }

        format!("{}", output.join("\n"))
    }
}

impl Error for ParsingError {}

impl std::fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err())
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.err())
    }
}

struct Parser {
    index: usize,
    contents: Vec<u8>,
    pub functions: Vec<Function>,
}

impl Parser {
    fn error<T>(&self, error_type: ParsingErrorType) -> Result<T, ParsingError> {
        self.custom_error(error_type, None)
    }
    fn unexpected_char<T>(&self) -> Result<T, ParsingError> {
        self.error(ParsingErrorType::UnexpectedChar)
    }
    fn unexpected_eof<T>(&self) -> Result<T, ParsingError> {
        self.error(ParsingErrorType::UnexpectedEOF)
    }
    fn custom_error<T>(
        &self,
        error_type: ParsingErrorType,
        file_char_number: Option<usize>,
    ) -> Result<T, ParsingError> {
        let use_index = if let Some(index) = file_char_number {
            index
        } else {
            self.index - 1
        };
        let mut line_number = 1;
        let mut current_line_position = 1;
        let mut prev_line_bytes: Option<Vec<u8>> = None;
        let mut current_line = vec![];

        for (index, letter) in self.contents.iter().enumerate() {
            if index == use_index {
                break;
            }
            match *letter as char {
                '\n' => {
                    prev_line_bytes = Some(current_line);
                    current_line = vec![];
                    line_number += 1;
                    current_line_position = 0;
                }
                '\r' => {} // Ignore this char
                letter_char => {
                    current_line.push(*letter);
                    current_line_position += if letter_char == '\t' { 2 } else { 1 };
                }
            }
        }

        let mut prev_line = None;
        if let Some(line_data) = prev_line_bytes {
            prev_line = Some(String::from_utf8(line_data).unwrap())
        }

        let mut next_line_bytes: Option<Vec<u8>> = None;
        let iterrator = self.contents.iter().skip(use_index);
        for letter in iterrator {
            match *letter as char {
                '\n' => {
                    if let Some(_) = next_line_bytes {
                        break;
                    }
                    next_line_bytes = Some(vec![]);
                }
                '\r' => {} // Ignore this char
                _ => {
                    if let Some(mut line) = next_line_bytes {
                        line.push(*letter);
                        next_line_bytes = Some(line);
                    } else {
                        current_line.push(*letter);
                    }
                }
            }
        }

        let next_line = if let Some(bytes) = next_line_bytes {
            Some(String::from_utf8(bytes).unwrap())
        } else {
            None
        };

        let res = ParsingError {
            location: CodeLocation {
                file_name: None,
                y: line_number,
                x: current_line_position,
            },
            error_type,
            prev_line,
            line: String::from_utf8(current_line).unwrap(),
            next_line: next_line,
        };
        Err(res)
    }
    fn parse(contents: impl Into<Vec<u8>>) -> Result<Self, ParsingError> {
        let mut parser = Self {
            index: 0,
            contents: contents.into(),
            functions: vec![],
        };
        parser.parse_nothing()?;
        Ok(parser)
    }
    fn next_char(&mut self) -> Option<char> {
        let letter = self.contents.get(self.index)?;
        self.index += 1;
        Some(*letter as char)
    }
    fn seek_next_char(&mut self) -> Option<char> {
        let letter = self.contents.get(self.index)?;
        Some(*letter as char)
    }
    fn next_while(&mut self, chars: &'static str) -> Option<char> {
        while let Some(c) = self.next_char() {
            if !chars.contains(c) {
                return Some(c);
            }
        }
        None
    }
    // fn expect(&mut self, text: &str) -> Result<(), ParsingError> {
    //     for letter in text.chars() {
    //         match self.next_char() {
    //             Some(v) if v == letter => {}
    //             Some(_) => return self.error(ParsingErrorType::UnexpectedChar, None),
    //             None => {
    //                 return self.error(ParsingErrorType::UnexpectedEOF, None);
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    fn forward_until(
        &mut self,
        allowed_chars: impl Into<String>,
        until: char,
    ) -> Result<(), ParsingError> {
        let allowed_chars_string = allowed_chars.into();
        while let Some(c) = self.next_char() {
            if c == until {
                return Ok(());
            }
            if !allowed_chars_string.contains(c) {
                return self.error(ParsingErrorType::UnexpectedChar);
            }
        }
        self.error(ParsingErrorType::UnexpectedEOF)
    }

    /// Tries to match something
    /// The second string for the options array is for checking if the matched value has a certen surfix
    /// The next char after the matched value will be checked against it
    /// For example surfix "abc" will match the following matched string surfix: 'a', 'b' or 'c'
    fn try_match(&mut self, options: &[(&'static str, &'static str)]) -> Option<&'static str> {
        if options.len() == 0 {
            return None;
        }

        let mut surfix_map: HashMap<&'static str, &'static str> =
            HashMap::with_capacity(options.len());
        let mut options_vec: Vec<&'static str> = vec![];
        for option in options {
            if option.0.len() == 0 {
                continue;
            }
            options_vec.push(option.0);

            if option.1.len() > 0 {
                surfix_map.insert(option.0, option.1);
            }
        }

        let mut char_count: usize = 0;
        while let Some(c) = self.next_char() {
            let mut new_options_vec: Vec<&'static str> = vec![];
            for option in options_vec {
                if option.len() <= char_count {
                    continue;
                }
                match option.as_bytes().get(char_count) {
                    Some(found_char) if *found_char as char == c => {
                        if option.len() != char_count + 1 {
                            new_options_vec.push(option);
                            continue;
                        }

                        if let Some(must_match_surfix) = surfix_map.get(option) {
                            // This option contains a surfix match, lets test it here
                            let next_char = self.seek_next_char();
                            if let None = next_char {
                                continue;
                            } else if !must_match_surfix.contains(next_char.unwrap()) {
                                continue;
                            }
                        }

                        return Some(option);
                    }
                    _ => continue,
                }
            }
            if new_options_vec.len() == 0 {
                break;
            }
            options_vec = new_options_vec;
            char_count += 1;
        }

        // Reset the index if we havent found the requested item
        self.index -= char_count;
        None
    }
    fn expect_next(&mut self, c: char) -> Result<(), ParsingError> {
        match self.next_char() {
            Some(v) if v == c => Ok(()),
            Some(_) => self.error(ParsingErrorType::UnexpectedChar),
            None => self.error(ParsingErrorType::UnexpectedEOF),
        }
    }
    fn parse_nothing(&mut self) -> Result<(), ParsingError> {
        while let Some(c) = self.next_char() {
            match c {
                'f' => {
                    self.expect_next('n')?;
                    let new_func = ParseFunction::start(self)?;
                    self.functions.push(new_func);
                }
                _ => {}
            };
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Function {
    pub name: Option<String>,
    pub args: Vec<(String, Type)>,
    pub body: Actions,
}

impl Function {
    fn empty() -> Self {
        Self {
            name: None,
            args: vec![],
            body: Actions::empty(),
        }
    }
}

#[derive(Debug)]
struct ParseFunctionStateNothing {
    function_name: Option<Vec<u8>>,
}

#[derive(Debug)]
struct ParseFunctionStateArg {
    name: Vec<u8>,
    type_: Option<Type>,
    parsing_name: bool,
}

impl ParseFunctionStateArg {
    fn new() -> Self {
        Self {
            name: vec![],
            type_: None,
            parsing_name: true,
        }
    }
}

#[derive(Debug)]
enum ParseFunctionState {
    Nothing(ParseFunctionStateNothing),
    Arg(ParseFunctionStateArg),
    AfterArg,
    Response,
}

struct ParseFunction<'a> {
    p: &'a mut Parser,
    res: Function,
    state: ParseFunctionState,
}

impl<'a> ParseFunction<'a> {
    fn change_state(&mut self, to: ParseFunctionState) {
        // Check if the current state has data and if so commit it to the response
        match &self.state {
            ParseFunctionState::Nothing(info) => {
                if let Some(name) = &info.function_name {
                    self.res.name = Some(String::from_utf8(name.clone()).unwrap());
                }
            }
            ParseFunctionState::Arg(info) if !info.parsing_name && info.name.len() > 0 => {
                if let Some(type_) = &info.type_ {
                    self.res
                        .args
                        .push((String::from_utf8(info.name.clone()).unwrap(), type_.clone()));
                }
            }
            ParseFunctionState::Arg(_) => {}
            ParseFunctionState::AfterArg => {}
            ParseFunctionState::Response => {}
        }

        self.state = to;
    }
    fn start(p: &'a mut Parser) -> Result<Function, ParsingError> {
        let mut s = Self {
            p,
            res: Function::empty(),
            state: ParseFunctionState::Nothing(ParseFunctionStateNothing {
                function_name: None,
            }),
        };
        s.parse()?;
        Ok(s.res)
    }
    fn parse(&mut self) -> Result<(), ParsingError> {
        while let Some(c) = self.p.next_char() {
            match &mut self.state {
                ParseFunctionState::Nothing(meta) => match c {
                    '\t' | '\n' | ' ' => {
                        if let Some(_) = meta.function_name {
                            // Not a valid name char return error
                            return self.p.error(ParsingErrorType::InvalidNameChar);
                        }
                    }
                    '(' => self.change_state(ParseFunctionState::Arg(ParseFunctionStateArg::new())), // end of function name, start parsing arguments
                    c if legal_name_char(c) => {
                        // Parsing the function name
                        if let Some(function_name) = &mut meta.function_name {
                            function_name.push(c as u8);
                        } else {
                            meta.function_name = Some(vec![c as u8])
                        }
                    }
                    _ => {
                        // Not a valid name char return error
                        return self.p.error(ParsingErrorType::InvalidNameChar);
                    }
                },
                ParseFunctionState::Arg(meta) => match c {
                    '\t' | '\n' | ' ' => {
                        if meta.name.len() > 0 {
                            meta.parsing_name = false;
                        }
                    }
                    ')' => match meta.type_ {
                        None if meta.name.len() > 0 => {
                            // Argument not completed
                            return self.p.error(ParsingErrorType::IncompletedArgument);
                        }
                        _ => {
                            // End of argument
                            self.change_state(ParseFunctionState::Response);
                        }
                    }, // end of argument, start parsing response
                    c if legal_name_char(c) => {
                        if meta.parsing_name {
                            // Parsing the function name
                            meta.name.push(c as u8);
                        } else {
                            // Parse the argument type
                            meta.type_ = Some(ParseType::start(self.p, true)?);
                            self.change_state(ParseFunctionState::AfterArg);
                        }
                    }
                    _ => {
                        // Not a valid name char return error
                        return self.p.error(ParsingErrorType::InvalidNameChar);
                    }
                },
                ParseFunctionState::AfterArg => match c {
                    '\t' | '\n' | ' ' => {}
                    ')' => {
                        self.change_state(ParseFunctionState::Response);
                    }
                    ',' => {
                        self.change_state(ParseFunctionState::Arg(ParseFunctionStateArg::new()));
                    }
                    _ => {
                        // This is not what we are searching for
                        return self.p.error(ParsingErrorType::InvalidNameChar);
                    }
                },
                ParseFunctionState::Response => match c {
                    '{' => {
                        self.res.body = ParseActions::start(self.p)?;
                        return Ok(());
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Type {
    name: String,
}

impl Type {
    fn empty() -> Self {
        Self {
            name: String::new(),
        }
    }
}

struct ParseTypeStateTypeName {
    name: Vec<u8>,
}

impl ParseTypeStateTypeName {
    fn new() -> Self {
        Self { name: vec![] }
    }
}

enum ParseTypeState {
    TypeName(ParseTypeStateTypeName),
}

struct ParseType<'a> {
    p: &'a mut Parser,
    res: Type,
    state: ParseTypeState,
}

impl<'a> ParseType<'a> {
    fn start(p: &'a mut Parser, go_back_one: bool) -> Result<Type, ParsingError> {
        if go_back_one {
            p.index -= 1;
        }
        let mut s = Self {
            p,
            res: Type::empty(),
            state: ParseTypeState::TypeName(ParseTypeStateTypeName::new()),
        };
        s.parse()?;
        Ok(s.res)
    }
    fn parse(&mut self) -> Result<(), ParsingError> {
        while let Some(c) = self.p.next_char() {
            match &mut self.state {
                ParseTypeState::TypeName(meta) => match c {
                    _ if legal_name_char(c) => {
                        meta.name.push(c as u8);
                    }
                    _ => {
                        self.p.index -= 1;
                        self.res.name = String::from_utf8(meta.name.clone()).unwrap();
                        return Ok(());
                    }
                },
            }
        }
        Ok(())
    }
}

/// Contains the
#[derive(Debug)]
struct Actions {
    list: Vec<Action>,
}

impl Actions {
    fn empty() -> Self {
        Self { list: vec![] }
    }
}

enum ParseActionsState {
    Nothing,
}

struct ParseActions<'a> {
    p: &'a mut Parser,
    res: Actions,
    state: ParseActionsState,
}

impl<'a> ParseActions<'a> {
    fn start(p: &'a mut Parser) -> Result<Actions, ParsingError> {
        let mut s = Self {
            p,
            res: Actions::empty(),
            state: ParseActionsState::Nothing,
        };
        s.parse()?;
        Ok(s.res)
    }
    fn parse(&mut self) -> Result<(), ParsingError> {
        while let Some(c) = self.p.next_char() {
            match self.state {
                ParseActionsState::Nothing => match c {
                    '}' => return Ok(()),
                    _ if legal_name_char(c) => {
                        let action =
                            ParseAction::start(self.p, true, ActionToExpect::ActionInBody)?;
                        self.res.list.push(action);
                    }
                    _ => return self.p.unexpected_char(),
                },
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum ActionVariableType {
    Const, // inmutatable
    Let,   // mutatable
}

#[derive(Debug)]
struct ActionVariable {
    name: String,
    var_type: ActionVariableType,
    type_: Option<Type>,
    action: Box<Action>,
}

#[derive(Debug)]
enum Action {
    Variable(ActionVariable),
    Return(Option<Box<Action>>),
}

struct ParseAction<'a> {
    p: &'a mut Parser,
    res: Option<Action>,
    action_to_expect: ActionToExpect,
}

enum ParseActionState {
    Var(ParseActionStateVar),
    Return(ParseActionStateReturn),
}

struct ParseActionStateReturn {
    action: Option<Action>, // The value to return
}

impl Into<ParseActionState> for ParseActionStateReturn {
    fn into(self) -> ParseActionState {
        ParseActionState::Return(self)
    }
}

struct ParseActionStateVar {
    var_type: ActionVariableType, // What kind of variable is this a const or a let
    name: Vec<u8>,                // The variable name
    type_: Option<Type>,          // The variable type
    action: Option<Action>,       // The value set to the variable
}

impl Into<ParseActionState> for ParseActionStateVar {
    fn into(self) -> ParseActionState {
        ParseActionState::Var(self)
    }
}

#[derive(PartialEq)]
enum ActionToExpect {
    ActionInBody, // A line in a function body
    Assignment, // A assingment of some sort, like the contents of a variable or a function argument or the value of the return
}

impl<'a> ParseAction<'a> {
    fn start(
        p: &'a mut Parser,
        go_back_one: bool,
        action_to_expect: ActionToExpect,
    ) -> Result<Action, ParsingError> {
        if go_back_one {
            p.index -= 1;
        }
        let mut s = Self {
            action_to_expect,
            p,
            res: None,
        };
        s.detect()?;
        if let Some(res) = s.res {
            Ok(res)
        } else {
            s.p.error(ParsingErrorType::UnexpectedResult)
        }
    }
    fn commit_state(&mut self, state: impl Into<ParseActionState>) -> Result<(), ParsingError> {
        match state.into() {
            ParseActionState::Var(meta) => {
                if let None = meta.action {
                    return self
                        .p
                        .error(ParsingErrorType::Custom("Missing variable assignment"));
                }

                self.res = Some(Action::Variable(ActionVariable {
                    name: String::from_utf8(meta.name.clone()).unwrap(),
                    var_type: meta.var_type.clone(),
                    type_: meta.type_,
                    action: Box::new(meta.action.unwrap()),
                }));
            }
            ParseActionState::Return(meta) => {
                let mut return_action: Option<Box<Action>> = None;
                if let Some(action) = meta.action {
                    return_action = Some(Box::new(action));
                }
                self.res = Some(Action::Return(return_action))
            }
        };
        Ok(())
    }
    fn detect(&mut self) -> Result<(), ParsingError> {
        let matched_res = if self.action_to_expect == ActionToExpect::ActionInBody {
            self.p.try_match(&[
                (CONST_KEYWORD, " \t\n"),
                (LET_KEYWORD, " \t\n"),
                (RETURN_KEYWORD, "} \t\n"),
            ])
        } else {
            None
        };
        if let Some(matched) = matched_res {
            if matched == CONST_KEYWORD || matched == LET_KEYWORD {
                // Go to parsing the variable
                let var_type = if matched == CONST_KEYWORD {
                    ActionVariableType::Const
                } else {
                    ActionVariableType::Let
                };
                let new_var = self.parse_variable(Some(var_type))?;
                self.commit_state(new_var)?;
            } else if matched == RETURN_KEYWORD {
                // Go to parsing the return
                let new_var = self.parse_return()?;
                self.commit_state(new_var)?;
            }

            return Ok(());
        }

        return self.p.unexpected_char();
    }
    fn parse_return(&mut self) -> Result<ParseActionStateReturn, ParsingError> {
        let mut res = ParseActionStateReturn { action: None };

        match self.p.next_while(" \t\n") {
            Some('}') => {}
            Some(_) => {
                let action = ParseAction::start(self.p, true, ActionToExpect::Assignment)?;
                res.action = Some(action);
            }
            None => return self.p.error(ParsingErrorType::UnexpectedEOF),
        }

        Ok(res)
    }
    fn parse_variable(
        &mut self,
        var_type_option: Option<ActionVariableType>, // If set we assume the let or const is already parsed
    ) -> Result<ParseActionStateVar, ParsingError> {
        let var_type = if let Some(type_) = var_type_option {
            type_
        } else {
            let to_match = &[(CONST_KEYWORD, " \t\n"), (LET_KEYWORD, " \t\n")];
            let match_result = self.p.try_match(to_match);
            if let None = match_result {
                return self.p.unexpected_char();
            }

            if match_result.unwrap() == CONST_KEYWORD {
                ActionVariableType::Const
            } else {
                ActionVariableType::Let
            }
        };

        let mut res = ParseActionStateVar {
            name: vec![],
            var_type,
            type_: None,
            action: None,
        };

        // Parse name
        let mut next_char = self.p.next_while(" \t\n");
        loop {
            if let Some(c) = next_char {
                match c {
                    _ if legal_name_char(c) => res.name.push(c as u8),
                    ' ' | '\t' | '\n' => break,
                    ':' | '=' => {
                        self.p.index -= 1;
                        break;
                    }
                    _ => return self.p.unexpected_char(),
                }
            } else {
                return self.p.unexpected_eof();
            }
            next_char = self.p.next_char();
        }

        // Parse the variable type if set
        next_char = self.p.next_while(" \t\n");
        if let None = next_char {
            return self.p.unexpected_eof();
        }
        if next_char.unwrap() == ':' {
            res.type_ = Some(ParseType::start(self.p, true)?);
            next_char = self.p.next_while(" \t\n");
        }

        // Check for the = symbol
        match next_char {
            Some('=') => {}
            Some(_) => return self.p.unexpected_char(),
            None => return self.p.unexpected_eof(),
        }

        // Parse the action after the action after the =
        let parsed_action = ParseAction::start(self.p, false, ActionToExpect::Assignment)?;
        res.action = Some(parsed_action);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parse a string of code
    fn parse_str(contents: impl Into<String>) -> Parser {
        Parser::parse(contents.into().as_bytes()).unwrap()
    }

    #[test]
    fn test_empty() {
        parse_str(r#""#);
    }

    #[test]
    fn test_function_empty() {
        parse_str(
            r#"
                fn test() {}
            "#,
        );
    }

    #[test]
    fn test_functions_empty() {
        parse_str(
            r#"
                fn test1() {}
                fn test2() {}
            "#,
        );
    }

    #[test]
    fn test_function_with_arg() {
        parse_str(
            r#"
                fn test(name string) {}
            "#,
        );
    }

    #[test]
    fn test_function_with_args() {
        parse_str(
            r#"
                fn test(foo string, bar string, baz string) {}
            "#,
        );
    }

    #[test]
    fn test_function_with_result() {
        parse_str(
            r#"
                fn test() string {
                    return "a"
                }
            "#,
        );
    }

    #[test]
    fn test_function_with_arg_and_result() {
        parse_str(
            r#"
                fn test(ab string) string {
                    return ab
                }
            "#,
        );
    }

    #[test]
    fn test_function_call() {
        parse_str(
            r#"
                fn test() {}
                fn test_1() {
                    test()
                }
            "#,
        );
    }

    #[test]
    fn test_function_call_with_args() {
        parse_str(
            r#"
                fn test(a int, b int) {}
                fn test_1() {
                    test(1, 2)
                }
            "#
        );
    }

    #[test]
    fn test_variable() {
        parse_str(
            r#"
                const foo = "1234"
            "#,
        );
    }

    #[test]
    fn test_variable_string_with_spaces() {
        parse_str(
            r#"
                const foo = "Hello world!"
            "#
        );
    }
    
    #[test]
    fn test_variable_strings_with_backslashes() {
        parse_str(
            r#"
                const foo = "I like to say \"Hello World!\" in my code."
                const bar = "This \\ backslash is displayed when printed!"
            "#
        );
    }
}
