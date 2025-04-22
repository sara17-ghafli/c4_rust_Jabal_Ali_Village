# C4 Rust Compiler (Team Jabal_Ali_Village)

This project is a Rust-based reimplementation of the C4 compiler, originally written in C. It supports compiling a subset of C code including the original C4 source code (self-hosting).

## Features
- Lexer for tokenizing C source code
- Parser and virtual machine for C4's supported operations
- Uses Rust's safety features and modern syntax
- Unit tested with 70%+ coverage

## Getting Started

### Build
```bash
cargo build
```

### Run
```bash
cargo run < source.c
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
- src/c4.rs: Main source file (lexer, parser, VM)
- tests/: Unit tests for components
- c4_rust_comparison.pdf: Comparison report between C and Rust versions

## Bonus Feature (Optional)
TBD: To be updated if a bonus feature is implemented.

## Team Members
- Sara Alghafli (Lexer and initial parser setup)
- Asma Alhumairi (Parser, VM, and testing)
