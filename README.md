# Talpa *a General Programming Language*

[![Rust](https://github.com/talpalang/talpa/workflows/Rust/badge.svg)](https://github.com/talpalang/talpa/actions)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct.md)
[![GitHub](https://img.shields.io/github/license/talpalang/talpa)](https://choosealicense.com/licenses/mit/)

> A WIP programming language designed to be compiled into many other languages.
> For more information, see the [OSI issue](https://github.com/open-source-ideas/open-source-ideas/issues/235).

Currently, the language contains very few working features. When adding new features, the project follows [test-driven development](https://en.wikipedia.org/wiki/Test-driven_development) practices.


## Roadmap

This roadmap is used to track the progress on the project. If you add a feature and all tests pass, tick it off below.

   * Parsing stage 1 (Parse the code into types)

      * [x]  Functions
        * [x]  Function keyword and body detection `fn FunctionName() {}`
        * [x]  Function arguments `fn foo(bar string) {}`
        * [x]  Function response `fn foo() string {}`
      * [x]  Types
        * [x]  Name parsing `string`, `foo`, `bar123`, `int`, `i8`
        * [x]  Extending types parsing  or `[]string`
        * [x]  Inline types
        * [x]  arrays `[]string`
        * [x]  structs `struct foo {}` & `struct {}`
        * [x]  enums `enum foo {}` & `enum {}`
        * [x]  custom types `type foo = []bar``
      * [ ]  Actions
        * [x]  Variables
        * [x]  Function
          * [x]  default `foo()`
          * [x]  arguments `foo(bar, "baz")`
        * [ ]  Static actions
          * [x]  `return`
          * [x]  `loop {}`
          * [x]  `while true {}`
          * [x]  `for foo in bar {}`
          * [x]  `if foo {} else if bar {} else {}`
          * [ ] match
            * [ ] `match foo { }`
            * [ ] `match foo { _ => {} }`
            * [ ] `match foo { bar => {} _ => {} }`
      * [ ]  Importing
        * [x]  Detecting to import something
        * [ ]  Validating imports
        * [ ]  Detect import cycles
        * [ ]  Proper debugging
          * [ ]  Error messages show file origin

   * Parsing stage 2 verifying the data and making it more accessible

      * [x]  Output
        * [x]  Error logging
        * [x]  Warning logging
      * [x]  Functions
        * [x]  Name
      * [x]  Enums, Structs, Global Types
        * [x]  Name
      * [x]  Validate types names
        * [x]  Duplicates
        * [ ]  Reference to other types must exist
      * [ ]  List of actions
        * [ ]  Make it impossible to set variable without using it
        * [x]  No duplicated variable names
        * [x]  No variable references that do not exist

   * Documentation

      * [x]  Code examples for the currently support language features in tests
      * [x]  A Markdown file with code examples (see [docs](docs/README.md))
        * [x]  What is currently supported (see [docs](docs/README.md))
        * [x]  List of language features goals like how should inline function work etc.. (see [dev plans](docs/README.md#maintainer-development-plans))


## Testing

As the project uses [test-driven development](https://en.wikipedia.org/wiki/Test-driven_development), it is important tests are run when making changes. Tests can be run by using the following commands in the command line.

```bash
# Run all tests
cargo test

# Run a specific test
cargo test -- --nocapture test_empty

# Run all tests with function in the name
cargo test function
```


## Design Goals

Some design goals I think are important:
- Preferably no dependencies
- No non cargo tools required to build this project *([By installing rust](https://www.rust-lang.org/tools/install) you should have everything to get started working on this though I might require rust nightly if needed)*


## License

[MIT](https://choosealicense.com/licenses/mit/)
