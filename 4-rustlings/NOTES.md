# Day 4 -- Rustlings (8/5/22)

Fourth day of rust bootcamp. We will do some rustlings and focus on enums / structs.

## Overview

- Rustlings
- Enums
- Structs
- 

## Notes

- you can format text with `format!()` macro
- Scope and ownership is important in rust, and the developer must be mindful of
  - memory is destroyed after the scope of the variable is gone
  - This prevents data races when parallelizing
- Borrow checker- you can't move the value while it is being borrowed in another scope

### Structs

- structs can have methods in rust
  - Methods go in an implementation block
- These methods can reference `self` in the method declaration
  - ie, `fn is_blue(&self) -> bool`
  - by using `&` we are passing a reference so that `self` is not destroyed after the method is done, since the scope is complete.

### Memory

- 