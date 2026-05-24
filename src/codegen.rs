use crate::ast::{Program, Function, Stmt, Expr, Op, CompOp, LogicalOp};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Int,
    Str,
    DynStr,
    List,
}

pub struct Codegen {
    out: String,
    label_count: usize,
    string_literals: Vec<String>,
    func_return_types: HashMap<String, Type>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            label_count: 0,
            string_literals: Vec::new(),
            func_return_types: HashMap::new(),
        }
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label_count += 1;
        format!("L_{}_{}", prefix, self.label_count)
    }

    fn add_string_literal(&mut self, s: String) -> usize {
        // Return index of existing or new string
        if let Some(pos) = self.string_literals.iter().position(|x| x == &s) {
            pos
        } else {
            self.string_literals.push(s);
            self.string_literals.len() - 1
        }
    }

    fn analyze_return_types(&mut self, program: &Program) {
        self.func_return_types.clear();
        
        // Built-in function return types
        self.func_return_types.insert("read_file_content".to_string(), Type::DynStr);
        self.func_return_types.insert("create_list".to_string(), Type::List);
        self.func_return_types.insert("append_list".to_string(), Type::Int);
        self.func_return_types.insert("get_list".to_string(), Type::Int);
        self.func_return_types.insert("set_list".to_string(), Type::Int);
        self.func_return_types.insert("length_list".to_string(), Type::Int);
        self.func_return_types.insert("string_length".to_string(), Type::Int);
        self.func_return_types.insert("get_character".to_string(), Type::Int);
        self.func_return_types.insert("display_string".to_string(), Type::Int);
        self.func_return_types.insert("get_argument_count".to_string(), Type::Int);
        self.func_return_types.insert("get_argument".to_string(), Type::Str);
        self.func_return_types.insert("write_file_content".to_string(), Type::Int);
        self.func_return_types.insert("run_command".to_string(), Type::Int);
        self.func_return_types.insert("substring".to_string(), Type::DynStr);
        self.func_return_types.insert("string_from_list".to_string(), Type::DynStr);
        self.func_return_types.insert("pop_list".to_string(), Type::Int);
        
        // 3 passes for resolution of dependencies/mutual calls
        for _ in 0..3 {
            for func in &program.functions {
                let mut var_types = HashMap::new();
                for param in &func.params {
                    var_types.insert(param.clone(), Type::Int);
                }
                self.collect_var_types(&func.body, &mut var_types);
                
                let ret = self.determine_ret_type(&func.body, &var_types).unwrap_or(Type::Int);
                self.func_return_types.insert(func.name.clone(), ret);
            }
        }
    }

    fn collect_var_types(&self, stmts: &[Stmt], var_types: &mut HashMap<String, Type>) {
        for stmt in stmts {
            match stmt {
                Stmt::Set(name, expr) => {
                    let t = self.infer_type(expr, var_types);
                    var_types.insert(name.clone(), t);
                }
                Stmt::If(_, then_branch, else_branch) => {
                    self.collect_var_types(then_branch, var_types);
                    if let Some(eb) = else_branch {
                        self.collect_var_types(eb, var_types);
                    }
                }
                Stmt::RepeatWhile(_, body) => {
                    self.collect_var_types(body, var_types);
                }
                _ => {}
            }
        }
    }

    fn determine_ret_type(&self, stmts: &[Stmt], var_types: &HashMap<String, Type>) -> Option<Type> {
        for stmt in stmts {
            match stmt {
                Stmt::Return(expr) => {
                    return Some(self.infer_type(expr, var_types));
                }
                Stmt::If(_, then_branch, else_branch) => {
                    if let Some(t) = self.determine_ret_type(then_branch, var_types) {
                        return Some(t);
                    }
                    if let Some(eb) = else_branch {
                        if let Some(t) = self.determine_ret_type(eb, var_types) {
                            return Some(t);
                        }
                    }
                }
                Stmt::RepeatWhile(_, body) => {
                    if let Some(t) = self.determine_ret_type(body, var_types) {
                        return Some(t);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn infer_type(&self, expr: &Expr, var_types: &HashMap<String, Type>) -> Type {
        match expr {
            Expr::Integer(_) => Type::Int,
            Expr::StringLiteral(_) => Type::Str,
            Expr::Identifier(name) => *var_types.get(name).unwrap_or(&Type::Int),
            Expr::Binary(_, _, _) => Type::Int,
            Expr::Comparison(_, _, _) => Type::Int,
            Expr::Logical(_, _, _) => Type::Int,
            Expr::Call(name, _) => {
                *self.func_return_types.get(name).unwrap_or(&Type::Int)
            }
        }
    }

    pub fn generate(&mut self, program: &Program) -> Result<String, String> {
        self.out.clear();
        self.string_literals.clear();
        self.analyze_return_types(program);

        // Write header
        self.out.push_str(".global _main\n");
        self.out.push_str(".align 4\n\n");

        for func in &program.functions {
            self.gen_function(func)?;
        }

        // Write readonly data (format strings and string literals)
        self.out.push_str(".section __TEXT,__cstring,cstring_literals\n");
        self.out.push_str("fmt_int:\n    .asciz \"%lld\\n\"\n");
        self.out.push_str("fmt_str:\n    .asciz \"%s\\n\"\n");

        for (i, lit) in self.string_literals.iter().enumerate() {
            let escaped = lit
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\t", "\\t")
                .replace("\r", "\\r");
            self.out.push_str(&format!("str_lit_{}:\n    .asciz \"{}\"\n", i, escaped));
        }

        Ok(self.out.clone())
    }

    fn gen_function(&mut self, func: &Function) -> Result<(), String> {
        let mut var_offsets = HashMap::new();
        let mut var_types = HashMap::new();
        let mut offset = 16;

        // Map parameters
        for param in &func.params {
            var_offsets.insert(param.clone(), offset);
            var_types.insert(param.clone(), Type::Int); // assume parameter is Int
            offset += 8;
        }

        // Helper to collect variable declarations in function body
        fn collect_vars(
            stmts: &[Stmt],
            var_offsets: &mut HashMap<String, i32>,
            var_types: &mut HashMap<String, Type>,
            offset: &mut i32,
            codegen: &Codegen,
        ) {
            for stmt in stmts {
                match stmt {
                    Stmt::Set(name, expr) => {
                        if !var_offsets.contains_key(name) {
                            var_offsets.insert(name.clone(), *offset);
                            *offset += 8;
                        }
                        let mut t = codegen.infer_type(expr, var_types);
                        if let Expr::Identifier(_) = expr {
                            if t == Type::List {
                                t = Type::Int;
                            } else if t == Type::DynStr {
                                t = Type::Str;
                            }
                        }
                        var_types.insert(name.clone(), t);
                    }
                    Stmt::If(_, then_branch, else_branch) => {
                        collect_vars(then_branch, var_offsets, var_types, offset, codegen);
                        if let Some(eb) = else_branch {
                            collect_vars(eb, var_offsets, var_types, offset, codegen);
                        }
                    }
                    Stmt::RepeatWhile(_, body) => {
                        collect_vars(body, var_offsets, var_types, offset, codegen);
                    }
                    _ => {}
                }
            }
        }

        collect_vars(&func.body, &mut var_offsets, &mut var_types, &mut offset, self);

        // Frame alignment: total local variables + parameters.
        // Stack must be 16-byte aligned. Each local var takes 8 bytes.
        let local_bytes = var_offsets.len() * 8;
        let aligned_local_bytes = ((local_bytes + 15) / 16) * 16;
        let frame_size = 16 + aligned_local_bytes; // 16 bytes for x29/x30 link registers

        // Write function label
        self.out.push_str(&format!("_{}:\n", func.name));

        // Prologue
        self.out.push_str(&format!("    stp x29, x30, [sp, #-{}]!\n", frame_size));
        self.out.push_str("    mov x29, sp\n");

        if func.name == "main" {
            self.out.push_str("    bl _init_ep_args\n");
        }

        // Copy parameters from x0-x7 to their stack locations
        for (i, param) in func.params.iter().enumerate() {
            if i >= 8 {
                return Err(format!("Function '{}' has too many parameters (maximum 8)", func.name));
            }
            let off = var_offsets.get(param).unwrap();
            self.out.push_str(&format!("    str x{}, [x29, {}]\n", i, off));
        }

        // Initialize all local list and dynamic string variables to 0 (NULL) to make cleanups safe
        for (name, off) in &var_offsets {
            let t = var_types.get(name);
            if t == Some(&Type::List) || t == Some(&Type::DynStr) {
                if !func.params.contains(name) {
                    self.out.push_str("    mov x0, #0\n");
                    self.out.push_str(&format!("    str x0, [x29, {}]\n", off));
                }
            }
        }

        let epilogue_label = format!("L_epilogue_{}", func.name);
        let cleanup_label = format!("L_cleanup_{}", func.name);

        // Generate function body
        for stmt in &func.body {
            self.gen_statement(stmt, &var_offsets, &var_types, &cleanup_label)?;
        }

        // Cleanup Block
        self.out.push_str(&format!("{}:\n", cleanup_label));
        // Save return value x0
        self.out.push_str("    str x0, [sp, #-16]!\n");
        // Free all local list and dynamic string variables
        for (name, off) in &var_offsets {
            let t = var_types.get(name);
            if t == Some(&Type::List) {
                if !func.params.contains(name) {
                    self.out.push_str(&format!("    ldr x0, [x29, {}]\n", off));
                    self.out.push_str("    bl _free_list\n");
                }
            } else if t == Some(&Type::DynStr) {
                if !func.params.contains(name) {
                    self.out.push_str(&format!("    ldr x0, [x29, {}]\n", off));
                    self.out.push_str("    bl _free\n");
                }
            }
        }
        // Restore return value x0
        self.out.push_str("    ldr x0, [sp], #16\n");

        // Epilogue
        self.out.push_str(&format!("{}:\n", epilogue_label));
        self.out.push_str("    mov sp, x29\n");
        self.out.push_str(&format!("    ldp x29, x30, [sp], #{}\n", frame_size));
        self.out.push_str("    ret\n\n");

        Ok(())
    }

    fn gen_statement(
        &mut self,
        stmt: &Stmt,
        var_offsets: &HashMap<String, i32>,
        var_types: &HashMap<String, Type>,
        cleanup_label: &str,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Set(name, expr) => {
                self.gen_expr(expr, var_offsets, var_types)?;
                
                // If the variable is Type::List or Type::DynStr, free its current value first before overwriting
                let t = var_types.get(name);
                if t == Some(&Type::List) {
                    let off = var_offsets.get(name).ok_or_else(|| format!("Unknown variable: {}", name))?;
                    self.out.push_str(&format!("    ldr x0, [x29, {}]\n", off));
                    self.out.push_str("    bl _free_list\n");
                } else if t == Some(&Type::DynStr) {
                    let off = var_offsets.get(name).ok_or_else(|| format!("Unknown variable: {}", name))?;
                    self.out.push_str(&format!("    ldr x0, [x29, {}]\n", off));
                    self.out.push_str("    bl _free\n");
                }

                // Pop the result from the stack and store it in variable location
                let off = var_offsets.get(name).ok_or_else(|| format!("Unknown variable: {}", name))?;
                self.out.push_str("    ldr x0, [sp], #16\n");
                self.out.push_str(&format!("    str x0, [x29, {}]\n", off));
            }
            Stmt::Return(expr) => {
                self.gen_expr(expr, var_offsets, var_types)?;
                // Return value is in x0
                self.out.push_str("    ldr x0, [sp], #16\n");

                // Null out returned local variable if it's a Type::List or Type::DynStr (ownership transfer)
                if let Expr::Identifier(name) = expr {
                    let t = var_types.get(name);
                    if t == Some(&Type::List) || t == Some(&Type::DynStr) {
                        if let Some(off) = var_offsets.get(name) {
                            self.out.push_str("    mov x1, #0\n");
                            self.out.push_str(&format!("    str x1, [x29, {}]\n", off));
                        }
                    }
                }

                self.out.push_str(&format!("    b {}\n", cleanup_label));
            }
            Stmt::Display(expr) => {
                let t = self.infer_type(expr, var_types);
                self.gen_expr(expr, var_offsets, var_types)?;
                
                // The variadic argument for printf is already on the stack at [sp].
                // Load the format string into x0.
                match t {
                    Type::Int | Type::List => {
                        self.out.push_str("    adrp x0, fmt_int@PAGE\n");
                        self.out.push_str("    add x0, x0, fmt_int@PAGEOFF\n");
                    }
                    Type::Str | Type::DynStr => {
                        self.out.push_str("    adrp x0, fmt_str@PAGE\n");
                        self.out.push_str("    add x0, x0, fmt_str@PAGEOFF\n");
                    }
                }
                
                // Call printf
                self.out.push_str("    bl _printf\n");
                
                // Clean up the printed value from the stack
                self.out.push_str("    add sp, sp, #16\n");
            }
            Stmt::If(cond, then_branch, else_branch) => {
                let else_lbl = self.new_label("else");
                let end_lbl = self.new_label("end");

                self.gen_expr(cond, var_offsets, var_types)?;
                self.out.push_str("    ldr x0, [sp], #16\n");
                self.out.push_str("    cmp x0, #0\n");
                self.out.push_str(&format!("    b.eq {}\n", else_lbl));

                // Process then branch
                for s in then_branch {
                    self.gen_statement(s, var_offsets, var_types, cleanup_label)?;
                }
                self.out.push_str(&format!("    b {}\n", end_lbl));

                // Process else branch
                self.out.push_str(&format!("{}:\n", else_lbl));
                if let Some(eb) = else_branch {
                    for s in eb {
                        self.gen_statement(s, var_offsets, var_types, cleanup_label)?;
                    }
                }

                self.out.push_str(&format!("{}:\n", end_lbl));
            }
            Stmt::RepeatWhile(cond, body) => {
                let start_lbl = self.new_label("loop_start");
                let end_lbl = self.new_label("loop_end");

                self.out.push_str(&format!("{}:\n", start_lbl));

                // Evaluate condition
                self.gen_expr(cond, var_offsets, var_types)?;
                self.out.push_str("    ldr x0, [sp], #16\n");
                self.out.push_str("    cmp x0, #0\n");
                self.out.push_str(&format!("    b.eq {}\n", end_lbl));

                // Generate loop body
                for s in body {
                    self.gen_statement(s, var_offsets, var_types, cleanup_label)?;
                }

                // Jump back to loop start
                self.out.push_str(&format!("    b {}\n", start_lbl));

                self.out.push_str(&format!("{}:\n", end_lbl));
            }
        }
        Ok(())
    }

    fn gen_expr(
        &mut self,
        expr: &Expr,
        var_offsets: &HashMap<String, i32>,
        var_types: &HashMap<String, Type>,
    ) -> Result<(), String> {
        match expr {
            Expr::Integer(val) => {
                self.out.push_str(&format!("    ldr x0, ={}\n", val));
                self.out.push_str("    str x0, [sp, #-16]!\n");
            }
            Expr::StringLiteral(s) => {
                let idx = self.add_string_literal(s.clone());
                // Load address of string literal
                self.out.push_str(&format!("    adrp x0, str_lit_{}@PAGE\n", idx));
                self.out.push_str(&format!("    add x0, x0, str_lit_{}@PAGEOFF\n", idx));
                self.out.push_str("    str x0, [sp, #-16]!\n");
            }
            Expr::Identifier(name) => {
                let off = var_offsets.get(name).ok_or_else(|| format!("Variable '{}' used before setting", name))?;
                self.out.push_str(&format!("    ldr x0, [x29, {}]\n", off));
                self.out.push_str("    str x0, [sp, #-16]!\n");
            }
            Expr::Binary(left, op, right) => {
                self.gen_expr(left, var_offsets, var_types)?;
                self.gen_expr(right, var_offsets, var_types)?;
                
                // Pop operands: right into x1, left into x0
                self.out.push_str("    ldr x1, [sp], #16\n");
                self.out.push_str("    ldr x0, [sp], #16\n");

                match op {
                    Op::Add => self.out.push_str("    add x0, x0, x1\n"),
                    Op::Sub => self.out.push_str("    sub x0, x0, x1\n"),
                    Op::Mul => self.out.push_str("    mul x0, x0, x1\n"),
                    Op::Div => self.out.push_str("    sdiv x0, x0, x1\n"),
                }
                
                // Push result
                self.out.push_str("    str x0, [sp, #-16]!\n");
            }
            Expr::Comparison(left, op, right) => {
                let is_string = self.infer_type(&*left, var_types) == Type::Str || self.infer_type(&*left, var_types) == Type::DynStr;
                
                self.gen_expr(left, var_offsets, var_types)?;
                self.gen_expr(right, var_offsets, var_types)?;

                // Pop operands: right into x1, left into x0
                self.out.push_str("    ldr x1, [sp], #16\n");
                self.out.push_str("    ldr x0, [sp], #16\n");

                if is_string {
                    self.out.push_str("    bl _strcmp\n");
                    self.out.push_str("    cmp x0, #0\n");
                } else {
                    self.out.push_str("    cmp x0, x1\n");
                }

                let cond = match op {
                    CompOp::LessThan => "lt",
                    CompOp::GreaterThan => "gt",
                    CompOp::Equals => "eq",
                    CompOp::NotEquals => "ne",
                };
                
                self.out.push_str(&format!("    cset x0, {}\n", cond));
                self.out.push_str("    str x0, [sp, #-16]!\n");
            }
            Expr::Call(name, args) => {
                // Evaluate arguments and push them to stack
                for arg in args {
                    self.gen_expr(arg, var_offsets, var_types)?;
                }

                // Pop them into parameter registers x0-x7 in reverse order
                for i in (0..args.len()).rev() {
                    if i >= 8 {
                        return Err(format!("Too many arguments to function call '{}' (maximum 8)", name));
                    }
                    self.out.push_str(&format!("    ldr x{}, [sp], #16\n", i));
                }

                // Call the function
                self.out.push_str(&format!("    bl _{}\n", name));
                
                // Push result (return value is in x0)
                self.out.push_str("    str x0, [sp, #-16]!\n");
            }
            Expr::Logical(left, op, right) => {
                match op {
                    LogicalOp::And => {
                        let false_lbl = self.new_label("logical_false");
                        let end_lbl = self.new_label("logical_end");

                        // Evaluate left side
                        self.gen_expr(left, var_offsets, var_types)?;
                        self.out.push_str("    ldr x0, [sp], #16\n");
                        self.out.push_str("    cmp x0, #0\n");
                        self.out.push_str(&format!("    b.eq {}\n", false_lbl));

                        // Evaluate right side
                        self.gen_expr(right, var_offsets, var_types)?;
                        self.out.push_str("    ldr x0, [sp], #16\n");
                        self.out.push_str("    cmp x0, #0\n");
                        self.out.push_str(&format!("    b.eq {}\n", false_lbl));

                        // If both true, result is 1
                        self.out.push_str("    mov x0, #1\n");
                        self.out.push_str(&format!("    b {}\n", end_lbl));

                        // False label
                        self.out.push_str(&format!("{}:\n", false_lbl));
                        self.out.push_str("    mov x0, #0\n");

                        // End label
                        self.out.push_str(&format!("{}:\n", end_lbl));
                        self.out.push_str("    str x0, [sp, #-16]!\n");
                    }
                    LogicalOp::Or => {
                        let true_lbl = self.new_label("logical_true");
                        let end_lbl = self.new_label("logical_end");

                        // Evaluate left side
                        self.gen_expr(left, var_offsets, var_types)?;
                        self.out.push_str("    ldr x0, [sp], #16\n");
                        self.out.push_str("    cmp x0, #0\n");
                        self.out.push_str(&format!("    b.ne {}\n", true_lbl));

                        // Evaluate right side
                        self.gen_expr(right, var_offsets, var_types)?;
                        self.out.push_str("    ldr x0, [sp], #16\n");
                        self.out.push_str("    cmp x0, #0\n");
                        self.out.push_str(&format!("    b.ne {}\n", true_lbl));

                        // If both false, result is 0
                        self.out.push_str("    mov x0, #0\n");
                        self.out.push_str(&format!("    b {}\n", end_lbl));

                        // True label
                        self.out.push_str(&format!("{}:\n", true_lbl));
                        self.out.push_str("    mov x0, #1\n");

                        // End label
                        self.out.push_str(&format!("{}:\n", end_lbl));
                        self.out.push_str("    str x0, [sp, #-16]!\n");
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn compile_helper(src: &str) -> String {
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut codegen = Codegen::new();
        codegen.generate(&program).unwrap()
    }

    #[test]
    fn test_codegen_basic() {
        let asm = compile_helper(
            "define main:\n    set x to 10 plus 20\n    display x\n    return 0"
        );
        
        // Assert that the generated assembly contains the function labels, mathematical operations, and printf calls
        assert!(asm.contains("_main:"));
        assert!(asm.contains("add x0, x0, x1"));
        assert!(asm.contains("bl _printf"));
        assert!(asm.contains("fmt_int:"));
    }

    #[test]
    fn test_codegen_loop() {
        let asm = compile_helper(
            "define main:\n    set x to 1\n    repeat while x is less than 5:\n        set x to x plus 1\n    return 0"
        );
        
        assert!(asm.contains("L_loop_start_1:"));
        assert!(asm.contains("L_loop_end_2:"));
        assert!(asm.contains("b.eq L_loop_end_2"));
        assert!(asm.contains("b L_loop_start_1"));
    }

    #[test]
    fn test_codegen_logical() {
        let asm = compile_helper(
            "define main:\n    set x to 10\n    set y to 20\n    if x equals 10 and also y equals 20:\n        display x\n    return 0"
        );
        
        assert!(asm.contains("L_logical_false_"));
        assert!(asm.contains("L_logical_end_"));
        assert!(asm.contains("b.eq L_logical_false_"));
    }

    #[test]
    fn test_codegen_memory_cleanup() {
        let asm = compile_helper(
            "define helper:\n    set a to create_list()\n    set b to a\n    set a to create_list()\n    return a"
        );

        // a should be initialized to 0 in prologue
        assert!(asm.contains("mov x0, #0"));
        
        // Should free previous value of a before reassignment
        assert!(asm.contains("bl _free_list"));
        
        // return a should null out a to transfer ownership
        assert!(asm.contains("mov x1, #0"));
        
        // cleanup should free variables of type List
        assert!(asm.contains("L_cleanup_helper:"));
    }

    #[test]
    fn test_codegen_string_cleanup() {
        let asm = compile_helper(
            "define helper:\n    set a to read_file_content(\"hello.ep\")\n    set b to a\n    set a to read_file_content(\"hello.ep\")\n    return a"
        );

        // a should be initialized to 0 in prologue
        assert!(asm.contains("mov x0, #0"));
        
        // Should free previous value of a before reassignment using standard free
        assert!(asm.contains("bl _free"));
        
        // return a should null out a to transfer ownership
        assert!(asm.contains("mov x1, #0"));
        
        // cleanup should free variables of type DynStr
        assert!(asm.contains("L_cleanup_helper:"));
    }

    #[test]
    fn test_codegen_bootstrapping_extensions() {
        let asm = compile_helper(
            "define main:\n    set a to \"hello\"\n    set b to \"world\"\n    if a equals b:\n        display a\n    return 0"
        );

        // main should call init_ep_args
        assert!(asm.contains("bl _init_ep_args"));
        
        // comparing two strings should use strcmp
        assert!(asm.contains("bl _strcmp"));
        assert!(asm.contains("cmp x0, #0"));
    }
}
