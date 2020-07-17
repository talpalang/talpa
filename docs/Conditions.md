# Conditions

> This document is incomplete

- [If](#if)
- [Else](#else)
- [Else If](#else-if)
- [Match](#match)

## If

An `if` statement allows you to evaluate whether or not a block of code should be run based on a condition. 
If the condition is met (returns true) the block of code will be run, otherwise it will not and the program will continue.

```cpp
if condition {
  // this code only runs if condition is true
}
```


## Else

We can include an `else` statement after an `if` statement. If the condition from the `if` statement is false, the else block will run instead. It will not run if the condition from the `if` statement is true.

```cpp
if condition {
  // this code only runs if condition is true
} else {
  // this code only runs if condition is not true (false)
}
```


## Else If

You can have multiple conditions by combining `if` and `else` in an `else if` expression. 

```cpp
if condition1 {
  // this code only runs if condition1 is true
} else if condition2 {
  // this code only runs if condition1 is false AND condition2 is true
} else {
  // this code only runs if condition1 AND condition2 are both false
}
```

## Match

Another way of comparing multiple conditions is to use the `match` statement.

```rust
match foo {
  // if foo == bar
  bar => do_something()
  // else if foo == baz
  baz => do_something_else()
}
```
