# Learn rust

## Variables
`Let` is used to declare variable in rust. By default values are immutable in rust. You have to use `mut` keyword to make variable mutable. In the below example variable `x` is a immutable variable and `y` is a mutable variable

```
fn main() {
    let x = 5;
    println!("{x}");
    let mut y = 20;
    println!("{y}");
    y = 10;
    println!("{y}");
}
```
There is also constant variable. `const` keyword is used to create constant and you have to mention the data type of the variable. Difference between constant and let variable is constant variable generated compile-time and let variable generated run time. constant variable are used for storing global variable which are used in different parts of the code.

```
fn main() {
    const tree:i32 = 3;
}
```