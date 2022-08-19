# Day 5 -- Results and Move semantics (7/22/22)

Second day of rust bootcamp. We will build a car factory.

## Overview

- Move semantics

## Notes

### Move Semantics / Borrowing

- Again, when you pass an unborrowed variable to a function, the function takes ownership of the variable.
- `rustup docs --std` to open the std docs
- `rustc --explain E0502` to see the error the compiler gives you
- `std` (standard) variables implement the trait copy
  - A trait is ...
  - Copying passes a copy, not ownership of a vairable
- `#[derive(Debug)]` aids debugging on a variable
- `#[derive(Copy, Clone)]` allows you to copy / clone the variable this precedes
- Giving a function mutability and a reference can change a variabele and have it reflected in the parent function
  - However, while it is borrowed. The parent function can not change it or it could cause a race condition.
  - And the compiler would not allow this to happen either
  - The function that the variable is in the scope of has exclusive access

## Referencing

- You can read the chapters on [Referencing and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) to understand this better and learn how `rust` prevents data races
- How to setup substreams development env https://substreams.streamingfast.io/developer-guide/overview



