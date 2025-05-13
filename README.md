# 🐒 Monkey Language Interpreter (Rust)

A production-grade, fully-featured interpreter for the [Monkey programming language](https://interpreterbook.com/), written in idiomatic Rust. This project is a faithful and extensible implementation of the Monkey language, featuring a REPL, parser, lexer, evaluator, built-in functions, and robust error handling.

---

## 🚀 Project Overview

This interpreter brings the Monkey language to life in Rust, following best practices for modularity, code quality, and documentation. It is designed for learning, hacking, and extending, and is suitable for both newcomers to interpreters and seasoned Rustaceans.

- **Language:** Rust 2021
- **Domain:** Interpreter, Programming Languages, REPL
- **Book Reference:** [Writing An Interpreter In Go](https://interpreterbook.com/)

---

## ✨ Features

- Complete Monkey language support (variables, functions, closures, arrays, hashes, conditionals, etc.)
- Interactive REPL (Read-Eval-Print Loop)
- Modular, production-grade Rust codebase
- Built-in functions (`len`, `first`, `last`, `rest`, `push`, `puts`)
- Robust error handling and clear error messages
- Comprehensive test suite
- Easy to extend with new features or built-ins

---

## 📂 Folder Structure

```
monkey-lang-interpreter-rust/
├── src/
│   ├── ast/         # Abstract Syntax Tree definitions
│   ├── builtins/    # Built-in Monkey functions
│   ├── evaluator/   # Evaluator (core interpreter logic)
│   ├── lexer/       # Lexer (tokenizer)
│   ├── object/      # Runtime objects and environment
│   ├── parser/      # Parser (syntax analysis)
│   ├── repl/        # REPL implementation
│   ├── token/       # Token definitions
│   └── main.rs      # Entry point
├── Cargo.toml       # Rust project manifest
├── README.md        # This file
└── ...
```

---

## 🏁 Getting Started

### Prerequisites
- [Rust toolchain](https://www.rust-lang.org/tools/install) (1.60+ recommended)

### 1. Clone the Repository
```sh
git clone https://github.com/enfinity/monkey-lang-interpreter-rust.git
```

### 2. Build the Project
```sh
cargo build --release
```

### 3. Run the REPL
```sh
cargo run
```

You should see:
```
Hello! This is the Monkey Programming language!
Feel free to type in the code
>> 
```
Start typing Monkey code and see it evaluated instantly!

### 4. Run the Test Suite
```sh
cargo test
```
All tests should pass, ensuring the correctness of the interpreter.

---

## 🛠️ Contributing

Contributions, bug reports, and feature requests are welcome! To contribute:
1. Fork the repository
2. Create a new branch (`git checkout -b feature/my-feature`)
3. Make your changes (please include tests and documentation)
4. Open a pull request

---

## 🙏 Acknowledgments

- Inspired by [Writing An Interpreter In Go](https://interpreterbook.com/) by Thorsten Ball
- Thanks to the Rust community for guidance and libraries

---

## 📣 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

Enjoy hacking on your own programming language in Rust! 🦀🐒