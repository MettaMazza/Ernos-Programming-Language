# Ernos Language Reference Manual

**Version 1.0.0** — Production Release

Ernos is a statically-typed, compiled programming language with plain English syntax, Hindley-Milner type inference, ownership-based memory safety, and native code generation via C.

---

## Table of Contents

1. [File Structure & Indentation](#1-file-structure--indentation)
2. [Variables & Type Annotations](#2-variables--type-annotations)
3. [Operators](#3-operators)
4. [Control Flow](#4-control-flow)
5. [Functions](#5-functions)
6. [Structs & Methods](#6-structs--methods)
7. [Enums & Pattern Matching](#7-enums--pattern-matching)
8. [Concurrency](#8-concurrency)
9. [Ownership & Borrowing](#9-ownership--borrowing)
10. [Closures & Higher-Order Functions](#10-closures--higher-order-functions)
11. [Error Handling](#11-error-handling)
12. [Standard Library](#12-standard-library)
13. [Compilation](#13-compilation)

---

## 1. File Structure & Indentation

Ernos uses Python-style indentation for block structure.
- Blocks introduced with `:` (colon)
- Consistent spaces for nesting (4 spaces recommended)
- No curly braces `{}` or semicolons `;`
- Comments start with `#`

```ernos
# This is a comment
define main:
    if 10 > 5:
        display "Inside block"
    display "Outside block"
    return 0
```

---

## 2. Variables & Type Annotations

### Declaration
```ernos
set x to 42                        # inferred as Int
set name to "Alice"                # inferred as Str
set pi to 3.14159                  # inferred as Float
set flag to true                   # inferred as Bool
```

### Explicit Type Annotations
```ernos
set x as Int to 42
set name as Str to "Alice"
set ratio as Float to 3.14
```

Type annotations are optional — the Hindley-Milner inference engine determines types automatically. But if inference is ambiguous, the compiler requires an annotation.

---

## 3. Operators

### Arithmetic
| Shorthand | English | Operation |
|:---------:|:--------|:----------|
| `+` | `plus` | Addition |
| `-` | `minus` | Subtraction |
| `*` | `multiplied by` | Multiplication |
| `/` | `divided by` | Division |
| `%` | `modulo` | Remainder |

### Comparison
| Shorthand | English | Comparison |
|:---------:|:--------|:-----------|
| `<` | `is less than` | Less Than |
| `>` | `is greater than` | Greater Than |
| `==` | `equals` / `is equal to` | Equal To |
| `!=` | `is not equal to` | Not Equal To |

### Logical
| Shorthand | English | Operation |
|:---------:|:--------|:----------|
| `&&` | `and also` | Logical AND |
| `\|\|` | `or else` | Logical OR |
| `not` | `not` | Logical NOT |

```ernos
set result to 10 + 5 * 2           # 20 (precedence enforced)
if score > 90 && passed == true:
    display "Excellence!"
```

---

## 4. Control Flow

### If / Else
```ernos
if score >= 90:
    display "Grade A"
else:
    display "Grade B"
```

### Repeat While (loops)
```ernos
set i to 0
repeat while i < 10:
    display i
    set i to i + 1
```

### For Each
```ernos
set items to create_list()
set ok to append_list(items and 10)
set ok to append_list(items and 20)

for each item in items:
    display item
```

### Break / Continue
```ernos
set i to 0
repeat while i < 100:
    set i to i + 1
    if i == 50:
        break
    if i % 2 == 0:
        continue
    display i
```

### Pattern Matching
```ernos
check status:
    on Success with value:
        display value
    on Error with msg:
        display msg
```

---

## 5. Functions

### Basic Functions
```ernos
define greet:
    display "Hello!"
    return 0

define add with a as Int and b as Int returning Int:
    return a + b
```

### Calling Functions
Arguments are separated by `and`:
```ernos
set result to add(10 and 20)
```

### Type Inference on Untyped Functions
```ernos
define double with x:
    return x * 2       # infers x must be Int (from * operator)

define main:
    display double(21)        # ✓ works: 42
    display double("hello")   # ✗ REJECTED: expected Int, found Str
    return 0
```

### Async Functions
```ernos
define async fetch_data with url as Str returning Int:
    set data to ep_net_connect(url)
    return data

define main:
    set result to await fetch_data("example.com")
    display result
    return 0
```

---

## 6. Structs & Methods

### Defining Structs
```ernos
define structure Point:
    field x as Int
    field y as Int

define structure User:
    field name as Str
    field age as Int
```

### Creating Instances
```ernos
set p to create Point:
    x is 10
    y is 20

set user to create User:
    name is "Alice"
    age is 30
```

### Methods
```ernos
define distance on Point with other as Point returning Int:
    set dx to self.x - other.x
    set dy to self.y - other.y
    return dx * dx + dy * dy

define main:
    set a to create Point:
        x is 0
        y is 0
    set b to create Point:
        x is 3
        y is 4
    display a.distance(b)    # 25
    return 0
```

---

## 7. Enums & Pattern Matching

```ernos
define choice Result:
    variant Ok with value as Int
    variant Error with message as Str

define main:
    set r to Ok with 42
    check r:
        on Ok with value:
            display value
        on Error with msg:
            display msg
    return 0
```

---

## 8. Concurrency

### Spawn Threads
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

### Channels
```ernos
define producer with ch:
    send 42 to ch
    return 0

define main:
    set ch to create_channel()
    spawn producer(ch)
    set value to receive from ch
    display value    # 42
    return 0
```

---

## 9. Ownership & Borrowing

Ernos enforces memory safety at compile time:

```ernos
# ✓ Valid: pass ownership
define consume with data:
    display data
    return 0

# ✗ REJECTED: cannot send borrowed reference to thread
define main:
    set x to create_list()
    spawn consume(borrow x)    # Compilation error!
    return 0
```

Rules:
- Each value has exactly one owner
- Borrowing creates a reference without transferring ownership
- Borrowed references cannot be sent to spawned threads
- The compiler rejects violations before generating code

---

## 10. Closures & Higher-Order Functions

```ernos
define apply with f and x as Int returning Int:
    return f(x)

define main:
    set doubler to given x:
        return x * 2
    display apply(doubler and 21)    # 42
    return 0
```

---

## 11. Error Handling

```ernos
set result to try risky_operation()
if result == 0:
    display "Operation failed safely"
else:
    display "Success"
```

---

## 12. Standard Library

Import standard library modules:
```ernos
import "stdlib/string.ep"
import "stdlib/collections.ep"
import "stdlib/json.ep"
```

19 modules available: `string`, `collections`, `fs`, `net`, `http`, `json`, `csv`, `datetime`, `crypto`, `regex`, `sync`, `os`, `test`, `log`, `math`, `sort`, `sql`, `gui`, `hash`.

See the full standard library documentation in `stdlib/README.md`.

---

## 13. Compilation

### Basic Usage
```bash
ernos program.ep           # Compile with -O2
./program                  # Run the native binary
```

### Build Modes
```bash
ernos --release program.ep  # Compile with -O3 + LTO
ernos --debug program.ep    # Compile with -O0 + debug symbols
ernos --check program.ep    # Type check only, no binary
ernos --format program.ep   # Auto-format source code
ernos --repl                # Interactive REPL
```

### Cross-Platform
The generated C code compiles on any platform with a C compiler:
```bash
# macOS
clang program_compiled.c -O2 -o program -lpthread

# Linux
gcc program_compiled.c -O2 -o program -lpthread

# Windows (MinGW)
gcc program_compiled.c -O2 -o program.exe -lpthread
```

---

## Type System Summary

| Type | Description | Example |
|------|-------------|---------|
| `Int` | 64-bit signed integer | `42` |
| `Float` | 64-bit double | `3.14` |
| `Bool` | Boolean | `true`, `false` |
| `Str` | String literal (immutable) | `"hello"` |
| `DynStr` | Heap-allocated string | `concat("a" and "b")` |
| `List of T` | Dynamic array | `create_list()` |
| `StructName` | Named struct | `create Point: ...` |
| `EnumName` | Tagged union | `Ok with 42` |

---

<p align="center"><b>Ernos</b> — Code that reads like English. Runs like C.</p>
