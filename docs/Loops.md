# Loops

> This document is incomplete

- [Loop](#loop)
- [While](#while)
- [For](#for)


## Loop

The `loop` keyword tells Talpa to execute a block of code infinitely or until you tell it to stop.

```rust
loop {
  // execute this code over and over again
}
```

You can exit the loop by using the `break` keyword. 

```rust
loop {
  // break the loop
  break
}
```


## While

The `while` keyword tells talpa to loop through a block of code only while the condition is true. 

```cpp
while condition == true {
  // while the condition is true, execute this code
}
```

You can also use the break keyword here.

```cpp
while condution == true {
  // break the loop
  break
}
```


## For

The `for` keyword tells talpa to loop through each item in an iterator and execute the code each time. 

```rust
for item in iter {
  // you can access the item variable while within this scope
}
```

You can also use the break keyword here.

```rust
for item in iter {
  // if we find a string "exit", break out from the loop
  if item == "exit" {
    break
  }
}
```
