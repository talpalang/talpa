# Functions

> This documentation is incomplete.

Functions are declared using the `fn` keyword. Its arguments are type annotated, with the type following the name of the formal argument. 
If the function returns a value, the return type must be specified after the arguments are defined, but before the scope.
The scope contains the functions code. It begins at `{` and ends at `}`.
Use the `return` keyword to return from the function, and if you are returning a value, it is followed by that value.

Below is an implementation of a simple adder function.

```
fn add(a int, b int) int {
    let c = a + b
    return c
}
```

As you can see, we take two arguments (a and b) that must both be integers. We define the return type (int). We then return the result of `a + b` using the return keyword inside the scope. 

We can call any function we’ve defined by typing its name followed by a set of parentheses. 
Any arguments we wish to parse are placed inside the parentheses.
For example, in our main function, we may wish to use our `add` function to add 2 and 4.

```
fn main() {
    let result = add(2, 4)
}
```

This code calls our `add` function, using 2 as the value for the `a` argument and 4 as the value for the `b` argument. 
The `add` function then computes and returns `c` which is now equal to `a + b`. 
Finally, `result` is assigned to the returned value (which should be 6).
