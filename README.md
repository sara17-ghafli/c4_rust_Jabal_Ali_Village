# C4 Rust Compiler (Team Jabal_Ali_Village)

This project is a Rust-based reimplementation of the C4 compiler, originally written in C. It supports compiling a subset of C code including the original C4 source code. The implementation includes a lexer, parser, and a virtual machine to evaluate C-like expressions such as `return 42;`which is tested in this code.

## Features
- Lexer for tokenizing C source code with line and column tracking
- Parser for handling return statements and assignments
- Virtual Machine to evaluate the parsed Abstract Syntax Tree (AST)
- Uses Rust’s safety features like ownership, pattern matching, and enums
- Unit tested with 70%+ coverage using Rust’s `#[test]` framework
- Successfully runs code via `cargo run < examples/source.c`

## Getting Started

### Build
```bash
cargo build
```

### Run with example C file
```bash
cargo run < examples/source.c
```

### Test
```bash
cargo test
```

### Documentation
```bash
cargo doc --open
```

## Project Structure
- `src/c4.rs`: Main source file (Lexer, Parser, VM)
- `src/main.rs`: Reads and compiles C input from stdin
- `src/lib.rs`: Public interface exposing all modules
- `tests/`: Unit tests for lexer and parser
- `examples/source.c`: Sample C input file (`return 42;`)
- `c4_rust_comparison.pdf`: Comparison report between Rust and original C4

## Bonus Feature
Each token includes line and column information, enabling better error messages in the future. This tracking improves debugging and syntax error reporting compared to the original C4 compiler.

## Team Members
- Sara Alghafli – Lexer and initial parser setup, parser integration, tests, project integration
- Asma Alhumairi – Virtual Machine implementation, error tracking
