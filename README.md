# Learn rust

### Index
- [Variables](#variables)
- [Data types](#data-types)
- [Functions](#functions)
- [Control flow](#control-flow)
- [Ownership](#ownership)
- [References](#references-and-borrowing)

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
### Using if condition in let statement
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

## Ownership
Ownership is a set of rules that govern how a Rust program manages memory. In rust, memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running. 
These rules are
- Each value in Rust has an **owner**
- There can only be one owner at a time
- When the owner goes out of score, the value will dropped.

In simple term the ownership in rust is like lifetime of a variable in other languages like JS or c++. 
In high level programming languages like JavaScript or Python, A garbage collector is used to free up unused memory. System programming languages like C or C++, If you are dynamically allocating memory then you have to remove unused memory manually. 
<br/>
<br/>
However, rust does not have a garbage collector, or you don't have to manually  unused memory. Rust has a concept called ownership. 
For example, In the below block of code, the variable `x` only valid inside the block of code. when the block of code executed the variable `x` removed from memory. This is similar to the other programming language. When a variable goes out of scope rust called `drop` function the remove the variable.

```
  {
     let x = 12;
  }  
```
### variable interaction

```
let x = 12;
let y = x;
```
In the above example, we have two variables `x` and `y`, and we are doing is we are assigning value 12 to the `x` variable and then making a copy of the value of the variable `x`, and assigning to the variable `y`. This is easy to understand. 
<br/>
One important thing to note is only data inside the stack can be copied. If a data stored inside the heap, then that data cannot be copied. For example, in rust string has four properties - name (variable name), ptr (pointer to the heap where value is stored), len (total amount of memory currently consuming), capacity (total amount of memory allocated by the allocator for the string). these four properties are store in the stack, and the value of the string stored in the heap. So, when we do this -

```
let s1 = String::from("Hello world!");
let s2 = s1;
```
We are actually coping the properties of `s1` which are stored in the stack to the s2. So, s2 is a new pointer pointed to the value stored in the heap. 
<br/>
Earlier, we said that we variable goes out of score, rust called the `drop` function to cleanup the memory. This is a problem for `s1` and `s2`, because when these two variables are just pointers. So, calling `drop` function means cleaning up same memory two times, which can lead to memory corruption.
<br/>
To prevent this issue, when the code `let s2 = s1` runs, the variable `s1` become invalid. You cannot used the `s1` variable, if you use it, you will be getting an error.
<br/>
PS: we can also copy heap value of a variable using clone method.
```
let s1 = String::from("hello");
let s2 = s1.clone();
```
<br/>
Another thing to remember is that, if a data type has `copy` trait, then the value always be copied. This type of value always be stored in stack. Such as integer, boolean, float, characters, tuples (if it contains data type that has `copy` trait). 

### ownership when passing value to a function
the mechanism is same like the variable.
```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

[Back to Top](#index)

## References and Borrowing
A reference is like a pointer pointed to a value stored in the memory, which is owned by an another variable. Unlike a pointer, a reference is guaranteed to point to a valid value.
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```


[Back to Top](#index)