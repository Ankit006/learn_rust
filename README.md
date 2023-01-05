# Learn rust

### Index
- [Variables](#variables)
- [Data types](#data-types)
- [Functions](#functions)
- [Control flow](#control-flow)

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
[Back to Top](#index)

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
[Back to Top](#index)

## Functions
```
fn main() {
    another_func(12);
}

fn another_func(x:i32) {
    println!("Hello world {x}");
}

```
When returning a value from a function, you have to explicitly define what type of data your are returning by using `->` sign after parenthesis. There are two ways you can return value in rust.
1. Using `return` keyword
```
fun add_five(x:i32)
{
    return x+5;
}
```
2. Not adding **semicolon** at the end of the statement. 

```
fun add_five(x:i32)
{
    x+5
}
```
One of the important thing to note is a statement does not return a value. So you cannot write a statement like this ``` let x = y = 12; ```. But an expression (a piece of code which returns a value) can be a part of statement.
```
fn main()
{
    let x = {
        let y = 12;
        y + 6
    };

    println!("{x}");
}
```
In the above code the expression is
```
    {
     let y = 12;
     y + 6
    };
```
[Back to Top](#index)

## Control flow

### If else statement
```
fn main()
{
    let number = 23;

    if number > 24{
        println!("Bigger");
    } else if number > 18 && number < 24{
        println!("middle");
    }else{
        println!("Less");
    }
}
```
### using if condition in let statement
You can assign if else condition to an variable. But the return value should be same in the conditions

```
fn main()
{
   let condition = true;

   let x = if condition {5} else {6}; 
   println!("{x}");
}
```
but you cannot write like this.
```
fn main()
{
   let condition = true;

   let x = if condition {5} else {"six"}; 
   println!("{x}");
}
```
because the return value if and else condition is different. If returning integer and else returning string but but value must be same data type.

### Loop
```
fn main()
{
  let mut counter = 0;

   loop{
    if counter == 10{
        break;
    }
    println!("again");
    counter += 1;
   }
}
```
### While loop

```
fn main()
{
  let mut num = 0;
  while num <= 10 {
      println!("again");
      num += 1;
  }
}
```
### For loop
In rust for loop normally used in traverse through list of values such as array
```
fn main()
{
    let x:[i32; 5] = [10,20,30,40,50];

    for val in x {
        println!("{val}");
    }
}
```
you can also print list of value like this
```
fn main()
{
    for val in 1..6{
        println!("{val}");
    }

    println!("------------------");
    // print values in reverse order
    for val in (1..6).rev(){
        println!("{val}");
    }
}
```
[Back to Top](#index)


