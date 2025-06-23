# Rust Bootcamp 🦀

A comprehensive learning journey through Rust programming fundamentals, designed to build skills for developing substreams and blockchain applications.

## 🎯 Learning Objectives

This bootcamp covers essential Rust concepts through hands-on exercises:
- Basic syntax and data types
- Ownership, borrowing, and lifetimes
- Error handling with Results
- Traits and generics
- Function development patterns
- Memory safety principles

## 📚 Course Structure

### 1. Hello World (`1-hello-world/`)
**Concepts:** Basic syntax, functions, string handling
- Introduction to Rust syntax
- Function definitions and calls
- String types and ownership basics

### 2. Car Factory (`2-car-function/`)
**Concepts:** Structs, enums, pattern matching
- Custom data types with structs
- Enum definitions and usage
- Factory pattern implementation

### 3. Borrow Checker (`3-borrow-checker/`)
**Concepts:** Lifetimes, references, borrowing rules
- Understanding Rust's ownership system
- Working with references and lifetimes
- Solving borrow checker challenges

### 4. Rustlings (`4-rustlings/`)
**Concepts:** Interactive exercises
- Rust-specific problem-solving
- Common patterns and idioms

### 5. Results (`5-results/`)
**Concepts:** Error handling, Result type
- Rust's approach to error management
- Pattern matching with Results
- Propagating errors safely

### 6. Traits (`6-traits/`)
**Concepts:** Trait definitions, implementations
- Defining shared behavior
- Implementing traits for custom types
- Generic programming patterns

### 7. Practice Projects (`rust-practice/`)
**Concepts:** Applied learning
- Real-world coding scenarios
- Integration of multiple concepts

## 🚀 Getting Started

### Prerequisites
- [Rust installed](https://rustup.rs/) (latest stable version)
- Basic understanding of programming concepts
- Text editor or IDE with Rust support (VS Code + rust-analyzer recommended)

### Running Exercises

Each exercise is a separate Rust project. Navigate to any directory and run:

```bash
# Example: Running the hello world exercise
cd 1-hello-world
cargo run

# To check for compilation errors without running
cargo check

# To run tests (if available)
cargo test
```

### Suggested Learning Path

1. Start with `1-hello-world` to get familiar with basic Rust syntax
2. Progress through `2-car-function` to learn about data structures
3. Tackle `3-borrow-checker` to understand ownership (most challenging!)
4. Complete `5-results` for error handling patterns
5. Explore `6-traits` for advanced type system features
6. Apply everything in `rust-practice` projects

## 🔧 Development Setup

For the best learning experience:

1. **VS Code + Extensions:**
   - rust-analyzer (official Rust language server)
   - CodeLLDB (debugging support)
   - Better TOML (for Cargo.toml files)

2. **Cargo Tools:**
   ```bash
   # Install useful cargo extensions
   cargo install cargo-watch    # Auto-compile on file changes
   cargo install cargo-expand   # See macro expansions
   cargo install clippy         # Linting tool
   ```

3. **Running with auto-reload:**
   ```bash
   cargo watch -x run
   ```

## 📖 Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learning through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust Reference](https://doc.rust-lang.org/reference/) - Language specification
- [Substreams Documentation](https://substreams.streamingfast.io/) - Target application domain

## 🎯 Target Application: Substreams

This bootcamp prepares you for building [Substreams](https://substreams.streamingfast.io/), which are:
- Blockchain data indexing solutions
- High-performance streaming data processors
- Built on Rust for memory safety and performance
- Used for real-time blockchain analysis

## 🤝 Contributing

Found bugs or improvements? Contributions are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b fix/your-fix`)
3. Make your changes with clear commit messages
4. Submit a pull request with description of changes

## 📝 Notes

Each exercise directory contains:
- `src/main.rs` - Main exercise code
- `Cargo.toml` - Project configuration
- `NOTES.md` - Learning notes and explanations (where available)

Take time to read through the code comments and experiment with modifications to deepen your understanding!

---

*Happy learning! 🦀 The journey to Rust mastery begins with a single step.*