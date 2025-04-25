# C4 Rust Compiler (Team Jabal_Ali_Village)

This project is a Rust-based reimplementation of the original C4 compiler.  
It compiles a subset of C code including expressions, assignments, and return statements.

## Features
- Lexer with tokenizing and line/column tracking (bonus feature)
- Parser for expressions, assignments, and return statements
- Virtual machine that evaluates expressions
- Unit tested with 70%+ coverage
- Modular Rust code with clean idioms (ownership, pattern matching)
- Compatible with original C4's basic functionality

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
- `src/c4.rs`: main source file (Lexer, Parser, AST, VM)
- `src/main.rs`: reads and compiles C input from stdin
- `src/lib.rs`: module exports
- `tests/`: unit tests for lexer, parser and VM
- `examples/source.c`: example c program
- `c4_rust_comparison.pdf`: Comparison report between Rust and original C4

## Bonus Feature
each token includes line and column information, enabling better error messages in the future. This tracking improves debugging and syntax error reporting compared to the original C4 compiler.

## Supported Subset
return 1 + 2;
x = 5;
return x + 3;
Parentheses and operator precedence (*, /, +, -)

## Team Members
- Sara Alghafli 100060342– Lexer and initial parser setup, parser integration, tests, project integration
- Asma Alhumairi – Virtual Machine implementation, error tracking, bonus feature
