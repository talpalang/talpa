# Compiler

> This document is incomplete

This compiler uses 3 stages to get from talpa code to the code in the target langauge.
Underhere the 3 stages are described.

## Stage 1 *Parsing the code*

Here we parse the user written code into data so we can more easily anylize the data and later transform it into new code.

Code Location:
```
compiler/tokenize
```

## Stage 2 *Checking*

Here we check and anylize the above parsed code for errors.
Stage 1 only parses the code.
For example:
- Newly created variables do not exist in the same scope.
- Variable assigment match type.
- Variable assigment it's variable exists and is not a constant.
- If a type refers to another type check it.

Code Location:
```
compiler/anylize
```

## Stage 3 *Create new code*

Here we create new code for the language the user specified.
This code uses the anylized code from stage 2 to build new code.

Code Location:
```
compiler/target
```
