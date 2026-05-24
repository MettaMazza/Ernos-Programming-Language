pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;

use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: epc <filename.ep>");
        std::process::exit(1);
    }

    let input_path_str = &args[1];
    let input_path = Path::new(input_path_str);
    
    if !input_path.exists() {
        eprintln!("Error: File '{}' does not exist.", input_path_str);
        std::process::exit(1);
    }

    let stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    
    // Read source code
    let source = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    println!("[1/3] Tokenizing and Parsing '{}'...", input_path_str);

    // Lexing
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(e) => {
            print_diagnostic(input_path_str, &source, &e.message, e.span.line, e.span.col);
            std::process::exit(1);
        }
    };

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            print_diagnostic(input_path_str, &source, &e.message, e.span.line, e.span.col);
            std::process::exit(1);
        }
    };

    // Validate that main function exists
    let has_main = program.functions.iter().any(|f| f.name == "main");
    if !has_main {
        eprintln!("Compiler Error: Every program must have a 'main' function.");
        std::process::exit(1);
    }

    println!("[2/3] Generating ARM64 Assembly...");

    // Code generation
    let mut codegen = codegen::Codegen::new();
    let assembly = match codegen.generate(&program) {
        Ok(asm) => asm,
        Err(e) => {
            eprintln!("Code Generation Error: {}", e);
            std::process::exit(1);
        }
    };

    // Write temporary assembly file
    let asm_path_str = format!("{}.s", stem);
    let asm_path = Path::new(&asm_path_str);
    if let Err(e) = fs::write(asm_path, &assembly) {
        eprintln!("Error writing assembly file: {}", e);
        std::process::exit(1);
    }

    // Write temporary C runtime file
    let runtime_path_str = format!("{}_runtime.c", stem);
    let runtime_path = Path::new(&runtime_path_str);
    if let Err(e) = fs::write(runtime_path, RUNTIME_C) {
        eprintln!("Error writing runtime C file: {}", e);
        let _ = fs::remove_file(asm_path);
        std::process::exit(1);
    }

    println!("[3/3] Assembling and Linking via Clang...");

    // Run clang to assemble and link both assembly and runtime C file
    let output_executable = format!("./{}", stem);
    let clang_status = Command::new("clang")
        .arg(&asm_path_str)
        .arg(&runtime_path_str)
        .arg("-o")
        .arg(&stem)
        .status();

    // Clean up temporary files
    let _ = fs::remove_file(asm_path);
    let _ = fs::remove_file(runtime_path);

    match clang_status {
        Ok(status) if status.success() => {
            println!("\nSuccessfully compiled into native binary: {}", output_executable);
        }
        Ok(status) => {
            eprintln!("Error: Clang compilation failed with exit code: {}", status);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error invoking Clang: {}", e);
            std::process::exit(1);
        }
    }
}

const RUNTIME_C: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* read_file_content(const char* filepath) {
    FILE* f = fopen(filepath, "rb");
    if (!f) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    
    char* buf = malloc(size + 1);
    if (!buf) {
        fclose(f);
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    
    size_t read_bytes = fread(buf, 1, size, f);
    buf[read_bytes] = '\0';
    fclose(f);
    return buf;
}

long long string_length(const char* s) {
    if (!s) return 0;
    return strlen(s);
}

long long get_character(const char* s, long long index) {
    if (!s) return 0;
    long long len = strlen(s);
    if (index < 0 || index >= len) return 0;
    return (unsigned char)s[index];
}

typedef struct {
    long long* data;
    long long capacity;
    long long length;
} EpList;

long long create_list(void) {
    EpList* list = malloc(sizeof(EpList));
    if (!list) return 0;
    list->capacity = 4;
    list->length = 0;
    list->data = malloc(list->capacity * sizeof(long long));
    return (long long)list;
}

long long append_list(long long list_ptr, long long value) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    if (list->length >= list->capacity) {
        list->capacity *= 2;
        list->data = realloc(list->data, list->capacity * sizeof(long long));
    }
    list->data[list->length] = value;
    list->length += 1;
    return value;
}

long long get_list(long long list_ptr, long long index) {
    EpList* list = (EpList*)list_ptr;
    if (!list || index < 0 || index >= list->length) return 0;
    return list->data[index];
}

long long set_list(long long list_ptr, long long index, long long value) {
    EpList* list = (EpList*)list_ptr;
    if (!list || index < 0 || index >= list->length) return 0;
    list->data[index] = value;
    return value;
}

long long length_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    return list->length;
}

void free_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return;
    free(list->data);
    free(list);
}

int ep_argc = 0;
char** ep_argv = NULL;

void init_ep_args(int argc, char** argv) {
    ep_argc = argc;
    ep_argv = argv;
}

long long get_argument_count(void) {
    return ep_argc;
}

const char* get_argument(long long index) {
    if (index < 0 || index >= ep_argc) {
        return "";
    }
    return ep_argv[index];
}

long long write_file_content(const char* filepath, const char* content) {
    FILE* f = fopen(filepath, "wb");
    if (!f) return 0;
    size_t len = strlen(content);
    size_t written = fwrite(content, 1, len, f);
    fclose(f);
    return written == len ? 1 : 0;
}

long long run_command(const char* command) {
    if (!command) return -1;
    return system(command);
}

char* substring(const char* s, long long start, long long len) {
    if (!s) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    long long total_len = strlen(s);
    if (start < 0 || start >= total_len || len <= 0) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    if (start + len > total_len) {
        len = total_len - start;
    }
    char* sub = malloc(len + 1);
    if (!sub) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    strncpy(sub, s + start, len);
    sub[len] = '\0';
    return sub;
}

char* string_from_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    char* s = malloc(list->length + 1);
    if (!s) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty ? empty : "";
    }
    for (long long i = 0; i < list->length; i++) {
        s[i] = (char)list->data[i];
    }
    s[list->length] = '\0';
    return s;
}

long long pop_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list || list->length <= 0) return 0;
    list->length -= 1;
    return list->data[list->length];
}

void display_string(const char* s) {
    if (!s) return;
    printf("%s\n", s);
}
"#;

fn print_diagnostic(file_path: &str, source: &str, message: &str, line: usize, col: usize) {
    let lines: Vec<&str> = source.lines().collect();
    eprintln!("\x1b[1;31mError\x1b[0m: \x1b[1m{}\x1b[0m", message);
    eprintln!("  \x1b[1;34m-->\x1b[0m {}:{}:{}", file_path, line, col);
    eprintln!("   \x1b[1;34m|\x1b[0m");
    if line > 0 && line <= lines.len() {
        let line_content = lines[line - 1];
        eprintln!(" \x1b[1;34m{:3} |\x1b[0m {}", line, line_content);
        let padding = " ".repeat(if col > 0 { col - 1 } else { 0 });
        eprintln!("   \x1b[1;34m|\x1b[0m {}\x1b[1;31m^\x1b[0m", padding);
        if let Some(suggestion) = get_suggestion(message) {
            eprintln!("   \x1b[1;34m|\x1b[0m {}\x1b[1;33mHelp: {}\x1b[0m", padding, suggestion);
        }
    }
    eprintln!("   \x1b[1;34m|\x1b[0m");
}

fn get_suggestion(message: &str) -> Option<&str> {
    if message.contains("plas") {
        Some("Did you mean 'plus'?")
    } else if message.contains("minis") {
        Some("Did you mean 'minus'?")
    } else if message.contains("defin") {
        Some("Did you mean 'define'?")
    } else if message.contains("displayy") {
        Some("Did you mean 'display'?")
    } else if message.contains("repeate") {
        Some("Did you mean 'repeat'?")
    } else if message.contains("character: ','") {
        Some("In ErnosPlain, function arguments are separated by 'and', not commas.")
    } else if message.contains("Unexpected statement start: Identifier") {
        Some("Functions called as statements must be assigned to variables, e.g. 'set ok to func(...)'.")
    } else {
        None
    }
}
