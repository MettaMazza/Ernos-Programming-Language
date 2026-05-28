#![deny(unused_imports)]
pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod type_check;
pub mod diagnostics;
pub mod borrow_check;
pub mod optimizer;
pub mod arm64;
pub mod native_codegen;
pub mod x86_64_codegen;
pub mod bind_c;

use std::env;
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

fn resolve_import_path(current_file: &Path, import_path: &str) -> PathBuf {
    let stdlib_modules = ["math", "hash", "net", "json", "string", "sql", "gui", "crypto", "fs", "http", "collections", "sort", "datetime", "os", "test", "log", "sync", "regex", "csv", "websocket", "static_server", "toml", "select"];
    if stdlib_modules.contains(&import_path) {
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
        // No hardcoded fallback — stdlib must be in CWD or alongside the compiler binary
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
    all_externals: &mut Vec<ast::ExternalFunction>,
    all_struct_defs: &mut Vec<ast::StructDef>,
    all_enum_defs: &mut Vec<ast::EnumDef>,
    all_method_defs: &mut Vec<ast::MethodDef>,
    all_trait_defs: &mut Vec<ast::TraitDef>,
    all_trait_impls: &mut Vec<ast::TraitImpl>,
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
    all_externals.extend(program.externals);
    all_struct_defs.extend(program.struct_defs);
    all_enum_defs.extend(program.enum_defs);
    all_method_defs.extend(program.method_defs);
    all_trait_defs.extend(program.trait_defs);
    all_trait_impls.extend(program.trait_impls);

    for (imp, alias) in program.imports {
        let resolved_path = resolve_import_path(&canonical_path, &imp);
        if !resolved_path.exists() {
            return Err(format!("Import error in '{}': module '{}' not found at '{}'", canonical_path.display(), imp, resolved_path.display()));
        }

        if let Some(ref prefix) = alias {
            // Aliased import: parse the module, then add BOTH prefixed and original names.
            // Prefixed names allow the caller to use alias_function_name().
            // Original names are needed because the module's own functions reference each other.
            let mut mod_funcs: Vec<ast::Function> = Vec::new();
            let mut mod_externs: Vec<ast::ExternalFunction> = Vec::new();
            let mut mod_structs: Vec<ast::StructDef> = Vec::new();
            let mut mod_enums: Vec<ast::EnumDef> = Vec::new();
            let mut mod_methods: Vec<ast::MethodDef> = Vec::new();
            let mut mod_traits: Vec<ast::TraitDef> = Vec::new();
            let mut mod_trait_impls: Vec<ast::TraitImpl> = Vec::new();
            parse_all_modules(&resolved_path, parsed_files, &mut mod_funcs, &mut mod_externs, &mut mod_structs, &mut mod_enums, &mut mod_methods, &mut mod_traits, &mut mod_trait_impls)?;

            // Add original-named functions (for internal module calls)
            for f in &mod_funcs {
                all_functions.push(f.clone());
            }
            // Add prefixed aliases
            for f in mod_funcs {
                let mut aliased = f;
                aliased.name = format!("{}_{}", prefix, aliased.name);
                all_functions.push(aliased);
            }
            for e in &mod_externs {
                all_externals.push(e.clone());
            }
            for e in mod_externs {
                let mut aliased = e;
                aliased.name = format!("{}_{}", prefix, aliased.name);
                all_externals.push(aliased);
            }
            all_struct_defs.extend(mod_structs);
            all_enum_defs.extend(mod_enums);
            all_method_defs.extend(mod_methods);
            all_trait_defs.extend(mod_traits);
            all_trait_impls.extend(mod_trait_impls);
        } else {
            // Unaliased import: dump everything into global namespace (backward compatible)
            parse_all_modules(&resolved_path, parsed_files, all_functions, all_externals, all_struct_defs, all_enum_defs, all_method_defs, all_trait_defs, all_trait_impls)?;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    // Handle REPL mode
    if args[1] == "--repl" || args[1] == "repl" {
        run_repl();
        return;
    }

    // Handle 'ernos bind header.h [-o output.ep]' subcommand
    if args[1] == "bind" {
        if args.len() < 3 {
            eprintln!("Usage: ernos bind <header.h> [-o output.ep]");
            std::process::exit(1);
        }
        let header_path = &args[2];
        let source = match fs::read_to_string(header_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading '{}': {}", header_path, e);
                std::process::exit(1);
            }
        };

        let bindings = bind_c::emit_ernos_bindings(header_path, &source);

        // Determine output path
        let output_path = if let Some(idx) = args.iter().position(|a| a == "-o") {
            args.get(idx + 1).cloned().unwrap_or_else(|| {
                let stem = Path::new(header_path).file_stem()
                    .and_then(|s| s.to_str()).unwrap_or("bindings");
                format!("bindings_{}.ep", stem)
            })
        } else {
            let stem = Path::new(header_path).file_stem()
                .and_then(|s| s.to_str()).unwrap_or("bindings");
            format!("bindings_{}.ep", stem)
        };

        if let Err(e) = fs::write(&output_path, &bindings) {
            eprintln!("Error writing '{}': {}", output_path, e);
            std::process::exit(1);
        }

        println!("Generated bindings: {}", output_path);
        println!("  Parsed: {}", header_path);
        // Count what was generated
        let extern_count = bindings.matches("external define").count();
        let struct_count = bindings.matches("define structure").count();
        let const_count = bindings.matches("set BIND_").count();
        println!("  Functions: {}", extern_count);
        println!("  Structures: {}", struct_count);
        println!("  Constants: {}", const_count);
        return;
    }

    // Handle global flags
    if args[1] == "--version" || args[1] == "-v" {
        println!("Ernos Compiler v1.0.0");
        println!("  Backend: C (Clang) / native assembly");
        println!("  Target:  {}", std::env::consts::ARCH);
        println!("  OS:      {}", std::env::consts::OS);
        return;
    }

    if args[1] == "--help" || args[1] == "-h" {
        print_usage();
        return;
    }

    if args[1] == "--list-builtins" {
        println!("Ernos Built-in Functions:");
        println!();
        println!("  ── Strings ──");
        println!("  concat(a: Str, b: Str) -> Str              Concatenate two strings");
        println!("  string_length(s: Str) -> Int                Get string length");
        println!("  substring(s: Str, start: Int, len: Int) -> Str  Extract substring");
        println!("  int_to_string(n: Int) -> Str                Convert integer to string");
        println!("  get_character(s: Str, idx: Int) -> Int      Get character code at index");
        println!("  string_contains(s: Str, sub: Str) -> Int    Check if string contains substring");
        println!("  string_index_of(s: Str, sub: Str) -> Int    Find first index of substring (-1 if not found)");
        println!("  string_replace(s: Str, old: Str, new: Str) -> Str  Replace occurrences");
        println!("  string_from_list(list: List) -> Str         Build string from list of char codes");
        println!("  string_upper(s: Str) -> Str                  Convert to uppercase");
        println!("  string_lower(s: Str) -> Str                  Convert to lowercase");
        println!("  string_trim(s: Str) -> Str                   Strip leading/trailing whitespace");
        println!("  string_split(s: Str, delim: Str) -> List     Split string into list of parts");
        println!("  char_at(s: Str, index: Int) -> Int           Get character code at index");
        println!("  char_from_code(code: Int) -> Str             Create single-char string from code");
        println!("  f\"Hello {{name}}!\"                           F-string interpolation");
        println!();
        println!("  ── Lists ──");
        println!("  create_list() -> List                       Create empty list");
        println!("  append_list(list: List, item: Int) -> Int   Append item to list");
        println!("  get_list(list: List, index: Int) -> Int     Get item at index");
        println!("  set_list(list: List, index: Int, val: Int) -> Int  Set item at index");
        println!("  length_list(list: List) -> Int              Get list length");
        println!("  pop_list(list: List) -> Int                 Remove and return last element");
        println!("  free_list(list: List) -> Int                Manually free a list");
        println!();
        println!("  ── Maps (Key-Value) ──");
        println!("  create_map() -> Map                         Create empty hashmap");
        println!("  map_insert(map: Map, key, val) -> Int       Insert/update key-value pair (key: Str or Int)");
        println!("  map_get_val(map: Map, key: Str) -> Int      Get value by key (0 if not found)");
        println!("  map_contains(map: Map, key: Str) -> Int     Check if key exists (1/0)");
        println!("  map_delete(map: Map, key: Str) -> Int       Remove a key");
        println!("  map_keys(map: Map) -> List                  Get list of all keys");
        println!("  map_values(map: Map) -> List                Get list of all values");
        println!("  map_size(map: Map) -> Int                   Get number of entries");
        println!();
        println!("  ── I/O ──");
        println!("  read_line() -> Str                          Read line from stdin");
        println!("  read_int() -> Int                           Read integer from stdin");
        println!("  display <expr>                              Print value to stdout");
        println!();
        println!("  ── Math & Random ──");
        println!("  ep_random_int(min: Int, max: Int) -> Int    Random integer in range [min, max]");
        println!("  ep_abs(n: Int) -> Int                       Absolute value");
        println!();
        println!("  ── Concurrency ──");
        println!("  create_channel() -> Channel                 Create a message channel");
        println!("  send <value> to <channel>                   Send value to channel");
        println!("  receive from <channel> -> Int               Receive value from channel");
        println!("  spawn <function>(args...)                    Run function in new thread");
        println!("  ep_sleep_ms(ms: Int) -> Int                 Sleep for milliseconds");
        println!("  channel_select(chans: List, ms: Int) -> Int  Wait on multiple channels");
        println!("  channel_has_data(ch: Channel) -> Int         Check if channel has data");
        println!("  channel_try_recv(ch: Channel, out: Ptr) -> Int  Non-blocking receive");
        println!();
        println!("  ── File I/O ──");
        println!("  file_read(path: Str) -> Str                 Read entire file");
        println!("  file_write(path: Str, content: Str) -> Int  Write to file");
        println!("  file_append(path: Str, content: Str) -> Int Append to file");
        println!("  file_exists(path: Str) -> Int               Check if file exists");
        println!();
        println!("  ── Time ──");
        println!("  ep_time_now_ms() -> Int                     Current time in milliseconds");
        println!("  ep_time_now_sec() -> Int                    Current time in seconds (epoch)");
        println!();
        println!("  ── JSON ──");
        println!("  json_get_string(json: Str, key: Str) -> Str  Extract string value from JSON");
        println!("  json_get_int(json: Str, key: Str) -> Int     Extract integer value from JSON");
        println!("  json_get_bool(json: Str, key: Str) -> Int    Extract boolean value from JSON");
        println!();
        println!("  ── Hashing ──");
        println!("  ep_sha256(data: Str) -> Str                 SHA-256 hash (hex string)");
        println!("  ep_md5(data: Str) -> Str                    MD5 hash (hex string)");
        println!("  ep_sha1(data: Str) -> Str                   SHA-1 hash (hex string)");
        println!();
        println!("  ── Networking ──");
        println!("  ep_http_request(method: Str, url: Str, headers: Str, body: Str) -> Str");
        println!("  ep_net_connect(host: Str, port: Int) -> Int  TCP connect");
        println!("  ep_net_listen(port: Int) -> Int              TCP listen");
        println!("  ep_net_accept(fd: Int) -> Int                TCP accept");
        println!("  ep_net_send(fd: Int, data: Str) -> Int       Send data");
        println!("  ep_net_recv(fd: Int, max: Int) -> Str        Receive data");
        println!("  ep_net_close(fd: Int) -> Int                 Close connection");
        println!();
        return;
    }

    // Handle --check (syntax check only, no codegen)
    if args[1] == "--check" || args[1] == "check" {
        if args.len() < 3 {
            eprintln!("Usage: epc --check <filename.ep>");
            std::process::exit(1);
        }
        let input_path = Path::new(&args[2]);
        if !input_path.exists() {
            eprintln!("Error: File '{}' does not exist.", args[2]);
            std::process::exit(1);
        }
        let mut all_functions = Vec::new();
        let mut all_externals = Vec::new();
        let mut all_struct_defs = Vec::new();
        let mut all_enum_defs = Vec::new();
        let mut all_method_defs = Vec::new();
        let mut all_trait_defs = Vec::new();
        let mut all_trait_impls = Vec::new();
        let mut parsed_files = HashSet::new();
        match parse_all_modules(input_path, &mut parsed_files, &mut all_functions, &mut all_externals, &mut all_struct_defs, &mut all_enum_defs, &mut all_method_defs, &mut all_trait_defs, &mut all_trait_impls) {
            Ok(()) => {
                println!("\x1b[1;32m✓\x1b[0m {} — no syntax errors ({} functions, {} structs, {} enums)", 
                    args[2], all_functions.len(), all_struct_defs.len(), all_enum_defs.len());
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Handle --format (code formatter)
    if args[1] == "--format" || args[1] == "format" || args[1] == "fmt" {
        if args.len() < 3 {
            eprintln!("Usage: epc --format <filename.ep>");
            std::process::exit(1);
        }
        let input_path = Path::new(&args[2]);
        if !input_path.exists() {
            eprintln!("Error: File '{}' does not exist.", args[2]);
            std::process::exit(1);
        }
        format_file(input_path);
        return;
    }

    let is_test_mode = args.len() >= 3 && args[1] == "test";
    let input_path_str = if is_test_mode { &args[2] } else { &args[1] };
    let input_path = Path::new(input_path_str);
    
    if !input_path.exists() {
        eprintln!("Error: File '{}' does not exist.", input_path_str);
        std::process::exit(1);
    }

    let stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    
    println!("[1/3] Tokenizing and Parsing '{}'...", input_path_str);

    let mut all_functions = Vec::new();
    let mut all_externals = Vec::new();
    let mut all_struct_defs = Vec::new();
    let mut all_enum_defs = Vec::new();
    let mut all_method_defs = Vec::new();
    let mut all_trait_defs = Vec::new();
    let mut all_trait_impls = Vec::new();
    let mut parsed_files = HashSet::new();
    if let Err(err_msg) = parse_all_modules(input_path, &mut parsed_files, &mut all_functions, &mut all_externals, &mut all_struct_defs, &mut all_enum_defs, &mut all_method_defs, &mut all_trait_defs, &mut all_trait_impls) {
        eprintln!("Compiler Error: {}", err_msg);
        std::process::exit(1);
    }

    // Deduplicate functions (aliased imports add both original and prefixed names)
    let mut function_names = HashSet::new();
    let mut deduped_functions = Vec::new();
    for func in all_functions {
        if function_names.insert(func.name.clone()) {
            deduped_functions.push(func);
        }
        // Silently skip duplicates — expected from aliased imports
    }
    let all_functions = deduped_functions;

    let mut program = ast::Program {
        imports: Vec::new(),
        externals: all_externals,
        functions: all_functions,
        struct_defs: all_struct_defs,
        enum_defs: all_enum_defs,
        method_defs: all_method_defs,
        trait_defs: all_trait_defs,
        trait_impls: all_trait_impls,
    };

    // Validate that main function exists
    if !is_test_mode {
        let has_main = program.functions.iter().any(|f| f.name == "main");
        if !has_main {
            eprintln!("Compiler Error: Every program must have a 'main' function.");
            std::process::exit(1);
        }
    }

    // Check if native backend is requested
    let use_native = args.iter().any(|a| a == "--native");

    if use_native {
        let arch = std::env::consts::ARCH;
        let os = std::env::consts::OS;
        
        let (asm, is_x86_64) = if arch == "x86_64" {
            println!("[2/3] Generating Native x86_64 Assembly...");
            let mut ncg = x86_64_codegen::X86_64Codegen::new(os == "macos");
            (ncg.generate(&program), true)
        } else {
            println!("[2/3] Generating Native ARM64 Assembly...");
            let mut ncg = native_codegen::NativeCodegen::new(os == "macos");
            (ncg.generate(&program), false)
        };

        let asm = match asm {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Native Code Generation Error: {}", e);
                std::process::exit(1);
            }
        };

        // Write assembly to temp file
        let asm_path = format!("{}_native.s", stem);
        if let Err(e) = fs::write(&asm_path, &asm) {
            eprintln!("Error writing assembly: {}", e);
            std::process::exit(1);
        }

        println!("[3/4] Assembling Native Code...");

        // Assemble with system 'as'
        let obj_path = format!("{}_native.o", stem);
        let as_status = if os == "macos" {
            let target_arch = if is_x86_64 { "x86_64" } else { "arm64" };
            Command::new("as")
                .arg("-arch").arg(target_arch)
                .arg("-o").arg(&obj_path)
                .arg(&asm_path)
                .status()
        } else {
            Command::new("as")
                .arg("-o").arg(&obj_path)
                .arg(&asm_path)
                .status()
        };
        match as_status {
            Ok(s) if s.success() => {}
            Ok(s) => {
                eprintln!("Error: Assembler failed with exit code: {}", s);
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error invoking assembler: {}", e);
                std::process::exit(1);
            }
        }

        // Hybrid compilation: compile C runtime to .o, then link with native .o
        // This provides all the runtime functions (create_list, length_list, channels, GC, etc.)
        // that the native assembly references as external symbols.
        println!("[4/4] Compiling C Runtime and Linking...");

        let mut cg = codegen::Codegen::new();
        let runtime_c_src = cg.emit_runtime_c(&program);
        let runtime_c_path = format!("{}_runtime.c", stem);
        if let Err(e) = fs::write(&runtime_c_path, &runtime_c_src) {
            eprintln!("Error writing runtime C source: {}", e);
            std::process::exit(1);
        }

        // Compile runtime C to .o
        let runtime_obj_path = format!("{}_runtime.o", stem);
        let cc = if Command::new("clang").arg("--version").output().is_ok() { "clang" } else { "gcc" };
        let cc_status = Command::new(cc)
            .arg("-c")
            .arg("-O2")
            .arg("-o").arg(&runtime_obj_path)
            .arg(&runtime_c_path)
            .status();
        match cc_status {
            Ok(s) if s.success() => {}
            Ok(s) => {
                eprintln!("Error: C compiler failed on runtime with exit code: {}", s);
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error invoking C compiler for runtime: {}", e);
                std::process::exit(1);
            }
        }

        // Link native assembly .o + C runtime .o into final binary
        let ld_status = if os == "macos" {
            let target_arch = if is_x86_64 { "x86_64" } else { "arm64" };
            Command::new("ld")
                .arg("-o").arg(stem)
                .arg(&obj_path)
                .arg(&runtime_obj_path)
                .arg("-lSystem")
                .arg("-syslibroot").arg("/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk")
                .arg("-arch").arg(target_arch)
                .status()
        } else {
            // Linux: use gcc to link, include pthread and math libs
            Command::new("gcc")
                .arg("-no-pie")
                .arg("-o").arg(stem)
                .arg(&obj_path)
                .arg(&runtime_obj_path)
                .arg("-lpthread")
                .arg("-lm")
                .status()
        };

        match ld_status {
            Ok(s) if s.success() => {
                // Clean up temp files
                let _ = fs::remove_file(&asm_path);
                let _ = fs::remove_file(&obj_path);
                let _ = fs::remove_file(&runtime_c_path);
                let _ = fs::remove_file(&runtime_obj_path);
                println!("\nSuccessfully compiled into native binary: ./{}", stem);
            }
            Ok(s) => {
                eprintln!("Error: Linker failed with exit code: {}", s);
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error invoking linker: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Type checking (Phase 1A) — HARD ERRORS: reject programs with type errors
    let (type_errors, _type_warnings) = type_check::TypeChecker::check_full(&program);
    if !type_errors.is_empty() {
        eprintln!("\n\x1b[1;31m── Type Errors ({}) ──\x1b[0m", type_errors.len());
        for err in &type_errors {
            eprintln!("  \x1b[1;31merror\x1b[0m: {}", err);
        }
        eprintln!();
        eprintln!("\x1b[1;31mCompilation failed:\x1b[0m {} type error(s) found. Fix all type errors before compiling.", type_errors.len());
        std::process::exit(1);
    }

    // Borrow checking (Phase 3) — HARD ERRORS: reject programs with ownership violations
    let borrow_errors = borrow_check::BorrowChecker::check(&program);
    if !borrow_errors.is_empty() {
        eprintln!("\n\x1b[1;31m── Ownership Errors ({}) ──\x1b[0m", borrow_errors.len());
        for err in &borrow_errors {
            eprint!("{}", err);
        }
        eprintln!();
        eprintln!("\x1b[1;31mCompilation failed:\x1b[0m {} ownership/borrowing error(s) found. Fix all safety violations before compiling.", borrow_errors.len());
        std::process::exit(1);
    }
    // Optimization pass (Phase 4B)
    let opt_stats = optimizer::Optimizer::run(&mut program);
    if opt_stats.constants_folded > 0 || opt_stats.dead_stmts_eliminated > 0 {
        eprintln!("\x1b[2m  optimizer: {} constants folded, {} dead statements eliminated\x1b[0m",
            opt_stats.constants_folded, opt_stats.dead_stmts_eliminated);
    }

    println!("[2/3] Generating C Code...");

    // Code generation (C backend)
    let mut codegen = codegen::Codegen::new();
    codegen.is_test_mode = is_test_mode;
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
    
    let mut link_flags = Vec::new();
    link_flags.push("-lpthread");
    for path in &parsed_files {
        let path_str = path.to_string_lossy();
        if path_str.ends_with("sql.ep") {
            link_flags.push("-lsqlite3");
        }
        if path_str.ends_with("gui.ep") {
            link_flags.push("-lraylib");
        }
        if path_str.ends_with("crypto.ep") {
            link_flags.push("-L/opt/homebrew/opt/openssl/lib");
            link_flags.push("-lcrypto");
        }
    }

    // Determine optimization level and sanitizer flags
    let use_asan = args.iter().any(|a| a == "--asan" || a == "--sanitize");
    let opt_level = if args.iter().any(|a| a == "--release") {
        vec!["-O3", "-DNDEBUG", "-flto"]
    } else if args.iter().any(|a| a == "--debug") {
        vec!["-O0", "-g"]
    } else {
        vec!["-O2"]
    };

    let sanitizer_flags: Vec<&str> = if use_asan {
        vec!["-fsanitize=address", "-fsanitize=undefined", "-fno-omit-frame-pointer", "-g"]
    } else {
        vec![]
    };

    let mut clang_cmd = Command::new("clang");
    clang_cmd.arg(&c_path_str)
             .arg("-o")
             .arg(&stem);
    for flag in &opt_level {
        clang_cmd.arg(flag);
    }
    for flag in &sanitizer_flags {
        clang_cmd.arg(flag);
    }
    for flag in link_flags {
        clang_cmd.arg(flag);
    }
    let clang_status = clang_cmd.status();

    // Clean up temporary files
    // let _ = fs::remove_file(c_path);

    match clang_status {
        Ok(status) if status.success() => {
            #[cfg(target_os = "macos")]
            {
                let _ = Command::new("codesign")
                    .arg("--force")
                    .arg("-s")
                    .arg("-")
                    .arg(&stem)
                    .status();
            }
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
        Some("In Ernos, function arguments are separated by 'and', not commas.")
    } else if message.contains("Unexpected statement start: Identifier") {
        Some("Functions called as statements must be assigned to variables, e.g. 'set ok to func(...)'.")
    } else {
        None
    }
}

fn print_usage() {
    eprintln!("\x1b[1;36m┌──────────────────────────────────────────────────┐\x1b[0m");
    eprintln!("\x1b[1;36m│\x1b[0m  \x1b[1mErnos Compiler\x1b[0m — v1.0.0                       \x1b[1;36m│\x1b[0m");
    eprintln!("\x1b[1;36m└──────────────────────────────────────────────────┘\x1b[0m");
    eprintln!();
    eprintln!("\x1b[1mUSAGE:\x1b[0m");
    eprintln!("  epc <filename.ep>               Compile to native binary");
    eprintln!("  epc <filename.ep> --native      Compile via native assembly (no Clang required)");
    eprintln!("  epc <filename.ep> --release     Compile with optimizations (O3+LTO)");
    eprintln!("  epc test <filename.ep>          Run as test");
    eprintln!();
    eprintln!("\x1b[1mDEV TOOLS:\x1b[0m");
    eprintln!("  epc --check <filename.ep>       Syntax check (no compilation)");
    eprintln!("  epc --format <filename.ep>      Auto-format source file");
    eprintln!("  epc --list-builtins             List all built-in functions");
    eprintln!("  epc --version                   Show version info");
    eprintln!("  epc --help                      Show this message");
    eprintln!();
    eprintln!("\x1b[1mSAFETY:\x1b[0m");
    eprintln!("  epc <filename.ep> --asan         Compile with AddressSanitizer");
    eprintln!("  epc <filename.ep> --debug        Compile with debug symbols");
}

fn format_file(path: &Path) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let mut output = String::new();
    let mut prev_blank = false;

    for line in source.lines() {
        let trimmed = line.trim_end();

        // Normalize blank lines (max 1 consecutive)
        if trimmed.is_empty() {
            if !prev_blank {
                output.push('\n');
                prev_blank = true;
            }
            continue;
        }
        prev_blank = false;

        // Count leading spaces in original
        let leading_spaces = line.len() - line.trim_start().len();

        // Normalize indentation: each level = 4 spaces
        // Detect the current indent level (assuming original uses at least 1 space per level)
        let indent_level = if leading_spaces > 0 {
            // If the original uses tabs, convert to 4 spaces
            if line.starts_with('\t') {
                line.chars().take_while(|c| *c == '\t').count()
            } else {
                // Try to detect indent width: common values are 2 or 4
                // We'll just normalize to 4 spaces
                (leading_spaces + 1) / 4
            }
        } else {
            0
        };

        // Re-indent with 4 spaces per level
        for _ in 0..indent_level {
            output.push_str("    ");
        }
        output.push_str(trimmed.trim_start());
        output.push('\n');
    }

    // Remove trailing newline if needed
    while output.ends_with("\n\n") {
        output.pop();
    }
    if !output.ends_with('\n') {
        output.push('\n');
    }

    match fs::write(path, &output) {
        Ok(()) => {
            println!("\x1b[1;32m✓\x1b[0m Formatted: {}", path.display());
        }
        Err(e) => {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    use std::io::{self, Write, BufRead};

    println!("\x1b[1;36m╔══════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║   ErnosPlain REPL v1.0                   ║\x1b[0m");
    println!("\x1b[1;36m║   Type ErnosPlain code and press Enter   ║\x1b[0m");
    println!("\x1b[1;36m║   Type 'exit' or 'quit' to leave         ║\x1b[0m");
    println!("\x1b[1;36m║   Type ':help' for commands               ║\x1b[0m");
    println!("\x1b[1;36m╚══════════════════════════════════════════╝\x1b[0m");
    println!();

    let stdin = io::stdin();
    let mut history: Vec<String> = Vec::new();
    let mut line_buffer = String::new();
    let mut _in_block = false;
    let mut accumulated_lines: Vec<String> = Vec::new();
    let mut user_functions: Vec<String> = Vec::new();
    let mut block_buffer: Vec<String> = Vec::new();

    loop {
        if _in_block {
            print!("\x1b[33m...  \x1b[0m");
        } else {
            print!("\x1b[1;32mep>\x1b[0m ");
        }
        io::stdout().flush().unwrap();

        line_buffer.clear();
        match stdin.lock().read_line(&mut line_buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        let trimmed = line_buffer.trim();

        // Handle multi-line block input
        if _in_block {
            if trimmed.is_empty() {
                // Empty line ends the block
                _in_block = false;
                let block_text = block_buffer.join("\n");
                block_buffer.clear();

                // Check if this is a function definition
                if block_text.starts_with("define ") && !block_text.starts_with("define main") {
                    user_functions.push(block_text.clone());
                    history.push(block_text);
                    println!("\x1b[2mFunction defined.\x1b[0m");
                } else {
                    accumulated_lines.push(block_text.clone());
                    history.push(block_text);
                }
                continue;
            } else {
                block_buffer.push(line_buffer.trim_end().to_string());
                continue;
            }
        }

        // Handle meta-commands
        if trimmed == "exit" || trimmed == "quit" {
            println!("\x1b[2mGoodbye!\x1b[0m");
            break;
        }

        if trimmed == ":help" {
            println!("\x1b[1mREPL Commands:\x1b[0m");
            println!("  \x1b[36m:help\x1b[0m      Show this help");
            println!("  \x1b[36m:history\x1b[0m   Show command history");
            println!("  \x1b[36m:clear\x1b[0m     Clear history");
            println!("  \x1b[36m:reset\x1b[0m     Clear all session state");
            println!("  \x1b[36mexit\x1b[0m       Exit the REPL");
            println!();
            println!("\x1b[1mSession State:\x1b[0m");
            println!("  Variables persist across lines (set x to 42, then display x)");
            println!("  Enter a block (define, if, etc.) and end with an empty line");
            println!();
            println!("\x1b[1mExamples:\x1b[0m");
            println!("  \x1b[2mset x to 42\x1b[0m");
            println!("  \x1b[2mdisplay x\x1b[0m");
            println!("  \x1b[2mdisplay concat(\"hello\" and \" world\")\x1b[0m");
            continue;
        }

        if trimmed == ":history" {
            for (i, cmd) in history.iter().enumerate() {
                println!("  \x1b[2m{}\x1b[0m  {}", i + 1, cmd);
            }
            continue;
        }

        if trimmed == ":clear" {
            history.clear();
            println!("History cleared.");
            continue;
        }

        if trimmed == ":reset" {
            accumulated_lines.clear();
            user_functions.clear();
            history.clear();
            println!("Session state and history cleared.");
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }

        // Check if this starts a multi-line block (ends with ':')
        if trimmed.ends_with(':') && (trimmed.starts_with("define ")
            || trimmed.starts_with("if ")
            || trimmed.starts_with("repeat ")
            || trimmed.starts_with("for ")
            || trimmed.starts_with("while "))
        {
            _in_block = true;
            block_buffer.clear();
            block_buffer.push(trimmed.to_string());
            continue;
        }

        // Save to history
        history.push(trimmed.to_string());

        // Check if this is a function definition (single line with body)
        if trimmed.starts_with("define ") && !trimmed.starts_with("define main") {
            user_functions.push(trimmed.to_string());
            println!("\x1b[2mFunction defined.\x1b[0m");
            continue;
        }

        // Add to accumulated lines
        accumulated_lines.push(format!("    {}", trimmed));

        // Build the full source with all accumulated state
        let funcs = user_functions.join("\n\n");
        let body = accumulated_lines.join("\n");
        let source = if funcs.is_empty() {
            format!("define main:\n{}\n    return 0\n", body)
        } else {
            format!("{}\n\ndefine main:\n{}\n    return 0\n", funcs, body)
        };

        // Try to parse and evaluate
        let mut lexer_instance = lexer::Lexer::new(&source);
        let tokens = match lexer_instance.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("\x1b[31mLexer error:\x1b[0m {:?}", e);
                // Remove the last line that caused the error
                accumulated_lines.pop();
                continue;
            }
        };

        let mut parser_instance = parser::Parser::new(tokens);
        let mut program = match parser_instance.parse_program() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("\x1b[31mParse error:\x1b[0m {}", e.message);
                accumulated_lines.pop();
                continue;
            }
        };

        // Optimize
        optimizer::Optimizer::run(&mut program);

        // Generate C code
        let mut cg = codegen::Codegen::new();
        cg.is_test_mode = false;
        let c_code = match cg.generate(&program) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("\x1b[31mCodegen error:\x1b[0m {}", e);
                accumulated_lines.pop();
                continue;
            }
        };

        // Write temp C file
        let tmp_c = "/tmp/ep_repl.c";
        let tmp_bin = "/tmp/ep_repl";
        if let Err(e) = fs::write(tmp_c, &c_code) {
            eprintln!("\x1b[31mError writing temp file:\x1b[0m {}", e);
            accumulated_lines.pop();
            continue;
        }

        // Compile with cc
        let compile = Command::new("cc")
            .args(&[tmp_c, "-o", tmp_bin, "-lpthread", "-lm"])
            .output();

        match compile {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("\x1b[31mCompile error:\x1b[0m {}", stderr);
                    accumulated_lines.pop();
                    continue;
                }
            }
            Err(e) => {
                eprintln!("\x1b[31mFailed to run compiler:\x1b[0m {}", e);
                accumulated_lines.pop();
                continue;
            }
        }

        // Execute
        let run = Command::new(tmp_bin).output();
        match run {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.is_empty() {
                    print!("{}", stdout);
                }
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.is_empty() {
                        eprint!("\x1b[31m{}\x1b[0m", stderr);
                    }
                }
            }
            Err(e) => {
                eprintln!("\x1b[31mExecution error:\x1b[0m {}", e);
            }
        }

        // Cleanup temp files
        let _ = fs::remove_file(tmp_c);
        let _ = fs::remove_file(tmp_bin);
    }
}
