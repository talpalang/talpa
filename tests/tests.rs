use gpl::Parser;

// Parse a string of code
fn parse_str(contents: impl Into<String>) -> gpl::Parser {
    gpl::Parser::parse(contents.into().as_bytes()).unwrap()
}

// Parse a string of code that is meant to fail
fn parse_str_fail(contents: impl Into<String>) {
    // Parse the code
    let res = gpl::Parser::parse(contents.into().as_bytes());
    // If the code parsed without error
    if let Ok(parsed_content) = res {
        // There is a problem with the parser
        // Output the result in an error (failing the test)
        panic!("{:?}", parsed_content);
    }
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
fn test_variable_starts_with_number() {
    // variables should never start with a number
    parse_str_fail(
        r#"
            const 1fail = 0
        "#
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

#[test]
fn test_variable_global_let() {
    parse_str_fail(
        r#"
            let foo = 0
        "#
    );
}
