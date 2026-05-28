// Native code generator for Ernos
// Translates AST to ARM64 assembly, then uses system assembler + linker
// This eliminates the need for Clang/GCC — only uses OS-provided tools

use std::collections::HashMap;
use crate::ast::*;

pub struct NativeCodegen {
    asm: String,
    string_literals: Vec<String>,
    label_counter: usize,
    func_labels: HashMap<String, String>,
    // Maps local variable names to stack offsets (negative from FP)
    var_offsets: HashMap<String, i64>,
    stack_size: i64,
    is_macos: bool,
    loop_break_label: Option<String>,
    loop_continue_label: Option<String>,
}

/// Escape a string for use in GAS .asciz directive
fn escape_asm_string(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'"'  => out.push_str("\\\""),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(b as char),
            _ => out.push_str(&format!("\\{:03o}", b)),
        }
    }
    out
}

impl NativeCodegen {
    pub fn new(is_macos: bool) -> Self {
        NativeCodegen {
            asm: String::new(),
            string_literals: Vec::new(),
            label_counter: 0,
            func_labels: HashMap::new(),
            var_offsets: HashMap::new(),
            stack_size: 0,
            is_macos,
            loop_break_label: None,
            loop_continue_label: None,
        }
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        if self.is_macos {
            format!("L_{}_{}", prefix, self.label_counter)
        } else {
            format!(".L_{}_{}", prefix, self.label_counter)
        }
    }

    fn sym(&self, name: &str) -> String {
        if self.is_macos {
            format!("_{}", name)
        } else {
            name.to_string()
        }
    }

    fn emit(&mut self, line: &str) {
        self.asm.push_str(line);
        self.asm.push('\n');
    }

    fn add_string(&mut self, s: &str) -> usize {
        let idx = self.string_literals.len();
        self.string_literals.push(s.to_string());
        idx
    }

    fn get_var_offset(&mut self, name: &str) -> i64 {
        if let Some(&off) = self.var_offsets.get(name) {
            return off;
        }
        self.stack_size += 8;
        let off = -(self.stack_size);
        self.var_offsets.insert(name.to_string(), off);
        off
    }

    /// Generate assembly for the entire program
    pub fn generate(&mut self, program: &Program) -> Result<String, String> {
        // Collect all function names
        for func in &program.functions {
            let label = if func.name == "main" {
                self.sym("main")
            } else {
                format!("{}ep_{}", if self.is_macos { "_" } else { "" }, func.name)
            };
            self.func_labels.insert(func.name.clone(), label);
        }

        // Preamble
        if self.is_macos {
            self.emit(".section __TEXT,__text,regular,pure_instructions");
            self.emit(".build_version macos, 14, 0");
        } else {
            self.emit(".text");
        }
        self.emit(".p2align 2");
        self.emit("");
        self.emit("");

        // Generate each function
        for func in &program.functions {
            self.gen_function(func)?;
        }

        // Entry point wrapper
        let main_sym = self.sym("main");
        self.emit(&format!(".globl {}", main_sym));
        self.emit(&format!("{}:", main_sym));
        self.emit("    stp x29, x30, [sp, #-16]!");
        self.emit("    mov x29, sp");
        let ep_main_sym = self.sym("ep_main");
        self.emit(&format!("    bl {}", ep_main_sym));
        // x0 already has the return value from ep_main
        self.emit("    ldp x29, x30, [sp], #16");
        self.emit("    ret");
        self.emit("");

        // String literals
        if !self.string_literals.is_empty() {
            if self.is_macos {
                self.emit(".section __TEXT,__cstring,cstring_literals");
            } else {
                self.emit(".section .rodata");
            }
            let strings: Vec<_> = self.string_literals.iter().cloned().collect();
            for (i, s) in strings.iter().enumerate() {
                let label = if self.is_macos {
                    format!("_str_{}", i)
                } else {
                    format!(".Lstr_{}", i)
                };
                self.emit(&format!("{}: .asciz \"{}\"", label, escape_asm_string(s)));
            }
            self.emit("");
        }

        // The printf format strings
        if self.is_macos {
            self.emit(".section __TEXT,__cstring,cstring_literals");
            self.emit("_fmt_int: .asciz \"%lld\\n\"");
            self.emit("_fmt_str: .asciz \"%s\\n\"");
        } else {
            self.emit(".section .rodata");
            self.emit(".Lfmt_int: .asciz \"%lld\\n\"");
            self.emit(".Lfmt_str: .asciz \"%s\\n\"");
        }
        self.emit("");

        Ok(self.asm.clone())
    }

    fn gen_function(&mut self, func: &Function) -> Result<(), String> {
        self.var_offsets.clear();
        self.stack_size = 0;

        // Pre-scan to allocate space for all variables
        let mut var_names = Vec::new();
        for param in &func.params {
            var_names.push(param.0.clone());
        }
        self.collect_var_names(&func.body, &mut var_names);

        // Allocate stack space (16-byte aligned)
        for name in &var_names {
            self.get_var_offset(name);
        }
        // Round up to 16-byte alignment, plus 16 for saved FP/LR
        let frame_size = ((self.stack_size + 15) / 16) * 16 + 16;

        let label = if func.name == "main" {
            self.sym("ep_main")
        } else {
            self.func_labels.get(&func.name).cloned().unwrap_or(format!("{}ep_{}", if self.is_macos { "_" } else { "" }, func.name))
        };

        self.emit(&format!(".globl {}", label));
        self.emit(&format!("{}:", label));

        // Prologue
        self.emit(&format!("    sub sp, sp, #{}", frame_size));
        self.emit(&format!("    stp x29, x30, [sp, #{}]", frame_size - 16));
        self.emit(&format!("    add x29, sp, #{}", frame_size - 16));

        // Store parameters (x0-x7) to their stack slots (FP-relative)
        for (i, param) in func.params.iter().enumerate() {
            if i < 8 {
                let off = self.var_offsets.get(&param.0).copied().unwrap_or(0);
                // FP + off (off is negative)
                self.emit(&format!("    str x{}, [x29, #{}]", i, off));
            }
        }

        let cleanup_label = format!("{}_cleanup", label);
        
        // Generate body
        for stmt in &func.body {
            self.gen_statement(stmt, &cleanup_label, frame_size)?;
        }

        // Cleanup and return
        self.emit(&format!("{}:", cleanup_label));
        self.emit(&format!("    ldp x29, x30, [sp, #{}]", frame_size - 16));
        self.emit(&format!("    add sp, sp, #{}", frame_size));
        self.emit("    ret");
        self.emit("");

        Ok(())
    }

    fn collect_var_names(&self, stmts: &[Stmt], names: &mut Vec<String>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(name, _, _) => {
                    if !names.contains(name) {
                        names.push(name.clone());
                    }
                }
                StmtNode::If(_, then_branch, else_branch) => {
                    self.collect_var_names(then_branch, names);
                    if let Some(eb) = else_branch {
                        self.collect_var_names(eb, names);
                    }
                }
                StmtNode::RepeatWhile(_, body) => {
                    self.collect_var_names(body, names);
                }
                StmtNode::ForEach(var, _, body) => {
                    if !names.contains(var) {
                        names.push(var.clone());
                    }
                    self.collect_var_names(body, names);
                }
                _ => {}
            }
        }
    }

    fn var_fp_offset(&self, name: &str) -> i64 {
        self.var_offsets.get(name).copied().unwrap_or(0)
    }

    fn gen_statement(&mut self, stmt: &Stmt, cleanup_label: &str, frame_size: i64) -> Result<(), String> {
        match &stmt.node {
            StmtNode::Display(expr) => {
                // Apple ARM64 ABI: variadic args go on the stack, not in registers
                self.gen_expr(expr, "x8", frame_size)?;

                // Allocate stack space for the variadic argument and store value
                self.emit("    sub sp, sp, #16");
                self.emit("    str x8, [sp]");

                // Load format string into x0
                if matches!(expr.node, ExprNode::StringLiteral(_)) {
                    if self.is_macos {
                        self.emit("    adrp x0, _fmt_str@PAGE");
                        self.emit("    add x0, x0, _fmt_str@PAGEOFF");
                    } else {
                        self.emit("    adrp x0, .Lfmt_str");
                        self.emit("    add x0, x0, :lo12:.Lfmt_str");
                    }
                } else {
                    if self.is_macos {
                        self.emit("    adrp x0, _fmt_int@PAGE");
                        self.emit("    add x0, x0, _fmt_int@PAGEOFF");
                    } else {
                        self.emit("    adrp x0, .Lfmt_int");
                        self.emit("    add x0, x0, :lo12:.Lfmt_int");
                    }
                }
                let printf_sym = self.sym("printf");
                self.emit(&format!("    bl {}", printf_sym));
                self.emit("    add sp, sp, #16");
            }

            StmtNode::Set(name, expr, _) => {
                self.gen_expr(expr, "x0", frame_size)?;
                let off = self.var_fp_offset(name);
                self.emit(&format!("    str x0, [x29, #{}]", off));
            }

            StmtNode::Return(expr) => {
                self.gen_expr(expr, "x0", frame_size)?;
                self.emit(&format!("    b {}", cleanup_label));
            }

            StmtNode::If(cond, then_branch, else_branch) => {
                let else_label = self.new_label("else");
                let end_label = self.new_label("endif");

                self.gen_expr(cond, "x0", frame_size)?;
                self.emit("    cmp x0, #0");
                self.emit(&format!("    b.eq {}", else_label));

                for s in then_branch {
                    self.gen_statement(s, cleanup_label, frame_size)?;
                }
                self.emit(&format!("    b {}", end_label));

                self.emit(&format!("{}:", else_label));
                if let Some(eb) = else_branch {
                    for s in eb {
                        self.gen_statement(s, cleanup_label, frame_size)?;
                    }
                }
                self.emit(&format!("{}:", end_label));
            }

            StmtNode::RepeatWhile(cond, body) => {
                let loop_label = self.new_label("loop");
                let end_label = self.new_label("endloop");

                let prev_break = self.loop_break_label.take();
                let prev_continue = self.loop_continue_label.take();
                self.loop_break_label = Some(end_label.clone());
                self.loop_continue_label = Some(loop_label.clone());

                self.emit(&format!("{}:", loop_label));
                self.gen_expr(cond, "x0", frame_size)?;
                self.emit("    cmp x0, #0");
                self.emit(&format!("    b.eq {}", end_label));

                for s in body {
                    self.gen_statement(s, cleanup_label, frame_size)?;
                }
                self.emit(&format!("    b {}", loop_label));
                self.emit(&format!("{}:", end_label));

                self.loop_break_label = prev_break;
                self.loop_continue_label = prev_continue;
            }

            StmtNode::ExprStmt(expr) => {
                self.gen_expr(expr, "x0", frame_size)?;
            }

            StmtNode::Break => {
                if let Some(ref lbl) = self.loop_break_label {
                    self.emit(&format!("    b {}", lbl));
                }
            }

            StmtNode::Continue => {
                if let Some(ref lbl) = self.loop_continue_label {
                    self.emit(&format!("    b {}", lbl));
                }
            }

            _ => {
                return Err(format!("Native ARM64 backend: unsupported statement type: {:?}", stmt.node));
            }
        }
        Ok(())
    }

    fn gen_expr(&mut self, expr: &Expr, dest: &str, frame_size: i64) -> Result<(), String> {
        match &expr.node {
            ExprNode::Integer(n) => {
                if *n >= 0 && *n <= 0xFFFF {
                    self.emit(&format!("    mov {}, #{}", dest, n));
                } else {
                    // Load large immediate via movz + movk
                    let v = *n as u64;
                    self.emit(&format!("    movz {}, #{}", dest, v & 0xFFFF));
                    if v > 0xFFFF {
                        self.emit(&format!("    movk {}, #{}, lsl #16", dest, (v >> 16) & 0xFFFF));
                    }
                    if v > 0xFFFFFFFF {
                        self.emit(&format!("    movk {}, #{}, lsl #32", dest, (v >> 32) & 0xFFFF));
                    }
                    if v > 0xFFFFFFFFFFFF {
                        self.emit(&format!("    movk {}, #{}, lsl #48", dest, (v >> 48) & 0xFFFF));
                    }
                }
            }

            ExprNode::StringLiteral(s) => {
                let idx = self.add_string(s);
                let str_label = if self.is_macos {
                    format!("_str_{}", idx)
                } else {
                    format!(".Lstr_{}", idx)
                };
                if self.is_macos {
                    self.emit(&format!("    adrp {}, {}@PAGE", dest, str_label));
                    self.emit(&format!("    add {}, {}, {}@PAGEOFF", dest, dest, str_label));
                } else {
                    self.emit(&format!("    adrp {}, {}", dest, str_label));
                    self.emit(&format!("    add {}, {}, :lo12:{}", dest, dest, str_label));
                }
            }

            ExprNode::Identifier(name) => {
                let off = self.var_fp_offset(name);
                self.emit(&format!("    ldr {}, [x29, #{}]", dest, off));
            }

            ExprNode::Binary(left, op, right) => {
                // Evaluate left into x9, right into x10, result into dest
                self.gen_expr(left, "x9", frame_size)?;
                // Save x9 on stack temporarily
                self.emit("    str x9, [sp, #-16]!");
                self.gen_expr(right, "x10", frame_size)?;
                self.emit("    ldr x9, [sp], #16");

                match op {
                    Op::Add => self.emit(&format!("    add {}, x9, x10", dest)),
                    Op::Sub => self.emit(&format!("    sub {}, x9, x10", dest)),
                    Op::Mul => self.emit(&format!("    mul {}, x9, x10", dest)),
                    Op::Div => self.emit(&format!("    sdiv {}, x9, x10", dest)),
                    Op::Mod => {
                        self.emit("    sdiv x11, x9, x10");
                        self.emit("    msub x11, x11, x10, x9");
                        self.emit(&format!("    mov {}, x11", dest));
                    }
                }
            }

            ExprNode::Comparison(left, cmp_op, right) => {
                self.gen_expr(left, "x9", frame_size)?;
                self.emit("    str x9, [sp, #-16]!");
                self.gen_expr(right, "x10", frame_size)?;
                self.emit("    ldr x9, [sp], #16");

                self.emit("    cmp x9, x10");
                let cond = match cmp_op {
                    CompOp::Equals => "eq",
                    CompOp::NotEquals => "ne",
                    CompOp::LessThan => "lt",
                    CompOp::GreaterThan => "gt",
                    CompOp::LessEqual => "le",
                    CompOp::GreaterEqual => "ge",
                };
                // Set dest to 1 if condition true, 0 otherwise
                self.emit(&format!("    cset {}, {}", dest, cond));
            }

            ExprNode::Logical(left, log_op, right) => {
                match log_op {
                    LogicalOp::And => {
                        let false_label = self.new_label("and_false");
                        let end_label = self.new_label("and_end");
                        self.gen_expr(left, dest, frame_size)?;
                        self.emit(&format!("    cbz {}, {}", dest, false_label));
                        self.gen_expr(right, dest, frame_size)?;
                        self.emit(&format!("    b {}", end_label));
                        self.emit(&format!("{}:", false_label));
                        self.emit(&format!("    mov {}, #0", dest));
                        self.emit(&format!("{}:", end_label));
                    }
                    LogicalOp::Or => {
                        let true_label = self.new_label("or_true");
                        let end_label = self.new_label("or_end");
                        self.gen_expr(left, dest, frame_size)?;
                        self.emit(&format!("    cbnz {}, {}", dest, true_label));
                        self.gen_expr(right, dest, frame_size)?;
                        self.emit(&format!("    b {}", end_label));
                        self.emit(&format!("{}:", true_label));
                        self.emit(&format!("    mov {}, #1", dest));
                        self.emit(&format!("{}:", end_label));
                    }
                }
            }

            ExprNode::Call(name, args) => {
                // Evaluate args into x0-x7
                // First save args to temp stack slots, then move to x0-x7
                let mut saved = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg, "x0", frame_size)?;
                    if i < args.len() - 1 {
                        // Save to stack for later
                        self.emit("    str x0, [sp, #-16]!");
                        saved.push(i);
                    } else {
                        // Last arg stays in x0, move to correct reg
                        if i > 0 {
                            self.emit(&format!("    mov x{}, x0", i));
                        }
                    }
                }
                // Pop saved args in reverse and place in correct registers
                for &i in saved.iter().rev() {
                    self.emit(&format!("    ldr x{}, [sp], #16", i));
                }

                // Call the function
                let target = self.func_labels.get(name).cloned()
                    .unwrap_or(self.sym(name));
                self.emit(&format!("    bl {}", target));

                if dest != "x0" {
                    self.emit(&format!("    mov {}, x0", dest));
                }
            }

            ExprNode::UnaryNot(inner) => {
                self.gen_expr(inner, dest, frame_size)?;
                self.emit(&format!("    cmp {}, #0", dest));
                self.emit(&format!("    cset {}, eq", dest));
            }

            ExprNode::BoolLiteral(b) => {
                self.emit(&format!("    mov {}, #{}", dest, if *b { 1 } else { 0 }));
            }

            _ => {
                return Err(format!("Native ARM64 backend: unsupported expression type: {:?}", expr.node));
            }
        }
        Ok(())
    }
}
