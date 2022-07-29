# Day 3 -- Borrow Checker / Memory Management (7/29/22)

QA, then learn about borrowing / memory management.

## Overview

- QA
- Borrow Checker
- Memory Management with a Rust Lens

## Question and Answer

- Rust is pretty much a replacement for C, but for our purposes it wouldn't make much sense to start using C in a library.

## Notes

### Borrowing

- When a scope ends all of the variables in that scope are destroyed. This is part of the memory management
  - Rust garbage collector is "proactive"
- Passing a variable into a `fn` moves the scope, then is destroyed at the end of the scope unless it is returned
  - If you want to prevent a variable from being destroyed (to use it after the `fn` call) --> You must pass a reference into the function instead
  - A reference is designated by `&` so a function declaration would look like this:

```rust
// person is a reference and the reference is dropped at the end of the function
fn is_dylan(person: &String) -> bool {
    true;
}
```
  - References are read-only
  - Only one write version of a variable at a time
  - Generally you want to pass references into functions to prevent this error
- You can create scopes anywhere in rust using `{ }`

### Lifetime Variables and Returning References

Messy example:

```rust
fn main() {
    let name1 = String::from("Pranav");
    let bigger = {
        let name2 = String::from("Tiago");
        let bigger = bigger_string(&name1, &name2);
        bigger
    };
    println!("The bigger string is: {bigger}");
}

fn bigger_string<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

- What is wrong?
  - Since bigger is dependent on `name1` and `name2` the scope of `name2` is destroyed prematurely and `bigger` is not possible outside of the scope.
