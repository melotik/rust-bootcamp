# Day 1 -- Basics (7/15/22)

First day of rust bootcamp covering the basics and setup coding enviornment.

## Overview

- Why rust?
- Setup coding env
- if/else
- Memory management

## Notes

- Wrote hello world program in rust
- Create rust application `cargo init hello`
- Run `cargo run` in project folder, this starts at main.rs/main()
- Build `cargo build` just builds, you get a target folder containing binary executable for program
- print function (also a macro *ie, dynamic params*) println!(“hello world”);  
- Function vs macro
  - macros are not made often
  - used with crates and more flexible with dynamic arguments
  - Functions have a set number of arguments
- You choose the bit size you need for number variables, allowing you to save space
- `rustc` is the compiler, `cargo` calls rustc
- Errors are documented very well


### Resources

- [Microsoft Rust Guide](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)
- [Install rust](https://rustup.rs/)
- [rust-analyzer](https://rustup.rs/)
- [Rust playground](https://play.rust-lang.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Next Lesson](hhttps://docs.microsoft.com/en-us/learn/modules/rust-create-program/7-exercise?ns-enrollment-type=learningpath&ns-enrollment-id=learn.languages.rust-first-steps)
