<p align="center">
  <h1 align="center">Ernos Programming Language</h1>
  <p align="center">A compiled language with plain-English syntax, unification-based type inference, garbage-collected memory with ownership-safety checks, C-level performance — and a compiler that compiles itself all the way down to a Rust-free, clang-only bootstrap.</p>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/Version-1.0.0-blue.svg" alt="Version"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="#the-test-matrix"><img src="https://img.shields.io/badge/Rust_suite-72%2F72-brightgreen.svg" alt="Rust suite 72/72"></a>
  <a href="#self-hosting--the-bootstrap"><img src="https://img.shields.io/badge/Self--hosted_parity-54%2F54-brightgreen.svg" alt="Self-hosted parity 54/54"></a>
  <a href="#self-hosting--the-bootstrap"><img src="https://img.shields.io/badge/Compile--error_gate-12%2F12-brightgreen.svg" alt="Rejection gate 12/12"></a>
  <a href="#self-hosting--the-bootstrap"><img src="https://img.shields.io/badge/Bootstrap-clang--only-success.svg" alt="Clang-only bootstrap"></a>
  <a href="#self-hosting--the-bootstrap"><img src="https://img.shields.io/badge/Fixpoint-byte--identical-success.svg" alt="Byte-identical fixpoint"></a>
  <a href="#platform-support"><img src="https://img.shields.io/badge/Platform-macOS%20%7C%20Linux-blueviolet.svg" alt="Platform"></a>
</p>

---

## What is Ernos?

Ernos is a **compiled, statically-typed, memory-safe programming language** that reads like plain English. It compiles to optimized native binaries via C with performance equivalent to hand-written C code.

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

### The headline: Ernos compiles itself, with no Rust in the loop

Ernos ships **two** complete compilers for the same language:

- **`ernos`** — the reference compiler, written in Rust (~30k lines).
- **`epc`** — the self-hosted compiler, **written entirely in Ernos** (`ep_lexer.ep`, `ep_parser.ep`, `ep_check.ep`, `ep_optimizer.ep`, `ep_codegen.ep`, `epc.ep`).

`epc` compiles **every one of the 54 runnable test programs**, rejects **all 12** compile-error tests through its own semantic checker, and — compiling its own source — reaches a **byte-identical fixpoint** (`gen2 == gen3`). A frozen C snapshot (`bootstrap/epc_bootstrap.c`) means the whole toolchain rebuilds from **clang alone** — no Rust, no `cargo`, no bootstrap chicken-and-egg. This is verified end-to-end on every change, **with zero disclosed caveats**.

> `clang bootstrap/epc_bootstrap.c -o epc && ./epc epc.ep` → a working compiler that recompiles itself and passes the full suite.

### See the whole language in one program

[`examples/bakery.ep`](examples/bakery.ep) — **The Plainville Bakery** — is a day-in-the-life simulation that exercises every major feature in ~300 lines of plain English: structs + methods, traits, enums + pattern matching, `try`/Result error handling, closures, a custom iterator, async ovens on the event loop, spawned worker threads with channels, floats, f-strings, maps, file I/O, SHA-256 receipts, and a plain-English insertion sort.

```bash
./target/release/ernos examples/bakery.ep && ./examples/bakery   # reference compiler
./epc examples/bakery.ep && ./examples/bakery                    # self-hosted compiler
```

Both compilers produce **byte-identical output** for it — CI proves that on every push.

```ernos
define sell on Pastry with qty as Int returning Outcome:
    if qty < 1:
        return Refused with "you have to buy at least one"
    if qty > self.stock:
        return Refused with "not enough in the case"
    set self.stock to self.stock - qty
    return Sold with self.price_cents * qty
```

---

## Why Ernos?

| Feature | Ernos | Rust | Java | Python |
|---------|-------|------|------|--------|
| **Readability** | ✅ Plain English | ❌ Symbolic | ❌ Verbose | ✅ Clean |
| **Type Safety** | ✅ Inferred + checked | ✅ Full | ✅ Full | ❌ Dynamic |
| **Memory Safety** | ✅ GC + ownership checks | ✅ Ownership | ⚠️ GC only | ❌ GC only |
| **Performance** | ✅ C-level | ✅ C-level | ⚠️ JVM overhead | ❌ Interpreted |
| **Compile Target** | Native binary | Native binary | JVM bytecode | Interpreted |
| **Self-Hosting** | ✅ | ✅ | ❌ | ❌ |

### Performance

Ernos compiles to C, then to a native binary via `clang -O2`. The generated code has no interpreter overhead, and numeric functions whose parameters are declared `Int`/`Float`/`Bool` pay **zero** GC bookkeeping — the compiler proves they can't touch the heap and strips the root registration entirely.

Measured on Apple Silicon (`fib(35)`, naive doubly-recursive):

| | user time |
|---|---|
| Ernos (`ernos fib.ep`) | **0.03 s** |
| C (`clang -O2 fib.c`) | **0.03 s** |

---

## Features

### 🛡️ Compile-Time Safety
- **Type inference with unification** — types are inferred even without annotations (HM-style unification; no let-generalization yet)
- **Enforced type checking** — declared return types, list-element types, and undefined names are hard errors that stop compilation
- **Ownership & borrowing analysis** — use-after-move, move-while-borrowed, modify-while-borrowed, and returning a borrow of a local are rejected (enforced in codegen, backed by the GC)
- **Send/Sync safety** — borrowed references cannot be sent to spawned threads

```ernos
define halve with n as Int returning Int:
    return "half"           # ✗ REJECTED: returns Str, but Int is declared

define main:
    set xs to [1, "two", 3] # ✗ REJECTED: list elements have conflicting types
    display mystery          # ✗ REJECTED: undefined name
    return 0
```

### ⚡ Performance
- Compiles to C, then to native binary via `clang -O2`
- Constant folding and dead code elimination at AST level
- Release mode: `clang -O3 -flto` (via `--release` flag)

### 📦 Standard Library (24 modules)

| Module | Description |
|--------|-------------|
| `string` | String functions, StringBuilder, formatting |
| `collections` | HashMap, HashSet, Stack, Queue, PriorityQueue |
| `fs` | File I/O, directories, path utilities |
| `net` / `http` | TCP sockets, HTTP client/server |
| `json` | JSON parsing and generation |
| `csv` | CSV parsing and generation |
| `datetime` | Timestamps, formatting, arithmetic |
| `crypto` | SHA256, MD5, SHA1, base64, UUID, random |
| `regex` | POSIX regex matching, find, replace, split |
| `sync` | Mutex, RWLock, Atomic, Barrier, Semaphore |
| `os` | Environment, process info, system commands |
| `test` | Assertions, test suites, test runner |
| `log` | Structured logging with levels and timestamps |
| `math` | Mathematical functions |
| `sort` | Sorting algorithms |
| `sql` | SQLite database bindings |
| `gui` | GUI via raylib |
| `hash` | Hashing utilities |
| `toml` | TOML config file parsing |
| `static_server` | Static file serving over HTTP |
| `websocket` | WebSocket protocol implementation |
| `select` | I/O multiplexing |
| `structured` | Structured concurrency (task groups, timeouts) |

### 🔌 FFI Bridge Libraries (29 bindings)

Pre-built bindings for C libraries via `ep_dlopen`/`ep_dlsym`/`ep_dlcall`:

`raylib` · `sdl2` · `ncurses` · `cairo` · `libpng` · `stb_image` · `stb_truetype` · `miniaudio` · `libsndfile` · `curl` · `openssl` · `libsodium` · `zlib` · `sqlite` · `jansson` · `expat` · `pcre` · `libgit2` · `libuv` · `lmdb` · `termbox2` · `portmidi` · `freetype` · `lua` · `mongoose` · `mosquitto` · `libnotify` · `libusb` · `chipmunk`

> **Note:** Stdlib modules are written in ErnosPlain and call into the compiler's C runtime. They require the corresponding C runtime functions to be available.

### 🔧 Developer Tools

| Tool | Command | Description |
|------|---------|-------------|
| **Compiler** | `ernos program.ep` | Compile to native binary |
| **REPL** | `ernos --repl` | Interactive evaluation with session state |
| **Formatter** | `ernos format file.ep` | Auto-format source code |
| **Checker** | `ernos check file.ep` | Type/syntax validation without compiling |
| **Test Runner** | `ernos test file.ep` | Run tests |
| **Builtins** | `ernos --list-builtins` | Show all built-in functions |
| **Debug** | `ernos file.ep --debug` | Compile with `-O0 -g` |
| **Release** | `ernos file.ep --release` | Compile with `-O3 -flto` |
| **ASAN** | `ernos file.ep --asan` | Compile with AddressSanitizer |
| **WASM** | `ernos file.ep --wasm` | Compile to WebAssembly |
| **Native** | `ernos file.ep --native` | Compile via native assembly (no Clang) |
| **LSP** | `ernos --lsp` | Language Server Protocol for editor support |
| **Doc Gen** | `ernos doc file.ep -o api.md` | Generate API documentation from doc comments |
| **Bind** | `ernos bind header.h` | Generate .ep bindings from C headers |
| **Transpile** | `ernos transpile file.py` | Translate Python/C/JS/Go/Rust/Ruby/Java/TS to EP |

### 🌍 Platform Support
- **macOS** (ARM64 + x86_64) — primary development platform
- **Linux** (x86_64 + aarch64, GCC or Clang) — supported

> **Note:** Windows has partial C runtime polyfills (`#ifdef _WIN32` blocks) but is not tested or officially supported yet.

---

## Quick Start

### Prerequisites
- A C compiler (`clang` or `gcc`)
- Rust (for building the bootstrap compiler)

### Build
```bash
git clone https://github.com/MettaMazza/Ernos-Programming-Language.git
cd Ernos-Programming-Language
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
define worker with id as Int and ch:
    send id * 10 to ch
    return 0

define main:
    set ch to channel
    spawn worker(1 and ch)
    spawn worker(2 and ch)

    set a to receive from ch
    set b to receive from ch
    display f"Total: {a + b}"
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

### Enums & Pattern Matching
```ernos
define choice Shape:
    variant Circle with radius as Int
    variant Rect with width as Int and height as Int

define area with s as Shape returning Int:
    check s:
        if Circle with r:
            return r * r * 3
        if Rect with w and h:
            return w * h

define main:
    set c to Circle with 5
    display f"Area: {area(c)}"
    return 0
```

### Namespace Imports
```ernos
import "math" as m

define main:
    set result to m_absolute(-42)
    display result    # 42
    return 0
```

### Dynamic Library Loading (FFI)
```ernos
define main:
    set lib to ep_dlopen("libm.dylib")
    set abs_fn to ep_dlsym(lib and "abs")
    set result to ep_dlcall1(abs_fn and -42)
    display result    # 42
    set _ to ep_dlclose(lib)
    return 0
```

### Cross-Language Transpilation
```bash
# Translate existing code from other languages into ErnosPlain
ernos transpile script.py -o script.ep     # Python → ErnosPlain
ernos transpile program.c -o program.ep    # C → ErnosPlain
ernos transpile app.js -o app.ep           # JavaScript → ErnosPlain
ernos transpile main.go -o main.ep         # Go → ErnosPlain
ernos transpile lib.rs -o lib.ep           # Rust → ErnosPlain
ernos transpile App.java -o App.ep         # Java → ErnosPlain
ernos transpile app.ts -o app.ep           # TypeScript → ErnosPlain
ernos transpile script.rb -o script.ep     # Ruby → ErnosPlain

# Generate .ep bindings from C headers
ernos bind /usr/include/math.h -o math_bindings.ep
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
  Type Checker (unification-based inference) — hard errors
    ↓
  Borrow Checker (ownership analysis) — hard errors
    ↓
  Optimizer (constant folding, dead code elimination)
    ↓
  Codegen → C source (includes ownership safety checks)
    ↓
  Clang -O2 → Native binary
```

> **Note:** The codegen phase performs additional ownership checks (use-after-move, borrow violations) as a safety net alongside the dedicated borrow checker. Both must pass for compilation to succeed.

### Reference compiler (Rust) — `~30,000` lines across 24 modules

| File | Lines | Description |
|------|-------|-------------|
| `src/lexer.rs` | 896 | Tokenizer with indentation tracking |
| `src/parser.rs` | 1,639 | Recursive-descent parser with Pratt precedence |
| `src/type_check.rs` | 1,987 | Type inference via unification (HM-style; no let-generalization) |
| `src/borrow_check.rs` | 783 | Ownership, borrowing, Send/Sync analysis |
| `src/optimizer.rs` | 1,577 | Constant folding, DCE, CSE, LICM, inlining, loop unrolling |
| `src/codegen.rs` | 3,958 | C code generation (runtime lives in `runtime/`, embedded via `include_str!`) |
| `src/llvm_codegen.rs` | 76 | LLVM IR backend (via `clang -emit-llvm`) |
| `src/lsp.rs` | 1,198 | Language Server Protocol implementation |
| `src/diagnostics.rs` | 382 | Rich error reporting with ANSI colors |
| `src/native_codegen.rs` | 656 | ARM64 native-assembly backend (macOS + Linux) |
| `src/x86_64_codegen.rs` | 623 | x86-64 native-assembly backend (macOS + Linux) |
| `src/bind_c.rs` | 1,441 | C-header binding generator (zero-dependency) |
| `src/main.rs` | 2,087 | CLI, imports, REPL, compilation pipeline |
| `src/transpile_py.rs` | 2,673 | Python → Ernos transpiler |
| `src/transpile_c.rs` | 1,376 | C → Ernos transpiler |
| `src/transpile_js.rs` | 1,235 | JavaScript → Ernos transpiler |
| `src/transpile_go.rs` | 1,362 | Go → Ernos transpiler |
| `src/transpile_rs.rs` | 1,211 | Rust → Ernos transpiler |
| `src/transpile_rb.rs` | 1,080 | Ruby → Ernos transpiler |
| `src/transpile_java.rs` | 731 | Java → Ernos transpiler |
| `src/transpile_ts.rs` | 725 | TypeScript → Ernos transpiler |
| `src/emit_c.rs` | 569 | Ernos → C emitter |
| `src/emit_js.rs` | 622 | Ernos → JavaScript emitter (enums → ES classes, trait-impl dispatch) |
| `src/emit_python.rs` | 640 | Ernos → Python emitter |
| **Total** | **~30,094** | |

### Shared C runtime — one source of truth, embedded by both compilers

| File | Lines | Description |
|------|-------|-------------|
| `runtime/ep_runtime.c` | 4,732 | Generational GC (precise STW + conservative stack scan, write barrier, OOM-guarded allocators), pointer-safe object accessors, coroutine/`EpFuture` scheduler, TCP/HTTP, SQLite, crypto, FFI |
| `runtime/ep_builtins.c` | 161 | Builtin registration glue |

The reference compiler embeds this via `include_str!`; the self-hosted compiler embeds the **byte-for-byte same source** through the generated `ep_runtime_gen.ep` (regenerate with `tools/gen_runtime_ep.ep`). Both compilers therefore emit the identical GC, accessors, and allocators — there is no "runtime drift" between them.

---

## Self-Hosting & the Bootstrap

This is the part most languages never finish. Ernos does — and proves it on every commit.

### The self-hosted compiler (`epc`) — written in Ernos, ~6,400 lines

| File | Lines | Description |
|------|-------|-------------|
| `ep_lexer.ep` | 817 | Lexer — indentation, f-string desugaring, English keyword aliases |
| `ep_parser.ep` | 1,449 | Parser — full grammar, `import "x" as alias`, traits, enums, closures |
| `ep_check.ep` | 301 | Semantic checker — reserved-name shadowing, Send-safety, list homogeneity, **enum-variant field-type checking** |
| `ep_optimizer.ep` | 122 | Constant folding + dead-code elimination |
| `ep_codegen.ep` | 3,342 | C code generator — closures, floats, traits, iterator protocol, `try`/Result, coroutine async, globals |
| `epc.ep` | 370 | Compiler driver — module flattening, aliased imports, `check`/`format`/`repl`/`doc` subcommands |
| `ep_runtime_gen.ep` | 5,070 | Generated: emits the shared C runtime verbatim |

The self-hosted pipeline is a full `lex → parse → **check** → **optimize** → codegen`, not just a lex/parse/emit skeleton.

### What "done" actually means here

| Gate | Result | Verified by |
|------|--------|-------------|
| Reference-compiler suite | **72 / 72** | `./run_tests.sh` |
| Self-hosted parity (runnable programs `epc` compiles + runs correctly) | **54 / 54** | `bash tests/run_epc_parity.sh` |
| Compile-error gate (programs `epc`'s checker must reject) | **12 / 12**, 0 wrongly accepted | `bash tests/run_epc_parity.sh` |
| Differential suite (both compilers agree on 39 adversarial programs) | **39 / 39** | `bash tests/run_differential.sh` |
| 3-stage self-compilation fixpoint (`gen2 == gen3`, byte-identical) | **OK** | `bash tests/run_fixpoint.sh` |
| Rust-free, clang-only bootstrap → recompile → fixpoint → full suite | **OK** | `bash bootstrap/verify.sh` |
| Cargo build warnings | **0** | `cargo build --release` |

The self-hosted checker is memory-safe under AddressSanitizer under **both** the Rust-built and the clang-built `epc` — the enum field-type pass, the last feature to land, was hardened against two distinct use-after-free classes (an aliased-sublist local and an unsound list-reassignment free) before it was accepted.

### Fully self-contained bootstrap — no Rust required

`bootstrap/epc_bootstrap.c` is `epc` compiled by `epc` (the fixpoint output). The whole toolchain rebuilds from a C compiler alone:

```bash
bash bootstrap/build.sh     # clang bootstrap/epc_bootstrap.c -> epc, then epc rebuilds epc.ep
bash bootstrap/verify.sh    # clang-only 3-stage fixpoint + parity suite + artifact-freshness check
```

`bootstrap/verify.sh` asserts the frozen C is **fresh** (matches the current `epc.ep`), so the artifact can never silently drift from source. Any change touching the self-hosted compiler must regenerate the bootstrap in the same commit.

The Rust compiler builds the same self-hosted compiler and remains the home of the two features intentionally kept Rust-only — the **LSP** and the **cross-language transpilers**:

```bash
./target/release/ernos epc.ep   # Rust compiler builds epc
./epc hello.ep && ./hello       # epc compiles programs — no Rust involved
```

---

## The Test Matrix

```bash
./run_tests.sh                 # reference (Rust) compiler:      72/72
bash tests/run_epc_parity.sh   # self-hosted: 54/54 runnable + 12/12 rejections
bash tests/run_differential.sh # both compilers agree on 39 adversarial programs
bash tests/run_fixpoint.sh     # 3-stage byte-identical fixpoint
bash bootstrap/verify.sh       # clang-only, Rust-free end-to-end proof
```

Every one of the 66 programs in `tests/` (54 runnable + 12 compile-error) is exercised by **both** compilers, and `tests/differential/` holds 39 adversarial programs (operator precedence, GC stress, closure corner cases, type-safety probes) on which the two compilers' compiled binaries must produce byte-identical output. Conformance tests live in [`conformance/`](conformance/); the formal grammar and type/memory/concurrency rules are in [`spec/ernos-spec.md`](spec/ernos-spec.md).

---

## Syntax Quick Reference

| Concept | Syntax |
|---------|--------|
| Variable | `set x to 42` |
| String | `"hello"` or `f"value: {x}"` |
| Function | `define foo with a and b:` |
| Typed param | `with a as Int and b as Str` |
| Return type | `define foo returning Int:` |
| If/else | `if cond:` ... `else:` |
| While loop | `repeat while cond:` or `while cond:` |
| For-each | `for each item in list:` |
| Comparison | `equals`, `is not equal to`, `<`, `>`, `<=`, `>=`, `==`, `!=` |
| Logical | `&&`, `\|\|`, `not`, `and also`, `or else` |
| Struct | `define structure Name:` with `field x as Type` |
| Enum | `define choice Name:` with `variant X` |
| Match | `check expr:` with `if Pattern:` |
| Method | `define foo on StructName:` |
| Struct create | `create StructName:` (block with `field is value`) |
| Channel | `set ch to channel` |
| Send | `send value to ch` |
| Receive | `set v to receive from ch` |
| Spawn | `spawn function(args)` |
| Try | `try expression` |
| List literal | `[1, 2, 3]` or `["a", "b"]` |
| Import | `import "module"` or `import "module" as alias` |
| Closure | `set f to given x: return x * 2` |
| Comment | `# this is a comment` |


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

MIT — free for anyone, for any purpose. See [LICENSE](LICENSE).

---

<p align="center">
  <b>Ernos</b> — Code that reads like English. Runs like C.
</p>
