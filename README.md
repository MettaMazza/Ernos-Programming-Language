<p align="center">
  <h1 align="center">Ernos Programming Language</h1>
  <p align="center">A production-grade compiled language with plain English syntax, Hindley-Milner type inference, ownership-based memory safety, and C-identical performance.</p>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/Version-1.0.0-blue.svg" alt="Version"></a>
  <a href="#"><img src="https://img.shields.io/badge/Status-Production--Ready-brightgreen.svg" alt="Status"></a>
  <a href="#"><img src="https://img.shields.io/badge/Performance-C--Identical-orange.svg" alt="Performance"></a>
  <a href="#"><img src="https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-blueviolet.svg" alt="Platform"></a>
  <a href="#"><img src="https://img.shields.io/badge/Compiler-Self--Hosted-success.svg" alt="Self-Hosted"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
</p>

---

## What is Ernos?

Ernos is a **compiled, statically-typed, memory-safe programming language** that reads like plain English. It compiles to optimized native binaries via C with performance identical to hand-written C code.

```ernos
define factorial with n as Int returning Int:
    if n < 2:
        return 1
    return n * factorial(n - 1)

define main:
    display "Factorial of 20:"
    display factorial(20)
    return 0
```

**No curly braces. No semicolons. No noise.** Just code that reads like instructions.

---

## Why Ernos?

| Feature | Ernos | Rust | Java | Python |
|---------|-------|------|------|--------|
| **Readability** | ✅ Plain English | ❌ Symbolic | ❌ Verbose | ✅ Clean |
| **Type Safety** | ✅ HM Inference | ✅ Full | ✅ Full | ❌ Dynamic |
| **Memory Safety** | ✅ Ownership + GC | ✅ Ownership | ⚠️ GC only | ❌ GC only |
| **Performance** | ✅ C-identical | ✅ C-identical | ⚠️ JVM overhead | ❌ Interpreted |
| **Compile Target** | Native binary | Native binary | JVM bytecode | Interpreted |
| **Self-Hosting** | ✅ | ✅ | ❌ | ❌ |

### Performance Proof

```
fib(40) benchmark:
  Ernos:  0.28s  ← compiled with clang -O2
  C:      0.29s  ← compiled with clang -O2
  Ratio:  1.0x   ← identical performance
```

---

## Features

### 🛡️ Compile-Time Safety
- **Hindley-Milner type inference** — types are inferred even without annotations
- **Enforced type checking** — type errors stop compilation (not warnings)
- **Ownership & borrowing** — use-after-move, move-while-borrowed detection
- **Send/Sync safety** — borrowed references cannot be sent to threads

```ernos
define foo with x:
    display x + 1       # type checker infers x must be Int
    return 0

define main:
    set ok to foo("hello")  # ✗ REJECTED: expected Int, found Str at line 5:15
    return 0
```

### ⚡ Performance
- Compiles to C, then to native binary via clang -O2
- Smart GC safepoints — pure functions skip garbage collection overhead
- Constant folding and dead code elimination at AST level

### 📦 Comprehensive Standard Library (19 modules)

| Module | Description |
|--------|-------------|
| `string` | 40+ string functions, StringBuilder, formatting |
| `collections` | HashMap, HashSet, Stack, Queue, PriorityQueue |
| `fs` | File I/O, directories, path utilities |
| `net` / `http` | TCP sockets, HTTP client |
| `json` | JSON parsing and generation |
| `csv` | CSV parsing and generation |
| `datetime` | Timestamps, formatting, arithmetic, stopwatch |
| `crypto` | SHA256, MD5, base64, UUID, random |
| `regex` | POSIX regex matching, find, replace, split |
| `sync` | Mutex, RWLock, Atomic, Barrier, Semaphore, CondVar |
| `os` | Environment, process info, system commands |
| `test` | Assertions, test suites, test runner |
| `log` | Structured logging with levels and timestamps |
| `math` | Mathematical functions |
| `sort` | Sorting algorithms |
| `sql` | SQLite database bindings |
| `gui` | GUI via raylib |
| `hash` | Hashing utilities |

### 🔧 Developer Tools

| Tool | Command | Description |
|------|---------|-------------|
| **Compiler** | `ernos program.ep` | Compile to native binary |
| **REPL** | `ernos --repl` | Interactive evaluation |
| **Formatter** | `ernos --format file.ep` | Auto-format source code |
| **Checker** | `ernos --check file.ep` | Syntax validation without compiling |
| **Package Manager** | `epm init/build/run/test` | Project management (written in Ernos) |

### 🌍 Cross-Platform
- **macOS** (ARM64 + x86_64) — primary platform
- **Linux** (GCC/Clang) — full support
- **Windows** (MSVC/MinGW) — C runtime polyfills included

---

## Quick Start

### Prerequisites
- A C compiler (`clang` or `gcc`)
- Rust (for building the bootstrap compiler)

### Install
```bash
git clone https://github.com/YOUR_USERNAME/ernos-programming-language.git
cd ernos-programming-language
cargo build --release
```

### Hello World
```ernos
# hello.ep
define main:
    display "Hello from Ernos!"
    return 0
```

```bash
./target/release/ernos hello.ep
./hello
# Output: Hello from Ernos!
```

### Typed Functions
```ernos
define add with a as Int and b as Int returning Int:
    return a + b

define greet with name as Str:
    display concat("Hello, " and name)
    return 0

define main:
    display add(10 and 20)      # 30
    set ok to greet("World")    # Hello, World
    return 0
```

### Concurrency
```ernos
define worker with id as Int:
    display concat("Worker " and int_to_string(id))
    return 0

define main:
    spawn worker(1)
    spawn worker(2)
    spawn worker(3)
    return 0
```

### Structs & Methods
```ernos
define structure User:
    field name as Str
    field age as Int

define greet on User:
    display concat("Hi, I'm " and self.name)
    return 0

define main:
    set user to create User:
        name is "Alice"
        age is 30
    set ok to user.greet()
    return 0
```

---

## Architecture

```
Source (.ep)
    ↓
  Lexer → Tokens
    ↓
  Parser → AST
    ↓
  Type Checker (Hindley-Milner inference) — hard errors
    ↓
  Borrow Checker (ownership analysis) — hard errors
    ↓
  Optimizer (constant folding, dead code elimination)
    ↓
  Codegen → C source
    ↓
  Clang -O2 → Native binary
```

### Compiler Modules

| File | Lines | Description |
|------|-------|-------------|
| `src/lexer.rs` | 700 | Tokenizer with indentation tracking |
| `src/parser.rs` | 1,300 | Recursive descent parser with Pratt precedence |
| `src/type_check.rs` | 1,100 | Hindley-Milner type inference with unification |
| `src/borrow_check.rs` | 490 | Ownership, borrowing, Send/Sync analysis |
| `src/optimizer.rs` | 200 | AST-level constant folding and DCE |
| `src/codegen.rs` | 4,000 | C code generation with full runtime |
| `src/diagnostics.rs` | 285 | Rich error reporting with ANSI colors |

---

## Self-Hosting

Ernos compiles its own compiler. The self-hosted compiler modules:

- `ep_lexer.ep` — Lexer (30K)
- `ep_parser.ep` — Parser (26K)  
- `ep_codegen.ep` — Code generator (178K)
- `epc.ep` — Compiler driver (8K)

### Bootstrap
```bash
cat ep_lexer.ep ep_parser.ep ep_codegen.ep epc.ep > self_hosted_compiler.ep
./target/release/ernos self_hosted_compiler.ep
./self_hosted_compiler hello.ep
./hello
```

---

## Language Specification

A formal specification is available in [`spec/ernos-spec.md`](spec/ernos-spec.md), including:
- Complete EBNF grammar
- Type system rules
- Memory model (ownership, borrowing, GC)
- Concurrency model (Send/Sync)
- Standard library contracts

Conformance tests are in the [`conformance/`](conformance/) directory.

---

## VS Code Syntax Highlighting

```bash
cp -R ernosplain-syntax ~/.vscode/extensions/
# Restart VS Code — all .ep files will have syntax highlighting
```

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <b>Ernos</b> — Code that reads like English. Runs like C.
</p>
