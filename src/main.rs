pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;

use std::env;
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

fn resolve_import_path(current_file: &Path, import_path: &str) -> PathBuf {
    if import_path == "math" || import_path == "hash" || import_path == "net" || import_path == "json" || import_path == "string" {
        let stdlib_path = Path::new("stdlib").join(format!("{}.ep", import_path));
        if stdlib_path.exists() {
            return stdlib_path;
        }
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let exe_stdlib = exe_dir.join("stdlib").join(format!("{}.ep", import_path));
                if exe_stdlib.exists() {
                    return exe_stdlib;
                }
            }
        }
        let fallback_path = Path::new("/Users/mettamazza/Desktop/ErnosPlain Programing Language/stdlib").join(format!("{}.ep", import_path));
        if fallback_path.exists() {
            return fallback_path;
        }
    }

    let mut resolved = current_file.parent().unwrap_or(Path::new("")).join(import_path);
    if !resolved.exists() && !import_path.ends_with(".ep") {
        resolved.set_extension("ep");
    }
    resolved
}

fn parse_all_modules(
    entry_path: &Path,
    parsed_files: &mut HashSet<PathBuf>,
    all_functions: &mut Vec<ast::Function>,
) -> Result<(), String> {
    let canonical_path = entry_path.canonicalize().map_err(|e| format!("Could not canonicalize path '{}': {}", entry_path.display(), e))?;
    
    if parsed_files.contains(&canonical_path) {
        return Ok(());
    }
    parsed_files.insert(canonical_path.clone());

    let source = fs::read_to_string(&canonical_path).map_err(|e| format!("Error reading file '{}': {}", canonical_path.display(), e))?;
    
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(e) => {
            print_diagnostic(canonical_path.to_str().unwrap_or(""), &source, &e.message, e.span.line, e.span.col);
            return Err("Lexer error".to_string());
        }
    };

    let mut parser = parser::Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            print_diagnostic(canonical_path.to_str().unwrap_or(""), &source, &e.message, e.span.line, e.span.col);
            return Err("Parser error".to_string());
        }
    };

    all_functions.extend(program.functions);

    for imp in program.imports {
        let resolved_path = resolve_import_path(&canonical_path, &imp);
        if !resolved_path.exists() {
            return Err(format!("Import error in '{}': module '{}' not found at '{}'", canonical_path.display(), imp, resolved_path.display()));
        }
        parse_all_modules(&resolved_path, parsed_files, all_functions)?;
    }

    Ok(())
}

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
    
    println!("[1/3] Tokenizing and Parsing '{}'...", input_path_str);

    let mut all_functions = Vec::new();
    let mut parsed_files = HashSet::new();
    if let Err(err_msg) = parse_all_modules(input_path, &mut parsed_files, &mut all_functions) {
        eprintln!("Compiler Error: {}", err_msg);
        std::process::exit(1);
    }

    let mut function_names = HashSet::new();
    for func in &all_functions {
        if !function_names.insert(&func.name) {
            eprintln!("Compiler Error: Function '{}' is defined multiple times across modules.", func.name);
            std::process::exit(1);
        }
    }

    let program = ast::Program {
        imports: Vec::new(),
        functions: all_functions,
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

    // Write temporary C source file
    let c_path_str = format!("{}_compiled.c", stem);
    let c_path = Path::new(&c_path_str);
    if let Err(e) = fs::write(c_path, &assembly) {
        eprintln!("Error writing compiled C file: {}", e);
        std::process::exit(1);
    }

    println!("[3/3] Compiling and Linking via Clang...");

    // Run clang to compile and link the transpiled C file
    let output_executable = format!("./{}", stem);
    let clang_status = Command::new("clang")
        .arg(&c_path_str)
        .arg("-o")
        .arg(&stem)
        .status();

    // Clean up temporary files
    // let _ = fs::remove_file(c_path);

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
