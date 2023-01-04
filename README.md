# Learn rust

## Variables
`Let` keyword is used to declare variable in rust. By default values are immutable in rust. You have to use `mut` keyword to make variable mutable. In the below example variable `x` is a immutable variable and `y` is a mutable variable

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
There is also constant variable. `const` keyword is used to create constant and you have to mention the data type of the variable. Difference between constant and let variable is constant variable generated compile-time and let variable generated in run-time. constant variable are used for storing global variable which are used in different parts of the code.

```
fn main() {
    const tree:i32 = 3;
}
```

## Data types
https://doc.rust-lang.org/book/ch03-02-data-types.html

### Tuples
Tuple is a list of values and values can be different types Tuples are fixed size.
```
fn main() {
    let tup:(i32,f64,u8) = (500,6.4,1);

    // use destructing to extract values
    let (x,y,z) = tup;
    println!("{x}, {y}, {z}");

    // use index to extract values
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("{first}, {second}, {third}");
}

```
### Array
Array is also a list of values but unlike tuple the values in the array must be same data type

```
fn main() {
    let arr = [1,2,3,4,6];

    //  you can also mention data type and size of the array
    let arr2:[i32; 5] = [1,2,3,4,6];

    let first = arr[0];
    let second = arr2[1];

    println!("{first}, {second}")
}

```