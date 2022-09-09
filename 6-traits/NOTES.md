# Day 6 -- Traits (9/9/2022)

Learning about traits

## Overview

- Traits
- Generics intro

## Notes

### What are traits?

It is like an abastract from other programming languages. Traits can own methods that need to be implemented.

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}
```

The above example is the syntax for a simple trait that outlines the method `append_bar()`.

You can also have a default implementation within the `trait {}` syntax to return something no matter what 

You can create functions (not functions in the trait) that call the implementation `append_bar()`. (obvious) But the cool thing is that you can have multiple implementations for `AppendBar` and using mono morphisation you can make the other functions parameters generic by writing `parameter: impl AppendBar` and no matter what the variable is, it will compile if it implements `AppendBar`.

You can extend this further and include multiple implementations to be more generic.
