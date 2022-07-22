# Day 2 -- Building a Car Factory (7/22/22)

Second day of rust bootcamp. We will build a car factory.

## Overview

- Using rust in a real application
- Some syntax
- Functions, enums, structs

## Notes

### Strings

```rust
let string_slice = "Hello World"; // this is immutable
let mut owned_string = String::from("Hello World"); // this is mutable
let comma_char = ','; // this is character
```

> characters use single quotes ('), string literals use double quotes (")

```rust
owned_string.push(comma_char); // this becomes "Hello World,"
```

- You cannot `.push()` to a `&str` because it is immutable
- But you can `&str.replace(old, new)` with string slices
  - old replaces all ocurrences of `old` with `new`
- 

### Booleans

```rust
let is_now_2022 = true; // this is a boolean

// this is a conditional 
if is_now_2022 {
    println!("It is now 2022!");
} else {
    println!("It is not 2022!");
}
```

- Another example with conditional statements

```rust
let year: u32 = 2022;
if year == 2022 {
    println!("It is now 2022!");
} else if year == 2023 {
    printlin("You are now in the future!");
} else {
    println!("It is not 2022!");
}
```

### Tuples

Always have a fixed size and objects are accessed by position

```rust
let point2d = (0, 0);
println!("This is a point! ({}, {})", point2d.0, point2d.1);
```
```

### Resources

- The rust docs are the BEST type `rustup docs` in cli
- [Car Factory Tutorial](https://docs.microsoft.com/en-us/learn/modules/rust-create-program/7-exercise?ns-enrollment-type=learningpath&ns-enrollment-id=learn.languages.rust-first-steps)
