# "the book" - *The Rust Programming Language*

I will use this directory to take `NOTES.md` and practice `rust` from "the book"

## About Rust

- A language capable of low-level efficiency/tasks designed to avoid the vulnerabilites/pitfalls of low-level languages
- `rustc` compiles the source code into a binary that anyone could run (even without an implementation of rust installed)
- You can use `cargo check` to check your code for errors without actually compiling it. Much faster

## Chapter 1

This chapter explains how to install / update rust, write a hello world program, use the compiler (`rustc`) directly, use `cargo` (the rust manager) properly.

## Chapter 3

I will skip chapter 2, read chapter 3, then go back to do the project in chapter 2.

Chapter 3 covers common coding practices in `rust`

### Variables and Mutability

- By default rust variables are immutable (cannot be changed)
  - You can make them mutable by adding `mut` to the declaration
- Constants are always immutable and you cannot add `mut` onto them
- Here is an example decleration of a constant

```rust
const ETHEREUM_DECIMALS: u8 = 18;
```

- *Shadowing* allows you to redeclare a variable with a new value
  - You would use this if you want to transform the variable a few times, but still indicate that it is immutable after the last declaration.
  - *Shadowing* creates a new variable so the type can also be changed, not possible by using `mut`

### Data Types

- Since rust is *statically typed* the compiler must know the type of every variable
- When a variable can have multiple types it is best to specify instead of letting rust guess:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

- Integer types: not fractional, can be signed or unsigned from 8-128 bits
- When compiling in debug mode rust checks for **integer overflow**
- When compiling in `--release` mode rust does *not* check for integer overflow
- Integer division rounds **down**
- chars are specified with single quotes `'` while strings use double quotes `"`
- Rust has 2 compound types: `tuples` and `arrays`
- Once a tuple is `declared` the size is static
  - Each position can have a different type

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("The value of the i32 is {tup.0}");
```

- Arrays have a fixed length in rust
  - Useful when you want data on the stack and not the heap
  - Use when the values on the array will not change
- Index out of bound errors are not detected during compile time
  - This runtime error is protective and will not return out of bounds memory

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- But *Vectors* are dynamic in length

### Functions

- `main()` is the entry point for rust programs