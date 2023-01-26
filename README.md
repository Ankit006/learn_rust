# Learn rust

### Index

- [Variables](#variables)
- [Data types](#data-types)
- [Functions](#functions)
- [Control flow](#control-flow)
- [Ownership](#ownership)
- [References](#references-and-borrowing)
- [The Slice type](#the-slice-type)
- [Struct](#struct)
- [Debug](#debug)

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

we can also take tuple as an argument of a function

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
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

One of the important thing to note is a statement does not return a value. So you cannot write a statement like this `let x = y = 12;`. But an expression (a piece of code which returns a value) can be a part of statement.

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
However, rust does not have a garbage collector, or you don't have to manually unused memory. Rust has a concept called ownership.
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

A reference is like a pointer pointed to a value stored in the memory, which is owned by an another variable. Unlike a pointer, a reference is guaranteed to point to a valid value. Reference is also called **borrowing**. One thing to rememebr

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

In case of the reference, the `s1` variable still be valid. So, here the ownership does not transfer from `s1` to `&s1`.
<br/>

### changing value using reference

Because variables are immutable by default in rust. You cannot change values directly using reference. You have to use mutable reference.

```
fn main(){
    let mut data = String::from("Hello");
    change_str(&mut data);
    println!("{data}");
}

fn change_str(s:&mut String){
    s.push_str(" world!");
}
```

One thing to remember is that **you cannot mutable reference of a variable if that variable already has a immutable reference and your are using the immutable reference**. for example -

```
fn main(){
    let mut s1 = String::from("hello");
    let slice:&str = &s1[0..2];
    s1.clear();
    println!("{}",slice);
}
```

The above code will throw an error because. the `slice` variable is taking an immutable reference of `s1` variable, but on the next statement we are try to clear `s1` variable using `clear` method which use mutable reference of the `s1` variable. Now because in the next statement we are using the immutable reference `slice` of the `s1` variable in the `println!` function, which means the `slice` variable still need to be active. So, the compiler will throw an error.

### Dangling pointer

A dangling pointer is a pointer that is a reference to memory that has been cleared up. So, the pointer pointed to a memory which does not contain any value. If a pointer is a dangling pointer, the rust compiler will throw an error. For example,

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

The above code will throw an error. Because, the string is created inside the `dangle` function, the `dangle` function finishes its execution, the `s` variable inside `dangle` function will become invalid and memory will be cleared. So, the variable `reference_to_noting` is pointed to a invalid string. This cause an error. To avoid this you can simply return the string

```
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

```

[Back to Top](#index)

## The Slice Type

Slice let you reference a part of a contiguous collection of element rather than the whole collection itself.

### String slice

String slice a reference to a part of a string.
For example, in the below code, the `slice` variable will store **world**.

```
 let s = String::from("hello world");

 let slice = &s[6..11];

```

in the range bracket 6 is the starting index of the slice slice and 11 is the one more than ending index of the slice. the `slice` variable stores starting index, length of the slice (this is corresponds to one more positing than the last index of the slice). the `slice` variable also stores a pointer pointed to the byte value in the position 6 (in this case it is `w`).

The data type of the string slice is `&str`. So, we can also write

```
 let slice:&str = &s[6..11];
```

here are some example to take different slices of string

```
 let s = String::from("hello world");
 let len = s.len();

 // the string slice will be up to the last value of the string

 let slice = &s[6..len];

 // this will take entire string as slice

 let slice = &s[..]
```

### integer array slice

concept is same as string slice. But the Data type is [i32]

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

```

[Back to Top](#index)

## Struct

Struct is a similar to class in other programming language. Here is example, of how to create struct and how create instances of a struct. Instances by default are immutable. You can also create mutable instances. Immutable instances value cannot be change once initialized.

```
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // immutable instance
    let ankit = User {
        active: true,
        name: String::from("Ankit Ghosh"),
        email: String::from("ankitghosh312@gmail.com"),
        sign_in_count: 1,
    };

    // mutable instance
    let mut arijit = User {
        active: true,
        name: String::from("Arijit"),
        email: String::from("arijit312@gmail.com"),
        sign_in_count: 1,
    };
    arijit.email = String::from("arijit111@gmail.com");

    // creating struct instance using function
    let _juli = create_user(String::from("Julfikar"), String::from("juli123@gmail.com"));
    println!(
        "{}, active: {}, sign-in {}",
        ankit.name, ankit.active, ankit.sign_in_count
    );
}

// function to create struct instances
fn create_user(name: String, email: String) -> User {
    User {
        active: true,
        name,
        email,
        sign_in_count: 1,
    }
}

```

### Copy value from instance to another

Let's say you have two user - `user1` & `user2`. `user2` has same value as `user1` except the email. You can easily copy `user1` value to `user2` using `..` syntax. This is kind a similar to javascript spread operator

```
fn main() {
    struct User {
        active: bool,
        name: String,
        email: String,
        sign_in_count: u64,
    }
    // immutable instance
    let user1 = User {
        active: true,
        name: String::from("Ankit Ghosh"),
        email: String::from("ankitghosh312@gmail.com"),
        sign_in_count: 1,
    };
    let user2 = User{
        email:String::from("nick543@gmail.com"),
        ..user1
    };
    println!("{}",user1.active);
}


```

However, you have remember the ownership. The `..` is using assignment `=` operator. In the user1 both `name` and `email` is string. So when we are copying the value from `user1` to `user2`, we create a new string for email in `user2`. But for the `name` properties, the ownership is move from `user1.name` to `user2.name`, which make `user1.name` invalid.
We can prevent this by simply creating a new string for `name` property in the `user2`.

```
 let user2 = User{
        email:String::from("nick543@gmail.com"),
        name:String::from("Ren Dover")
        ..user1
    };
```
### Methods
struct method are similar to the  method in class in other object oriented programming languages.

```
#[derive(Debug)]
struct Reactangle{
   width:u32,
   height:u32
}

impl Reactangle {
    fn area(&self)->u32{
      self.width * self.height
    }
}

fn main(){
   let ret1 = Reactangle{
      width:32,
      height:20
   };

   println!("{}",ret1.area());
}
```
the `impl` block is associate with the struct. withing this block we write the method. For every method the first parameter must be `self:&self` or in short hand `&self`. `&self` is a alias of the struct. It allows us to read properties of the struct. 
<br/>
However, `&self` does not allow us to properties value of the struct. To do this we need to use mutable self `&mut self` and we also need to make the struct instance mutable.

```
#[derive(Debug)]
struct Reactangle{
   width:u32,
   height:u32
}

impl Reactangle {
    // mutable self
    fn area(&mut self)->u32{
      self.width = 12;
      self.width * self.height
    }
}

fn main(){
    // mutable instance
   let mut ret1 = Reactangle{
      width:32,
      height:20
   };

   println!("{}",ret1.area());
}
```
this is an example method with multiple parameter

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

### Associate functions
Function withing `impl` (or methods) are also called Associate function. You can define associate function without `&self` parameter. These types of associate functions are not methods  These types function often used for construction. A constructor return new instance of its struct.

```
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

```
We have to use `::` syntax to called associate function

```
let sq = Rectangle::square(3);
```
The `square` function is namespaced by the `Reactangle` struct.

### Tuple struct

You can also create tuple struct, but unlike normal struct, a tuple struct properties does not have name, only types. You can access properties by using index values, just like normal tuple

```
fn main() {
   struct  Color(i32, i32, i32);
   let black = Color(0,0,0);
   println!("{} {} {}",black.0, black.1, black.2);
}

```

[Back to Top](#index)

## Debug

### Struct debug
Sometimes we print data to debug our code. But for the case of struct we cannot direct print data using `println!` macro. Macho uses different types of formatting to print data. By default it used `Display` format to print the data. `Display` format is good for displaying primitive values such as integer, float, and boolean. But it cannot display complex data such as struct. In this case we have to use `Debug` formatter. To use `Debug` we can use specifier `{:?}` or `{:#?}`. `{#?}` is more precise than `{:?}`. We also have to put `#derive(Debug)` at the top of the struct. for example
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

```
the result will be

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```
you can also use `dbg!` macro for debugging. However, one thing to note is `dbg!` takes ownership. So, use reference if you don't want to loose ownership

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```


[Back to Top](#index)
