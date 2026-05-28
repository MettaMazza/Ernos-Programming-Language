# AGENT.md — AI Coding Agent Guidance for ErnosPlain

## Project Identity

ErnosPlain (Ernos) is a compiled, statically-typed, memory-safe programming language that reads like plain English. It compiles to native binaries via C transpilation. The goal is the best universal coding language that any person can read and write — code that looks like structured instructions, not cryptic symbols.

**Ernos must always be self-hosting and self-compiling.** The self-hosted compiler (`epc.ep` + `ep_lexer.ep` + `ep_parser.ep` + `ep_codegen.ep`) is not optional. It is the proof that the language works. If the self-hosted compiler cannot compile itself, the language is broken. Every change to the Rust bootstrap compiler must be validated against the self-hosted compiler.

---

## Anti-Hallucination Rules

These rules exist because AI agents hallucinate. They invent APIs that don't exist, claim features work when they don't, and generate code that compiles but doesn't do what was asked. Every rule here is a guardrail against that.

### 1. Never claim a feature works without running it

```
WRONG: "This should work" / "This will compile"
RIGHT: Compile it. Run it. Show the output. Then say it works.
```

If you change `codegen.rs`, you MUST:
1. `cargo build --release` — zero warnings
2. Write a `.ep` test file that exercises the change
3. Compile and run the test: `cargo run -- test.ep && ./test`
4. Verify the output matches expectations
5. Run `./run_tests.sh` — all tests pass

### 2. Never invent ErnosPlain syntax

The language has a specific grammar defined in `spec/ernos-spec.md` and implemented in `src/parser.rs`. If you write example ErnosPlain code, verify it parses:

```bash
cargo run -- example.ep 2>&1 | head -5
```

Common agent mistakes:
- Using `,` instead of `and` between arguments
- Using `{` / `}` instead of indentation
- Using `var` or `let` instead of `set ... to`
- Using `fn` or `func` instead of `define`
- Using `print` or `println` instead of `display`
- Using `match` instead of `check`
- Inventing functions like `len()`, `push()`, `print()` that don't exist (the real names are `length_list`, `append_list`, `display`)

### 3. Never claim self-hosting works without proving it

```bash
# This is the self-hosting gate. It must pass.
cargo run -- epc.ep && ./epc tests/test_core.ep && ./test_core
```

If you change the type checker, codegen, or parser in ways that affect the language semantics, you MUST run this gate. The self-hosted compiler is the canary.

### 4. Never approximate — read the source

When you need to understand how something works, read the actual source files. Do not guess from function names or documentation. The documentation may be stale. The source code is truth.

Key files and what they contain:
| File | What's in it |
|---|---|
| `src/lexer.rs` | Tokenizer. Every keyword, every symbol, every token type. |
| `src/token.rs` | Token enum definition. If a keyword isn't here, it doesn't exist. |
| `src/ast.rs` | AST node types. If a node isn't here, the parser can't produce it. |
| `src/parser.rs` | What syntax is valid. The parser is the grammar. |
| `src/type_check.rs` | Type inference. Builtin function signatures. What types exist. |
| `src/borrow_check.rs` | Ownership analysis. What is tracked, what is moved. |
| `src/codegen.rs` | C code generation. The C runtime. Every builtin function's implementation. This is the largest and most critical file. |
| `src/main.rs` | CLI, flags, import resolution, REPL, the compilation pipeline. |
| `src/native_codegen.rs` | ARM64 native backend (macOS only currently). |
| `src/x86_64_codegen.rs` | x86_64 native backend (macOS + Linux). |

### 5. Never change the runtime without understanding the memory model

The C runtime is embedded in `codegen.rs` as string literals. It manages:
- **GC root stack**: Thread-local shadow stack (`__thread`). Each thread has its own.
- **Thread registry**: Tracks all spawned threads so GC mark can walk all root stacks.
- **Channels**: Mutex-protected circular buffer with condvars for send/receive.
- **Closures**: `EpClosure` struct with magic number for dispatch (function pointer vs closure).
- **Structs/Enums**: Heap-allocated, GC-registered, with generated `free_` functions.

Everything is `long long` (64-bit). Pointers, ints, booleans — all `long long`. This is intentional. It means the type system is a compile-time overlay on a unityped runtime.

### 6. Never add dependencies without justification

The Rust bootstrap compiler has minimal dependencies (just the standard library). The generated C code has no dependencies beyond libc and pthreads. Keep it that way. The language must compile on any system with a C compiler.

---

## Design Principles

### Plain English First
Every language construct should read like a sentence a non-programmer could understand:
- `set score to 85` not `int score = 85;`
- `if score is greater than 80:` not `if (score > 80) {`
- `repeat while count is less than 10:` not `while (count < 10) {`
- `for each item in list:` not `for (auto& item : list) {`

Symbol shortcuts (`+`, `<`, `==`, `&&`) are allowed as opt-in shorthands for experienced programmers. The plain English form is always the primary syntax.

### Self-Hosting is Non-Negotiable
The self-hosted compiler (`epc.ep` + modules) must always compile itself using the Rust bootstrap compiler. This is the ultimate integration test. If the type checker rejects the self-hosted compiler, the type checker is too strict — not the self-hosted compiler is wrong. The self-hosted compiler is 4,534 lines of real, working ErnosPlain. It is the language's own dogfood.

### Cross-Platform by Default
Ernos must work on:
- macOS ARM64 (Apple Silicon) — primary development platform
- macOS x86_64 (Intel Mac)
- Linux x86_64
- Linux aarch64

Both the C backend (default) and native assembly backends (`--native`) must produce correct binaries on all four targets. If a change works on macOS but breaks Linux, the change is broken.

### The C Runtime is the Language
The C runtime embedded in `codegen.rs` defines what the language can actually do at runtime. It is not a library — it is part of the compiler output. Every `.ep` program gets a copy of the runtime compiled into its binary. This means:
- Runtime changes affect ALL programs
- Runtime bugs affect ALL programs
- Runtime performance is language performance

### Tests Are Evidence, Not Decoration
Every feature, every bugfix, every behavioral change needs a test file in `tests/`. The test format:
- `tests/test_feature_name.ep` — the test program
- `tests/test_feature_name.expected` — the exact expected stdout (optional, for deterministic tests)

The `run_tests.sh` script compiles and runs every test. It is the regression gate.

---

## Compilation Pipeline

```
Source (.ep)
    ↓
Lexer (lexer.rs) → Token stream
    ↓
Parser (parser.rs) → AST
    ↓
Type Checker (type_check.rs) → Type-annotated AST (errors = hard stop)
    ↓
Borrow Checker (borrow_check.rs) → Ownership validation (warnings, not errors currently)
    ↓
Optimizer (optimizer.rs) → Constant folding, dead code elimination
    ↓
Codegen (codegen.rs) → C source code (includes full runtime inline)
    ↓
Clang/GCC → Native binary

Alternative: --native flag
    Codegen → ARM64 or x86_64 assembly → system assembler → system linker → native binary
```

---

## Value Representation

Everything is `long long` at the C level. The type system is a compile-time overlay:

| Ernos Type | Runtime Value | How to interpret |
|---|---|---|
| `Int` | Raw 64-bit integer | Used directly |
| `Bool` | 0 or 1 | Used as integer |
| `Str` | Pointer to `const char*` cast to `long long` | Cast back to `char*` for string ops |
| `DynStr` | Pointer to `malloc'd char*` cast to `long long` | Same, but freed at scope exit |
| `List` | Pointer to `EpList*` cast to `long long` | Cast back for list ops |
| `Struct(Name)` | Pointer to `EpStruct_Name*` cast to `long long` | Cast back for field access |
| `Enum(Name)` | Pointer to `EpEnum_Name*` cast to `long long` | Cast back, check tag for variant |
| `Closure` | Pointer to `EpClosure*` cast to `long long` | Magic number distinguishes from raw function ptr |

This means `get_list(list and 0)` returns a `long long` that could be an int, a string pointer, or a list pointer. The type checker tracks what it is at compile time, but at runtime it's just bits.

---

## Syntax Quick Reference for Agents

```ernos
# Variables
set x to 42
set name to "Alice"

# Functions  
define add with a as Int and b as Int returning Int:
    return a + b

# Function calls — args separated by 'and', not commas
set result to add(10 and 20)

# Bare calls (no return capture needed)
append_list(items and "value")

# If/else
if x > 10:
    display "big"
else if x > 5:
    display "medium"
else:
    display "small"

# Loops
repeat while x < 100:
    set x to x + 1

for each item in items:
    display item

# Structs
define structure User:
    field name as Str
    field age as Int

set user to create User:
    name is "Alice"
    age is 30

display user.name

# Enums
define choice Shape:
    variant Circle with radius as Int
    variant Square with side as Int

set s to Circle with 5
check s:
    if Circle with r:
        display r
    if Square with side:
        display side

# Methods (on structs or enums)
define greet on User:
    display self.name
    return 0

set ok to user.greet()

# Closures (lambdas)
set double to given x:
    return x * 2

# Higher-order functions
define apply with f and x as Int returning Int:
    return f(x)
set result to apply(double and 5)

# Concurrency
set ch to create_channel()
spawn worker(ch)
set v to receive from ch
send 42 to ch

# Imports
import "string"
import "fs"

# F-strings
display f"Hello {name}, you are {age} years old"

# Lists
set items to create_list()
append_list(items and 10)
display get_list(items and 0)
display length_list(items)

# Maps
set m to create_map()
map_insert(m and "key" and 42)
display map_get_val(m and "key")

# Comments
# This is a comment
```

---

## Builtin Functions Reference

These are implemented as C functions in the runtime (codegen.rs). They are NOT ErnosPlain functions — they are compiler intrinsics. Do not try to redefine them.

### Lists
`create_list()`, `append_list(list and value)`, `get_list(list and index)`, `set_list(list and index and value)`, `length_list(list)`, `pop_list(list)`, `remove_list(list and index)`, `free_list(list)`

### Maps
`create_map()`, `map_insert(map and key and value)`, `map_get_val(map and key)`, `map_get_str(map and key)`, `map_set_str(map and key and value)`, `map_contains(map and key)`, `map_delete(map and key)`, `map_keys(map)`, `map_values(map)`, `map_size(map)`, `free_map(map)`

### Strings
`string_length(s)`, `substring(s and start and len)`, `string_concat(a and b)` (alias: `concat`), `int_to_string(n)`, `string_to_int(s)`, `string_contains(s and sub)`, `string_index_of(s and sub)`, `string_replace(s and old and new)`, `string_split(s and delim)`, `string_upper(s)`, `string_lower(s)`, `string_trim(s)`, `char_at(s and index)`, `get_character(s and index)` (returns ASCII code)

### I/O
`display`, `read_file_content(path)`, `write_file_content(path and content)`, `file_append(path and content)`

### Concurrency
`create_channel()`, `send ... to ...`, `receive from ...`, `spawn`

### Math / System
`random_range(min and max)`, `time_now()`, `sleep_ms(ms)`, `exit(code)`

---

## Verification Protocol

Before committing ANY change, run this sequence:

```bash
# 1. Rust build — zero warnings
cargo build --release

# 2. All tests pass
./run_tests.sh

# 3. Self-hosting gate (when type checker/codegen/parser changed)
cargo run -- epc.ep && ./epc tests/test_core.ep && ./test_core

# 4. Native backend gate (when native codegen changed)
cargo run -- tests/test_core.ep --native && ./test_core
```

If any step fails, the change is not ready. Fix it before committing.

---

## Known Constraints

1. **All values are `long long`** — no floats at runtime yet (Float is in the spec but not fully implemented in codegen)
2. **Single-file compilation** — the import system flattens everything into one C file
3. **No generics** — the type system is monomorphic (parametric polymorphism via `MonoType::Any` for container returns)
4. **No closures over mutable state** — closures capture by value at creation time
5. **GC is stop-the-world** — all threads pause during collection (protected by `ep_gc_mutex`)
6. **Thread limit** — maximum 256 concurrent threads (`EP_MAX_THREADS`)
7. **No Windows support** — partial `#ifdef _WIN32` polyfills exist but are untested

---

## File Naming Conventions

- Source files: `snake_case.ep`
- Test files: `tests/test_feature_name.ep` with optional `tests/test_feature_name.expected`
- Stdlib modules: `stdlib/module_name.ep`
- Self-hosted compiler: `epc.ep`, `ep_lexer.ep`, `ep_parser.ep`, `ep_codegen.ep`
- Generated C: `filename_compiled.c` (temporary, cleaned up by compiler)
- Generated binary: `./filename` (same stem as source)

---

## When Making Changes

### Adding a new builtin function
1. Register the type signature in `type_check.rs` (`register_builtins`)
2. Implement the C function body in `codegen.rs` (in the runtime emission section)
3. Register it in `codegen.rs` `func_return_types` map
4. Add it to the `is_builtin_or_runtime_func` filter
5. Write a test in `tests/`
6. Document it in this file

### Adding new syntax
1. Add token(s) in `token.rs`
2. Add lexer rule(s) in `lexer.rs`
3. Add AST node(s) in `ast.rs`
4. Add parser rule(s) in `parser.rs`
5. Add type checking in `type_check.rs`
6. Add borrow checking in `borrow_check.rs` (if heap-allocating)
7. Add C codegen in `codegen.rs`
8. Add native codegen in `native_codegen.rs` and `x86_64_codegen.rs` (or return unsupported error)
9. Write tests, update spec

### Modifying the type checker
- The type checker uses Hindley-Milner unification in `type_check.rs`
- `MonoType` is the core type enum
- `Substitution` maps type variables to concrete types
- `unify()` is the unification function
- Changing type signatures of builtins can cascade — always run self-hosting gate

### Modifying the GC / runtime
- The GC root stack is `__thread` (thread-local)
- `ep_gc_push_root` / `ep_gc_pop_roots` are per-thread
- `ep_gc_mark` walks ALL threads' root stacks (under `ep_thread_registry_mutex`)
- `ep_gc_maybe_collect` is called after every N allocations
- If you change the runtime, you change the behavior of ALL compiled programs
