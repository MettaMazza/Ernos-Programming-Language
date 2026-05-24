# ErnosPlain Language Reference Manual

Welcome to the **ErnosPlain** Programming Language reference manual. ErnosPlain is an expression-oriented, indentation-structured programming language. 

It supports both a **plain English conversational syntax** (highly readable) and **optional symbols shorthand** (fast writing). The compiler compiles code directly to native ARM64 machine-code binaries on Apple Silicon macOS.

---

## Table of Contents
1. [File Structure & Indentation](#1-file-structure--indentation)
2. [Variables](#2-variables)
3. [Operators (Math, Comparisons, Logic)](#3-operators-math-comparisons-logic)
4. [Control Flow (If/Else, Loops)](#4-control-flow-ifelse-loops)
5. [Functions](#5-functions)
6. [Built-in Functions (Runtime Library)](#6-built-in-functions-runtime-library)
7. [Compilation Guide](#7-compilation-guide)

---

## 1. File Structure & Indentation

ErnosPlain uses Python-style block indentation. 
* Code blocks are introduced with a colon (`:`).
* Nested blocks must be indented with consistent spaces or tabs (spaces are recommended).
* A decrease in indentation (dedent) signals the end of the block.
* No curly braces (`{}`) or semicolons (`;`) are required.

```ernos
define main:
    if 10 is greater than 5:
        display "Inside block"
    display "Outside block"
    return 0
```

---

## 2. Variables

Variables are declared and updated using the `set [variable] to [expression]` statement:
```ernos
set a to 10
set name to "ErnosPlain"
set result to a + 5
```

---

## 3. Operators (Math, Comparisons, Logic)

ErnosPlain allows you to choose between plain English phrasing or standard symbols.

### Arithmetic Operators
Arithmetic follows standard mathematical order of operations (multiplication and division first, then addition and subtraction):

| Shorthand | Plain English | Operation |
|:---:|:---|:---|
| `+` | `plus` | Addition |
| `-` | `minus` | Subtraction |
| `*` | `multiplied by` | Multiplication |
| `/` | `divided by` | Division |

*Example:*
```ernos
set key to 10 plus 5 multiplied by 2    # Equals 20 (precedence enforced!)
set key_alt to 10 + 5 * 2              # Same result
```

### Comparison Operators
Comparisons evaluate to `1` (true) or `0` (false):

| Shorthand | Plain English | Comparison |
|:---:|:---|:---|
| `<` | `is less than` | Less Than |
| `>` | `is greater than` | Greater Than |
| `==` | `equals` / `is equal to` | Equal To |
| `!=` | `is not equal to` | Not Equal To |

*Example:*
```ernos
if score > 90:
    display "Winner!"
```

### Logical Operators
Conditionals are combined using short-circuiting logical operators:

| Shorthand | Plain English | Logical Operation |
|:---:|:---|:---|
| `&&` | `and also` | Logical AND |
| `||` | `or else` | Logical OR |

*Example:*
```ernos
if age >= 18 && age <= 30:
    display "Eligible"
```

---

## 4. Control Flow (If/Else, Loops)

### Conditionals (If / Else)
Execute branches based on conditions:
```ernos
if score >= 90:
    display "Grade A"
else:
    display "Grade B"
```

### Loops
Execute blocks repeatedly while a condition is true:
```ernos
# English syntax:
set i to 1
repeat while i is less than 5:
    display i
    set i to i plus 1

# Shorthand syntax:
set i to 1
while i < 5:
    display i
    set i to i + 1
```

---

## 5. Functions

### Declaration
Functions are declared globally using `define [name] with [p1] and [p2]:`.
Every executable program must contain a `main` function as its starting entry point.

```ernos
define main:
    set score to sum(10 and 20)
    display score
    return 0

define sum with x and y:
    return x + y
```

### Invocation
Functions are called with arguments enclosed in parentheses and separated by the `and` keyword:
```ernos
set val to calculate_bonus(salary and years_worked)
```

---

## 6. Built-in Functions (Runtime Library)

ErnosPlain comes with a built-in C runtime containing vital functions for file parsing, string querying, and dynamic memory allocation.

### File Parsing & Strings
* **`read_file_content(path)`**:
  Reads a text file and returns its content as a string.
  ```ernos
  set file_content to read_file_content("hello.ep")
  ```
* **`string_length(str)`**:
  Returns the length (as an integer) of the string.
  ```ernos
  set size to string_length(file_content)
  ```
* **`get_character(str and index)`**:
  Returns the ASCII integer value of the character at the specified index.
  ```ernos
  set char_code to get_character(file_content and 0)
  ```

### Dynamic Lists (Arrays)
Lists hold 64-bit integers. Since string pointers and list pointers are also 64-bit addresses, they can store any values:
* **`create_list()`**:
  Allocates a new heap-based list and returns its pointer.
  ```ernos
  set list to create_list()
  ```
* **`append_list(list and value)`**:
  Appends an integer/pointer to the list. Returns the value.
  *Note: must be assigned using `set`.*
  ```ernos
  set ok to append_list(list and 42)
  set ok to append_list(list and "Bob")
  ```
* **`get_list(list and index)`**:
  Returns the value at the index.
  ```ernos
  set name_ptr to get_list(list and 1)
  ```
* **`set_list(list and index and value)`**:
  Updates the list value at the index.
  ```ernos
  set ok to set_list(list and 0 and 99)
  ```
* **`length_list(list)`**:
  Returns the size of the list.
  ```ernos
  set size to length_list(list)
  ```

### Printing String Pointers
* **`display_string(str_ptr)`**:
  Prints a raw string pointer retrieved from lists or dynamic memory directly to stdout.
  *Note: must be assigned using `set`.*
  ```ernos
  set ok to display_string(name_ptr)
  ```

---

## 7. Compilation Guide

To compile your ErnosPlain files:
1. Save your code in a file with `.ep` extension, e.g. `test.ep`.
2. Run the compiler:
   ```bash
   ./epc test.ep
   ```
3. Run the resulting native executable:
   ```bash
   ./test
   ```
   This compiled program runs directly on the ARM64 macOS CPU with zero external dependencies.
