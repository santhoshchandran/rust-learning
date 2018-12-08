# Functions:
1. Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words
2. Functions can also be defined to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters
```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

# Statements and Expressions
Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples.
Creating a variable and assigning a value to it with the let keyword is a statement. Here, let y = 6; is a statement:
```
fn main() {
    let y = 6;
}
```
Likewise in the below example, `x + 1` is an expression as it is defined without an semicolon at the end and will return a value.

```
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
