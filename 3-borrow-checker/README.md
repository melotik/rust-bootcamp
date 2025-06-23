# Borrow Checker example – fixed

This small program demonstrates how Rust's borrow checker enforces lifetimes for references.

The original version tried to return a reference (`&String`) that pointed to `name2`, a value that went out of scope at the end of an inner block. The compiler rightfully rejected the code because such a reference would have been dangling.

The corrected version simply declares **both** strings in the same scope so that they live long enough for the returned reference to remain valid.

Run it with:

```bash
cd 3-borrow-checker
cargo run
```

You should see:

```
The bigger string is: Pranav
```

---

Key take-aways:

* A reference can never outlive the value it points to.
* Keep referenced values in a scope that is at least as long as the references you hand out.
