# ErnosPlain Programming Language

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform: macOS ARM64](https://img.shields.io/badge/Platform-macOS%20ARM64-blue.svg)](#)
[![Self-Hosted](https://img.shields.io/badge/Compiler-Self--Hosted-success.svg)](#)

**ErnosPlain** is a compiled, systems-level programming language with a syntax that reads like plain English. 

The language is **fully self-hosting (bootstrapped)**. The codebase includes both a reference compiler written in **Rust** (with zero external dependencies) and a production compiler written directly in **ErnosPlain**. Both compile ErnosPlain source code (`.ep` files) directly into native, standalone **ARM64 machine code** binaries for Apple Silicon (macOS) with automated scope-based memory cleanup (RAII).

---

## 🌟 Key Features

1. **Plain English Syntax**: Replaces standard curly brackets, parentheses, and algebraic operators with plain English words.
2. **Shorthand Mode**: Allows developers to mix standard mathematical symbols (`+`, `-`, `*`, `/`) and boolean operators (`==`, `!=`, `<`, `>`, `&&`, `||`) directly with English phrasing.
3. **Automated Memory Cleanup (RAII)**: Features compile-time scope tracking for dynamic lists and heap-allocated strings. The compiler automatically injects deallocations (`free`/`free_list`) when variables go out of scope or are reassigned.
4. **Self-Hosting (Bootstrapped)**: The compiler is written in ErnosPlain itself and can fully re-compile its own source code, achieving complete self-replication.
5. **Zero Interpreter/VM Dependencies**: Compiles directly to native macOS ARM64 Apple Silicon machine code. The output binaries run at hardware speed with flat memory profiles.
6. **Built-in Diagnostics**: Provides high-fidelity syntax error highlighting with context display and suggestions.
7. **VS Code Syntax Grammar**: Comes with an official TextMate syntax highlighting grammar extension.

---

## 📖 Language Syntax Tour

### 1. Variables & Math
```ernosplain
set a to 10
set b to 20
# Precedence-aware math evaluation: 10 + (20 * 2) = 50
set result to a plus b multiplied by 2
```

### 2. Conditionals & Logic
```ernosplain
if result is greater than 30 and also result is not equal to 100:
    display "The number is in the sweet spot!"
else:
    display "Out of bounds."
```

### 3. Loops (`while`)
```ernosplain
set count to 5
while count > 0:
    display count
    set count to count - 1
```

### 4. Dynamic Lists
```ernosplain
set numbers to create_list()
set ok to append_list(numbers and 10)
set ok to append_list(numbers and 20)

display get_list(numbers and 0)   # Prints 10
display length_list(numbers)       # Prints 2
# Lists are automatically deallocated when they go out of scope!
```

---

## 🛠️ Getting Started

### Prerequisites
* A macOS Apple Silicon machine (M1/M2/M3/M4)
* `clang` command-line tools installed (`xcode-select --install`)
* Rust installed (only if building the Rust bootstrap compiler)

### Quick Install
If you have cloned this repository, you can build, replicate, and globally install the self-hosted ErnosPlain compiler by running:
```bash
./install.sh
```

### Manual Build Instructions

If you prefer to compile and install the compiler step-by-step manually:

#### 1. Build the Rust Bootstrap Compiler
```bash
cargo build --release
cp target/release/ernosplain ./epc
```

### 2. Compile and Run a Test Program
Create a file `hello.ep`:
```ernosplain
define main:
    display "Hello from ErnosPlain!"
    set numbers to create_list()
    set ok to append_list(numbers and 42)
    display get_list(numbers and 0)
    return 0
```

Compile it using the compiler driver:
```bash
./epc hello.ep
```
This produces a native binary `./hello` (and automatically cleans up the temporary `.s` assembly and runtime C files). Run it:
```bash
./hello
```

---

## 🚀 Bootstrapping the Self-Hosted Compiler

ErnosPlain can compile its own compiler! The self-hosted compiler modules are located in the project root:
* `ep_lexer.ep`: Lexer module
* `ep_parser.ep`: Parser and S-expression AST generator
* `ep_codegen.ep`: ARM64 Assembly code generator
* `epc.ep`: Compiler driver

### How to Bootstrap
1. **Concatenate the self-hosted compiler modules** into a single source file:
   ```bash
   cat ep_lexer.ep ep_parser.ep ep_codegen.ep epc.ep > self_hosted_compiler.ep
   ```

2. **Compile the self-hosted compiler** using the Rust bootstrap compiler:
   ```bash
   ./epc self_hosted_compiler.ep
   ```
   This generates the native compiler executable `./self_hosted_compiler`.

3. **Verify Self-Replication (Generational Bootstrapping)**:
   Use the first-generation self-hosted compiler to compile its own source file to verify replication:
   ```bash
   cp ./self_hosted_compiler ./self_hosted_compiler_gen1
   ./self_hosted_compiler_gen1 self_hosted_compiler.ep
   ```
   This outputs the second-generation `./self_hosted_compiler` binary. 

4. **Verify correctness**:
   Use the new self-compiled binary to compile any program (e.g., `hello.ep`):
   ```bash
   ./self_hosted_compiler hello.ep
   ./hello
   ```

---

## 🎨 VS Code Syntax Highlighting

To enable syntax highlighting in Visual Studio Code:
1. Copy the `ernosplain-syntax` folder to your VS Code extensions directory:
   ```bash
   cp -R ernosplain-syntax ~/.vscode/extensions/
   ```
2. Restart VS Code. All `.ep` files will now render with semantic colors!

---

## 📚 Complete Guides

* [LANGUAGE_REFERENCE.md](file:///Users/mettamazza/Desktop/ErnosPlain%20Programing%20Language/LANGUAGE_REFERENCE.md): Detailed token mappings, operators, and shorthand grammar.
* [Coding in ErnosPlain Guide / Bible](file:///Users/mettamazza/.gemini/antigravity/brain/baa39659-e677-45d8-bf01-98d4631ee81c/coding_in_ernosplain_guide.md): The comprehensive guide for beginners and systems programming experts covering stack frames, the RAII engine, optimization tricks, and codebase structures.

---

## 📄 License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
