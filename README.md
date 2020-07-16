# Talpa *a General Programming Language*

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
          * [x] structs `struct{}`
          * [x] arrays `[]string`
          * [x] enums `enum{}`
        * [x] global types
          * [x] structs `struct foo {}`
          * [x] enums `enum foo {}`
          * [x] custom types `type foo = []bar`
      * [x]  Actions
        * [x]  Variables
          * [x]  Keyword and name `let a`, `const a`
          * [x]  Variable type `let a: string`
          * [x]  assignment `let foo = "bar"` or `let foo = bar()`
        * [x]  Function
          * [x]  default `foo()`
          * [x]  arguments `foo(bar, "baz")`
        * [x]  Static actions
          * [x]  `return`
          * [x]  `loop {}`
          * [x]  `while true {}`
          * [x]  `for foo in bar {}`
      * [ ] Importing
        * [ ] Detecting to import someting
        * [ ] Validating imports
        * [ ] Detect import cycles
        * [ ] Propper debugging
          * [ ] Error messages show file origin

   * Parsing stage 2 verifying the data and making it more accessible

      * [x] Functions
        * [x] Name
          * [x] Duplicates
      * [x] Enums, Structs, Global Types
        * [x] Name
          * [x] Duplicates
      * [ ] Validate type names
        * [ ] Duplicates
      * [ ] List of actions
        * [ ] Make it impossible to set variable without using it
        * [ ] No duplicated variable names
        * [ ] No variable references that do not exsists

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
- No non cargo tools required to build this project *([By installing rust](https://www.rust-lang.org/tools/install) you should have everything to get started working on this though i might require rust nightly if needed)*


## License

[MIT](https://choosealicense.com/licenses/mit/)
