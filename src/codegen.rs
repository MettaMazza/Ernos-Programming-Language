use crate::ast::{Program, Function, StructDef, EnumDef, MethodDef, TypeAnnotation, Stmt, StmtNode, Expr, ExprNode, Op, CompOp, LogicalOp};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Int,
    Float,
    Bool,
    Str,
    DynStr,
    List,
    Map,
    RefList,
    RefStr,
    Struct(String),
    Enum(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OwnerState {
    Owned,
    Moved,
}

fn is_tracked(t: &Type) -> bool {
    matches!(t, Type::List | Type::Map | Type::Str | Type::DynStr | Type::RefList | Type::RefStr | Type::Struct(_) | Type::Enum(_))
}

fn expr_contains_var(expr: &Expr, var_name: &str) -> bool {
    match &expr.node {
        ExprNode::Identifier(name) => name == var_name,
        ExprNode::Binary(l, _, r) | ExprNode::Comparison(l, _, r) | ExprNode::Logical(l, _, r) => {
            expr_contains_var(l, var_name) || expr_contains_var(r, var_name)
        }
        ExprNode::Call(_, args) => {
            args.iter().any(|arg| expr_contains_var(arg, var_name))
        }
        ExprNode::MethodCall(obj, _, args) => {
            expr_contains_var(obj, var_name) || args.iter().any(|arg| expr_contains_var(arg, var_name))
        }
        ExprNode::FieldAccess(obj, _) => {
            expr_contains_var(obj, var_name)
        }
        ExprNode::StructCreate(_, field_inits) => {
            field_inits.iter().any(|(_, fexpr)| expr_contains_var(fexpr, var_name))
        }
        ExprNode::EnumCreate(_, _, args) => {
            args.iter().any(|arg| expr_contains_var(arg, var_name))
        }
        ExprNode::UnaryNot(inner) | ExprNode::TryExpr(inner) | ExprNode::Await(inner) | ExprNode::Borrow(inner) | ExprNode::Receive(inner) => {
            expr_contains_var(inner, var_name)
        }
        ExprNode::ListLiteral(elements) => {
            elements.iter().any(|el| expr_contains_var(el, var_name))
        }
        _ => false,
    }
}

fn is_primitive_op_expr(expr: &Expr, param_name: &str) -> bool {
    match &expr.node {
        ExprNode::Integer(_) | ExprNode::FloatLiteral(_) | ExprNode::BoolLiteral(_) | ExprNode::StringLiteral(_) => true,
        ExprNode::Identifier(_name) => {
            // The identifier itself is fine as long as we only use it in primitive context
            true
        }
        ExprNode::Binary(l, _, r) | ExprNode::Comparison(l, _, r) | ExprNode::Logical(l, _, r) => {
            is_primitive_op_expr(l, param_name) && is_primitive_op_expr(r, param_name)
        }
        ExprNode::UnaryNot(inner) => {
            is_primitive_op_expr(inner, param_name)
        }
        ExprNode::Call(name, args) => {
            let primitive_arg_builtins = [
                "int_to_string", "int_to_float", "float_to_int", "char_from_code", "ep_abs",
                "ep_sleep_ms", "sleep_ms", "ep_time_ms", "ep_random_int"
            ];
            if primitive_arg_builtins.contains(&name.as_str()) {
                args.iter().all(|arg| is_primitive_op_expr(arg, param_name))
            } else {
                !expr_contains_var(expr, param_name)
            }
        }
        _ => {
            !expr_contains_var(expr, param_name)
        }
    }
}

fn stmt_contains_non_primitive_usage(stmt: &Stmt, param_name: &str) -> bool {
    match &stmt.node {
        StmtNode::Set(_, expr, _) => {
            !is_primitive_op_expr(expr, param_name)
        }
        StmtNode::If(cond, then_b, else_b) => {
            if !is_primitive_op_expr(cond, param_name) { return true; }
            then_b.iter().any(|s| stmt_contains_non_primitive_usage(s, param_name)) ||
            else_b.as_ref().map_or(false, |eb| eb.iter().any(|s| stmt_contains_non_primitive_usage(s, param_name)))
        }
        StmtNode::RepeatWhile(cond, body) => {
            if !is_primitive_op_expr(cond, param_name) { return true; }
            body.iter().any(|s| stmt_contains_non_primitive_usage(s, param_name))
        }
        StmtNode::ForEach(loop_var, iterable, body) => {
            if loop_var == param_name { return true; }
            if !is_primitive_op_expr(iterable, param_name) { return true; }
            body.iter().any(|s| stmt_contains_non_primitive_usage(s, param_name))
        }
        StmtNode::Return(expr) | StmtNode::Display(expr) | StmtNode::ExprStmt(expr) => {
            !is_primitive_op_expr(expr, param_name)
        }
        StmtNode::Send(expr, body) => {
            expr_contains_var(expr, param_name) || expr_contains_var(body, param_name)
        }
        StmtNode::FieldSet(obj, _, val) => {
            expr_contains_var(obj, param_name) || expr_contains_var(val, param_name)
        }
        StmtNode::Match(expr, arms) => {
            if expr_contains_var(expr, param_name) { return true; }
            for (_, bindings, body) in arms {
                if bindings.contains(&param_name.to_string()) { return true; }
                if body.iter().any(|s| stmt_contains_non_primitive_usage(s, param_name)) { return true; }
            }
            false
        }
        _ => false,
    }
}

/// Whether a local variable needs to be registered as a GC root.
/// Only root variables whose inferred type could hold a heap pointer.
/// Int, Float, and Bool variables are never heap pointers and can skip
/// GC rooting entirely. This eliminates all GC overhead for pure-compute
/// functions like fib() that only use integer parameters.
fn needs_gc_root(t: &Type) -> bool {
    match t {
        Type::Int | Type::Float | Type::Bool => false,
        _ => true,
    }
}

pub struct Codegen {
    out: String,
    func_return_types: HashMap<String, Type>,
    current_return_type: Type,
    spawn_index: usize,
    spawn_wrapper_index: usize,
    pub is_test_mode: bool,
    struct_defs: HashMap<String, StructDef>,
    enum_defs: HashMap<String, EnumDef>,
    /// Maps variant name -> enum name for quick lookup
    variant_to_enum: HashMap<String, String>,
    /// Maps list variable name -> element type (for string list display)
    list_element_types: HashMap<String, Type>,
    /// Maps closure variable name -> generated C function name
    closure_c_names: HashMap<String, String>,
    /// Set by the Set handler to pass the variable name to Closure codegen
    pending_closure_name: Option<String>,
    /// Maps closure C function name -> list of captured variable names from outer scope
    closure_captures: HashMap<String, Vec<String>>,
    /// Set of C runtime builtin function names (used to skip conflicting stdlib imports)
    builtin_c_funcs: std::collections::HashSet<String>,
    /// Set of top-level constant names (emitted as C globals, not re-declared as locals)
    global_constants: std::collections::HashSet<String>,
    is_async_func: bool,
    current_async_func_locals: std::collections::HashSet<String>,
    await_counter: usize,
    trait_impls: std::collections::HashSet<(String, String)>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            func_return_types: HashMap::new(),
            current_return_type: Type::Int,
            spawn_index: 0,
            spawn_wrapper_index: 0,
            is_test_mode: false,
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            variant_to_enum: HashMap::new(),
            list_element_types: HashMap::new(),
            closure_c_names: HashMap::new(),
            pending_closure_name: None,
            closure_captures: HashMap::new(),
            builtin_c_funcs: std::collections::HashSet::new(),
            global_constants: std::collections::HashSet::new(),
            is_async_func: false,
            current_async_func_locals: std::collections::HashSet::new(),
            await_counter: 0,
            trait_impls: std::collections::HashSet::new(),
        }
    }

    /// Sanitize an ErnosPlain identifier to avoid C reserved word collisions
    fn sanitize_c_name(name: &str) -> String {
        const C_KEYWORDS: &[&str] = &[
            "auto", "break", "case", "char", "const", "continue", "default",
            "do", "double", "else", "enum", "extern", "float", "for", "goto",
            "if", "int", "long", "register", "return", "short", "signed",
            "sizeof", "static", "struct", "switch", "typedef", "union",
            "unsigned", "void", "volatile", "while",
            // C99+
            "inline", "restrict", "_Bool", "_Complex", "_Imaginary",
            // C11+
            "_Alignas", "_Alignof", "_Atomic", "_Generic", "_Noreturn",
            "_Static_assert", "_Thread_local",
            // Common clashes
            "main", "printf", "scanf", "malloc", "free", "exit",
            "read", "write", "open", "close", "send", "recv",
            "select", "remove", "rename", "time", "sleep",
        ];
        if C_KEYWORDS.contains(&name) {
            format!("ep_{}", name)
        } else {
            name.to_string()
        }
    }

    fn count_awaits_in_expr(&self, expr: &Expr) -> usize {
        let mut count = 0;
        match &expr.node {
            ExprNode::Await(inner) => {
                count += 1 + self.count_awaits_in_expr(inner);
            }
            ExprNode::Binary(left, _, right) | ExprNode::Comparison(left, _, right) | ExprNode::Logical(left, _, right) => {
                count += self.count_awaits_in_expr(left) + self.count_awaits_in_expr(right);
            }
            ExprNode::UnaryNot(inner) | ExprNode::TryExpr(inner) | ExprNode::Borrow(inner) | ExprNode::Receive(inner) | ExprNode::FieldAccess(inner, _) => {
                count += self.count_awaits_in_expr(inner);
            }
            ExprNode::Call(_, args) | ExprNode::MethodCall(_, _, args) | ExprNode::EnumCreate(_, _, args) | ExprNode::ListLiteral(args) => {
                for arg in args {
                    count += self.count_awaits_in_expr(arg);
                }
            }
            ExprNode::StructCreate(_, fields) => {
                for (_, f_expr) in fields {
                    count += self.count_awaits_in_expr(f_expr);
                }
            }
            _ => {}
        }
        count
    }

    fn count_awaits_in_stmts(&self, stmts: &[Stmt]) -> usize {
        let mut count = 0;
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(_, expr, _) | StmtNode::ExprStmt(expr) | StmtNode::Display(expr) | StmtNode::Return(expr) => {
                    count += self.count_awaits_in_expr(expr);
                }
                StmtNode::If(cond, then_branch, else_branch) => {
                    count += self.count_awaits_in_expr(cond);
                    count += self.count_awaits_in_stmts(then_branch);
                    if let Some(else_stmts) = else_branch {
                        count += self.count_awaits_in_stmts(else_stmts);
                    }
                }
                StmtNode::RepeatWhile(cond, body) => {
                    count += self.count_awaits_in_expr(cond);
                    count += self.count_awaits_in_stmts(body);
                }
                StmtNode::ForEach(_, list_expr, body) => {
                    count += self.count_awaits_in_expr(list_expr);
                    count += self.count_awaits_in_stmts(body);
                }
                StmtNode::Match(expr, arms) => {
                    count += self.count_awaits_in_expr(expr);
                    for (_, _, body) in arms {
                        count += self.count_awaits_in_stmts(body);
                    }
                }
                _ => {}
            }
        }
        count
    }

    fn emit_yields_for_expr(&mut self, expr: &Expr, var_types: &HashMap<String, Type>) -> Result<(), String> {
        match &expr.node {
            ExprNode::Await(inner) => {
                self.emit_yields_for_expr(inner, var_types)?;
                self.await_counter += 1;
                let inner_str = self.gen_expr(inner, var_types)?;
                let next_state = self.await_counter;
                
                self.out.push_str(&format!(
                    "            {{\n\
                     \x20               EpFuture* _f = (EpFuture*)({inner});\n\
                     \x20               args->awaited_fut_{id} = _f;\n\
                     \x20               if (_f && !_f->completed) {{\n\
                     \x20                   args->state = {next_state};\n\
                     \x20                   _f->waiting_task = ep_current_task;\n\
                     \x20                   return -999999;\n\
                     \x20               }}\n\
                     \x20           }}\n\
                     \x20           case {next_state}:\n",
                    inner = inner_str,
                    id = self.await_counter,
                    next_state = next_state
                ));
            }
            ExprNode::Binary(left, _, right) | ExprNode::Comparison(left, _, right) | ExprNode::Logical(left, _, right) => {
                self.emit_yields_for_expr(left, var_types)?;
                self.emit_yields_for_expr(right, var_types)?;
            }
            ExprNode::UnaryNot(inner) | ExprNode::TryExpr(inner) | ExprNode::Borrow(inner) | ExprNode::Receive(inner) | ExprNode::FieldAccess(inner, _) => {
                self.emit_yields_for_expr(inner, var_types)?;
            }
            ExprNode::Call(_, args) | ExprNode::MethodCall(_, _, args) | ExprNode::EnumCreate(_, _, args) | ExprNode::ListLiteral(args) => {
                for arg in args {
                    self.emit_yields_for_expr(arg, var_types)?;
                }
            }
            ExprNode::StructCreate(_, fields) => {
                for (_, f_expr) in fields {
                    self.emit_yields_for_expr(f_expr, var_types)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_yields_for_statement(&mut self, stmt: &Stmt, var_types: &HashMap<String, Type>) -> Result<(), String> {
        match &stmt.node {
            StmtNode::Set(_, expr, _) | StmtNode::ExprStmt(expr) | StmtNode::Display(expr) | StmtNode::Return(expr) => {
                self.emit_yields_for_expr(expr, var_types)?;
            }
            StmtNode::If(cond, then_branch, else_branch) => {
                self.emit_yields_for_expr(cond, var_types)?;
                for s in then_branch {
                    self.emit_yields_for_statement(s, var_types)?;
                }
                if let Some(else_stmts) = else_branch {
                    for s in else_stmts {
                        self.emit_yields_for_statement(s, var_types)?;
                    }
                }
            }
            StmtNode::RepeatWhile(cond, body) => {
                self.emit_yields_for_expr(cond, var_types)?;
                for s in body {
                    self.emit_yields_for_statement(s, var_types)?;
                }
            }
            StmtNode::ForEach(_, list_expr, body) => {
                self.emit_yields_for_expr(list_expr, var_types)?;
                for s in body {
                    self.emit_yields_for_statement(s, var_types)?;
                }
            }
            StmtNode::Match(expr, arms) => {
                self.emit_yields_for_expr(expr, var_types)?;
                for (_, _, body) in arms {
                    for s in body {
                        self.emit_yields_for_statement(s, var_types)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn type_annotation_to_type(&self, ann: &TypeAnnotation) -> Type {
        match ann {
            TypeAnnotation::Int => Type::Int,
            TypeAnnotation::Float => Type::Float,
            TypeAnnotation::Bool => Type::Bool,
            TypeAnnotation::Str => Type::Str,
            TypeAnnotation::DynStr => Type::DynStr,
            TypeAnnotation::List => Type::List,
            TypeAnnotation::UserDefined(name) => {
                if name == "Any" {
                    Type::Int
                } else if self.enum_defs.contains_key(name) {
                    Type::Enum(name.clone())
                } else {
                    Type::Struct(name.clone())
                }
            }
            TypeAnnotation::Generic(name, _) => {
                if self.enum_defs.contains_key(name) {
                    Type::Enum(name.clone())
                } else {
                    Type::Struct(name.clone())
                }
            }
        }
    }

    fn analyze_return_types(&mut self, program: &Program) {
        self.func_return_types.clear();
        
        self.func_return_types.insert("read_file_content".to_string(), Type::DynStr);
        self.func_return_types.insert("create_list".to_string(), Type::List);
        self.func_return_types.insert("ep_md5".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_sha256".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_hmac_sha256".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_net_connect".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_listen".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_accept".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_send".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_recv".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_net_close".to_string(), Type::Int);
        self.func_return_types.insert("append_list".to_string(), Type::Int);
        self.func_return_types.insert("get_list".to_string(), Type::Int);
        self.func_return_types.insert("set_list".to_string(), Type::Int);
        self.func_return_types.insert("remove_list".to_string(), Type::Int);
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
        self.func_return_types.insert("get_list_data_ptr".to_string(), Type::Int);
        self.func_return_types.insert("sqlite_get_callback_ptr".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_open".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_close".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_exec".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_prepare_v2".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_bind_text".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_bind_int".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_step".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_column_count".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_column_text".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_sqlite3_column_int".to_string(), Type::Int);
        self.func_return_types.insert("ep_sqlite3_finalize".to_string(), Type::Int);
        self.func_return_types.insert("free_list".to_string(), Type::Int);
        self.func_return_types.insert("create_map".to_string(), Type::Map);
        self.func_return_types.insert("map_insert".to_string(), Type::Int);
        self.func_return_types.insert("map_get_val".to_string(), Type::Int);
        self.func_return_types.insert("map_contains".to_string(), Type::Int);
        self.func_return_types.insert("map_delete".to_string(), Type::Int);
        self.func_return_types.insert("map_keys".to_string(), Type::List);
        self.func_return_types.insert("map_size".to_string(), Type::Int);
        self.func_return_types.insert("map_values".to_string(), Type::List);
        self.func_return_types.insert("free_map".to_string(), Type::Int);
        self.func_return_types.insert("map_set_str".to_string(), Type::Int);
        self.func_return_types.insert("map_get_str".to_string(), Type::DynStr);
        self.func_return_types.insert("create_deque".to_string(), Type::Int);
        self.func_return_types.insert("deque_push_back".to_string(), Type::Int);
        self.func_return_types.insert("deque_push_front".to_string(), Type::Int);
        self.func_return_types.insert("deque_pop_back".to_string(), Type::Int);
        self.func_return_types.insert("deque_pop_front".to_string(), Type::Int);
        self.func_return_types.insert("deque_length".to_string(), Type::Int);
        self.func_return_types.insert("free_deque".to_string(), Type::Int);
        self.func_return_types.insert("fs_scan_dir".to_string(), Type::List);
        self.func_return_types.insert("fs_copy_file".to_string(), Type::Int);
        self.func_return_types.insert("fs_delete_file".to_string(), Type::Int);
        self.func_return_types.insert("fs_move_file".to_string(), Type::Int);
        self.func_return_types.insert("fs_exists".to_string(), Type::Int);
        self.func_return_types.insert("fs_is_dir".to_string(), Type::Int);
        self.func_return_types.insert("fs_is_file".to_string(), Type::Int);
        self.func_return_types.insert("fs_get_size".to_string(), Type::Int);
        self.func_return_types.insert("ep_http_request".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_sleep_ms".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlopen".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlsym".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlclose".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall0".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall1".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall2".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall3".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall4".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall5".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall6".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall7".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall8".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall9".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall10".to_string(), Type::Int);
        // Float FFI return types
        self.func_return_types.insert("ep_dlcall_f0".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_f1".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_f2".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_f3".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_f4".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_f5".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_f6".to_string(), Type::Float);
        self.func_return_types.insert("ep_dlcall_fd1".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall_fd2".to_string(), Type::Int);
        self.func_return_types.insert("ep_dlcall_fd3".to_string(), Type::Int);
        self.func_return_types.insert("ep_double_to_bits".to_string(), Type::Int);
        self.func_return_types.insert("ep_bits_to_double".to_string(), Type::Float);
        self.func_return_types.insert("ep_system".to_string(), Type::Int);
        self.func_return_types.insert("ep_play_sound".to_string(), Type::Int);
        self.func_return_types.insert("concat".to_string(), Type::DynStr);
        self.func_return_types.insert("int_to_string".to_string(), Type::DynStr);
        self.func_return_types.insert("read_line".to_string(), Type::DynStr);
        self.func_return_types.insert("read_int".to_string(), Type::Int);
        self.func_return_types.insert("read_float".to_string(), Type::Float);
        self.func_return_types.insert("int_to_float".to_string(), Type::Float);
        self.func_return_types.insert("float_to_int".to_string(), Type::Int);
        self.func_return_types.insert("file_read".to_string(), Type::DynStr);
        self.func_return_types.insert("file_write".to_string(), Type::Int);
        self.func_return_types.insert("file_append".to_string(), Type::Int);
        self.func_return_types.insert("file_exists".to_string(), Type::Int);
        self.func_return_types.insert("string_contains".to_string(), Type::Int);
        self.func_return_types.insert("string_index_of".to_string(), Type::Int);
        self.func_return_types.insert("string_replace".to_string(), Type::DynStr);
        self.func_return_types.insert("json_get_string".to_string(), Type::DynStr);
        self.func_return_types.insert("json_get_int".to_string(), Type::Int);
        self.func_return_types.insert("json_get_bool".to_string(), Type::Int);
        self.func_return_types.insert("ep_sha1".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_net_recv_bytes".to_string(), Type::DynStr);
        self.func_return_types.insert("channel_try_recv".to_string(), Type::Int);
        self.func_return_types.insert("channel_has_data".to_string(), Type::Int);
        self.func_return_types.insert("channel_select".to_string(), Type::Int);
        self.func_return_types.insert("ep_auto_to_string".to_string(), Type::DynStr);
        self.func_return_types.insert("string_upper".to_string(), Type::DynStr);
        self.func_return_types.insert("string_lower".to_string(), Type::DynStr);
        self.func_return_types.insert("string_trim".to_string(), Type::DynStr);
        self.func_return_types.insert("string_split".to_string(), Type::List);
        self.func_return_types.insert("string_to_list".to_string(), Type::List);
        self.func_return_types.insert("char_at".to_string(), Type::Int);
        self.func_return_types.insert("char_from_code".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_abs".to_string(), Type::Int);
        self.func_return_types.insert("string_to_int".to_string(), Type::Int);
        self.func_return_types.insert("ep_random_int".to_string(), Type::Int);
        self.func_return_types.insert("ep_time_ms".to_string(), Type::Int);
        self.func_return_types.insert("ep_uuid_v4".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_base64_encode".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_os_name".to_string(), Type::Str);
        self.func_return_types.insert("create_channel".to_string(), Type::Int);

        // Structured Concurrency builtins
        self.func_return_types.insert("create_task_group".to_string(), Type::Int);
        self.func_return_types.insert("add_task_group".to_string(), Type::Int);
        self.func_return_types.insert("wait_task_group".to_string(), Type::List);
        self.func_return_types.insert("async_timeout".to_string(), Type::Int);
        self.func_return_types.insert("cancel_task".to_string(), Type::Int);
        self.func_return_types.insert("sleep_ms".to_string(), Type::Int);
        self.func_return_types.insert("async_wait_readable".to_string(), Type::Int);

        // FFI pointer/byte builtins
        self.func_return_types.insert("str_to_ptr".to_string(), Type::Int);
        self.func_return_types.insert("ptr_to_str".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_int_to_str".to_string(), Type::DynStr);
        self.func_return_types.insert("peek_byte".to_string(), Type::Int);
        self.func_return_types.insert("poke_byte".to_string(), Type::Int);
        self.func_return_types.insert("alloc_bytes".to_string(), Type::Int);
        self.func_return_types.insert("free_bytes".to_string(), Type::Int);
        self.func_return_types.insert("list_to_bytes".to_string(), Type::Int);
        self.func_return_types.insert("bytes_to_list".to_string(), Type::List);
        self.func_return_types.insert("ep_gc_get_minor_count".to_string(), Type::Int);
        self.func_return_types.insert("ep_gc_get_major_count".to_string(), Type::Int);
        self.func_return_types.insert("ep_gc_get_nursery_count".to_string(), Type::Int);

        // Save the set of builtin C runtime names before externals/user funcs are added
        self.builtin_c_funcs = self.func_return_types.keys().cloned().collect();

        for ext in &program.externals {
            if let Some(ref rt) = ext.return_type {
                self.func_return_types.insert(ext.name.clone(), self.type_annotation_to_type(rt));
            } else if !self.func_return_types.contains_key(&ext.name) {
                self.func_return_types.insert(ext.name.clone(), Type::Int);
            }
        }

        // Fixed-point iteration for resolution of dependencies/mutual calls
        let mut changed = true;
        let mut pass = 0;
        while changed && pass < 20 {
            changed = false;
            pass += 1;

            // 1. Top-level functions
            for func in &program.functions {
                // If the function has an explicit return type annotation, use it
                if let Some(ref rt) = func.return_type {
                    let ty = self.type_annotation_to_type(rt);
                    if self.func_return_types.get(&func.name) != Some(&ty) {
                        self.func_return_types.insert(func.name.clone(), ty);
                        changed = true;
                    }
                    continue;
                }

                let mut var_types = HashMap::new();
                for param in &func.params {
                    let param_type = if let Some(ref ann) = param.2 {
                        self.type_annotation_to_type(ann)
                    } else if param.1 {
                        Type::RefList
                    } else {
                        // Try to infer struct type from field access patterns in the body
                        self.infer_param_struct_type(&param.0, &func.body).unwrap_or(Type::Int)
                    };
                    var_types.insert(param.0.clone(), param_type);
                }
                self.collect_var_types(&func.body, &mut var_types);
                
                let ret = self.determine_ret_type(&func.body, &var_types).unwrap_or(Type::Int);
                if self.func_return_types.get(&func.name) != Some(&ret) {
                    self.func_return_types.insert(func.name.clone(), ret);
                    changed = true;
                }
            }

            // 2. Methods
            for md in &program.method_defs {
                let key = format!("{}_{}", md.struct_name, md.name);
                if let Some(ref rt) = md.return_type {
                    let ty = self.type_annotation_to_type(rt);
                    if self.func_return_types.get(&key) != Some(&ty) {
                        self.func_return_types.insert(key, ty);
                        changed = true;
                    }
                    continue;
                }

                let mut var_types = HashMap::new();
                if self.enum_defs.contains_key(&md.struct_name) {
                    var_types.insert("self".to_string(), Type::Enum(md.struct_name.clone()));
                } else {
                    var_types.insert("self".to_string(), Type::Struct(md.struct_name.clone()));
                }

                for param in &md.params {
                    let param_type = if let Some(ref ann) = param.2 {
                        self.type_annotation_to_type(ann)
                    } else if param.1 {
                        Type::RefList
                    } else {
                        self.infer_param_struct_type(&param.0, &md.body).unwrap_or(Type::Int)
                    };
                    var_types.insert(param.0.clone(), param_type);
                }
                self.collect_var_types(&md.body, &mut var_types);

                let ret = self.determine_ret_type(&md.body, &var_types).unwrap_or(Type::Int);
                if self.func_return_types.get(&key) != Some(&ret) {
                    self.func_return_types.insert(key, ret);
                    changed = true;
                }
            }

            // 3. Trait impl methods
            for ti in &program.trait_impls {
                for m in &ti.methods {
                    let key = format!("{}_{}", ti.for_type, m.name);
                    if let Some(ref rt) = m.return_type {
                        let ty = self.type_annotation_to_type(rt);
                        if self.func_return_types.get(&key) != Some(&ty) {
                            self.func_return_types.insert(key, ty);
                            changed = true;
                        }
                        continue;
                    }

                    let mut var_types = HashMap::new();
                    if self.enum_defs.contains_key(&ti.for_type) {
                        var_types.insert("self".to_string(), Type::Enum(ti.for_type.clone()));
                    } else {
                        var_types.insert("self".to_string(), Type::Struct(ti.for_type.clone()));
                    }

                    for param in &m.params {
                        let param_type = if let Some(ref ann) = param.2 {
                            self.type_annotation_to_type(ann)
                        } else if param.1 {
                            Type::RefList
                        } else {
                            self.infer_param_struct_type(&param.0, &m.body).unwrap_or(Type::Int)
                        };
                        var_types.insert(param.0.clone(), param_type);
                    }
                    self.collect_var_types(&m.body, &mut var_types);

                    let ret = self.determine_ret_type(&m.body, &var_types).unwrap_or(Type::Int);
                    if self.func_return_types.get(&key) != Some(&ret) {
                        self.func_return_types.insert(key, ret);
                        changed = true;
                    }
                }
            }
        }
    }

    /// Collect all field names accessed on a variable via FieldAccess in the AST
    fn collect_field_accesses_expr(&self, param_name: &str, expr: &Expr, fields: &mut Vec<String>) {
        match &expr.node {
            ExprNode::FieldAccess(obj, field_name) => {
                if let ExprNode::Identifier(name) = &obj.node {
                    if name == param_name {
                        fields.push(field_name.clone());
                    }
                }
                self.collect_field_accesses_expr(param_name, obj, fields);
            }
            ExprNode::Binary(l, _, r) | ExprNode::Comparison(l, _, r) | ExprNode::Logical(l, _, r) => {
                self.collect_field_accesses_expr(param_name, l, fields);
                self.collect_field_accesses_expr(param_name, r, fields);
            }
            ExprNode::Call(_, args) => {
                for a in args { self.collect_field_accesses_expr(param_name, a, fields); }
            }
            ExprNode::MethodCall(obj, _, args) => {
                self.collect_field_accesses_expr(param_name, obj, fields);
                for a in args { self.collect_field_accesses_expr(param_name, a, fields); }
            }
            ExprNode::UnaryNot(inner) | ExprNode::TryExpr(inner) | ExprNode::Await(inner) | ExprNode::Borrow(inner) | ExprNode::Receive(inner) => {
                self.collect_field_accesses_expr(param_name, inner, fields);
            }
            ExprNode::StructCreate(_, field_inits) => {
                for (_, fexpr) in field_inits { self.collect_field_accesses_expr(param_name, fexpr, fields); }
            }
            ExprNode::EnumCreate(_, _, args) => {
                for a in args { self.collect_field_accesses_expr(param_name, a, fields); }
            }
            ExprNode::Closure(_, body) => {
                self.collect_field_accesses_stmts(param_name, body, fields);
            }
            _ => {}
        }
    }

    fn collect_field_accesses_stmts(&self, param_name: &str, stmts: &[Stmt], fields: &mut Vec<String>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Display(e) | StmtNode::ExprStmt(e) | StmtNode::Return(e) => {
                    self.collect_field_accesses_expr(param_name, e, fields);
                }
                StmtNode::Set(_, expr, _) => {
                    self.collect_field_accesses_expr(param_name, expr, fields);
                }
                StmtNode::FieldSet(obj, field_name, expr) => {
                    // Also collect the field name from FieldSet (set p.field to ...)
                    if let ExprNode::Identifier(name) = &obj.node {
                        if name == param_name {
                            fields.push(field_name.clone());
                        }
                    }
                    self.collect_field_accesses_expr(param_name, obj, fields);
                    self.collect_field_accesses_expr(param_name, expr, fields);
                }
                StmtNode::If(cond, then_b, else_b) => {
                    self.collect_field_accesses_expr(param_name, cond, fields);
                    self.collect_field_accesses_stmts(param_name, then_b, fields);
                    if let Some(eb) = else_b { self.collect_field_accesses_stmts(param_name, eb, fields); }
                }
                StmtNode::RepeatWhile(cond, body) => {
                    self.collect_field_accesses_expr(param_name, cond, fields);
                    self.collect_field_accesses_stmts(param_name, body, fields);
                }
                StmtNode::ForEach(_, iter_expr, body) => {
                    self.collect_field_accesses_expr(param_name, iter_expr, fields);
                    self.collect_field_accesses_stmts(param_name, body, fields);
                }
                StmtNode::Send(val, chan) => {
                    self.collect_field_accesses_expr(param_name, val, fields);
                    self.collect_field_accesses_expr(param_name, chan, fields);
                }
                StmtNode::Spawn(_, args) => {
                    for a in args { self.collect_field_accesses_expr(param_name, a, fields); }
                }
                StmtNode::Match(expr, arms) => {
                    self.collect_field_accesses_expr(param_name, expr, fields);
                    for (_, _, body) in arms { self.collect_field_accesses_stmts(param_name, body, fields); }
                }
                _ => {}
            }
        }
    }

    /// Try to infer a struct type for a parameter by matching field access patterns
    fn infer_param_struct_type(&self, param_name: &str, body: &[Stmt]) -> Option<Type> {
        let mut accessed_fields = Vec::new();
        self.collect_field_accesses_stmts(param_name, body, &mut accessed_fields);
        if accessed_fields.is_empty() {
            return None;
        }
        // Deduplicate
        accessed_fields.sort();
        accessed_fields.dedup();
        
        // Find structs where ALL accessed fields exist
        let mut candidates = Vec::new();
        for (struct_name, sd) in &self.struct_defs {
            let struct_field_names: Vec<&str> = sd.fields.iter().map(|(n, _, _)| n.as_str()).collect();
            let all_match = accessed_fields.iter().all(|f| struct_field_names.contains(&f.as_str()));
            if all_match {
                candidates.push(struct_name.clone());
            }
        }
        
        if candidates.len() == 1 {
            Some(Type::Struct(candidates[0].clone()))
        } else {
            None // Ambiguous or no match — require explicit annotation
        }
    }

    fn infer_param_types_from_usage(&self, body: &[Stmt], var_types: &mut HashMap<String, Type>) {
        fn scan_expr(expr: &Expr, var_types: &mut HashMap<String, Type>) {
            match &expr.node {
                ExprNode::Call(name, args) => {
                    if name == "length_list" || name == "append_list" || name == "get_list" || 
                       name == "set_list" || name == "remove_list" || name == "pop_list" {
                        if let Some(ExprNode::Identifier(pname)) = args.first().map(|e| &e.node) {
                            if let Some(ty) = var_types.get_mut(pname) {
                                if *ty == Type::Int {
                                    *ty = Type::List;
                                }
                            }
                        }
                    }
                    if name == "map_insert" || name == "map_get_val" || name == "map_contains" || 
                       name == "map_delete" || name == "map_keys" || name == "map_size" || 
                       name == "map_values" || name == "map_set_str" || name == "map_get_str" {
                        if let Some(ExprNode::Identifier(pname)) = args.first().map(|e| &e.node) {
                            if let Some(ty) = var_types.get_mut(pname) {
                                if *ty == Type::Int {
                                    *ty = Type::Map;
                                }
                            }
                        }
                    }
                    if name == "string_length" || name == "get_character" || name == "char_at" ||
                       name == "string_contains" || name == "string_index_of" || name == "string_replace" ||
                       name == "string_upper" || name == "string_lower" || name == "string_trim" ||
                       name == "string_split" || name == "string_to_list" || name == "json_get_string" || name == "json_get_int" ||
                       name == "json_get_bool" {
                        if let Some(ExprNode::Identifier(pname)) = args.first().map(|e| &e.node) {
                            if let Some(ty) = var_types.get_mut(pname) {
                                if *ty == Type::Int {
                                    *ty = Type::Str;
                                }
                            }
                        }
                    }
                    for arg in args {
                        scan_expr(arg, var_types);
                    }
                }
                ExprNode::MethodCall(obj, _, args) => {
                    scan_expr(obj, var_types);
                    for arg in args {
                        scan_expr(arg, var_types);
                    }
                }
                ExprNode::Binary(l, _, r) | ExprNode::Comparison(l, _, r) | ExprNode::Logical(l, _, r) => {
                    scan_expr(l, var_types);
                    scan_expr(r, var_types);
                }
                ExprNode::UnaryNot(inner) | ExprNode::TryExpr(inner) | ExprNode::Await(inner) | ExprNode::Borrow(inner) | ExprNode::Receive(inner) => {
                    scan_expr(inner, var_types);
                }
                ExprNode::StructCreate(_, field_inits) => {
                    for (_, fexpr) in field_inits {
                        scan_expr(fexpr, var_types);
                    }
                }
                ExprNode::EnumCreate(_, _, args) => {
                    for arg in args {
                        scan_expr(arg, var_types);
                    }
                }
                ExprNode::ListLiteral(elements) => {
                    for el in elements {
                        scan_expr(el, var_types);
                    }
                }
                _ => {}
            }
        }

        fn scan_stmts(stmts: &[Stmt], var_types: &mut HashMap<String, Type>) {
            for stmt in stmts {
                match &stmt.node {
                    StmtNode::Set(_, expr, _) => scan_expr(expr, var_types),
                    StmtNode::If(cond, then_b, else_b) => {
                        scan_expr(cond, var_types);
                        scan_stmts(then_b, var_types);
                        if let Some(eb) = else_b {
                            scan_stmts(eb, var_types);
                        }
                    }
                    StmtNode::RepeatWhile(cond, body) => {
                        scan_expr(cond, var_types);
                        scan_stmts(body, var_types);
                    }
                    StmtNode::ForEach(_, iterable, body) => {
                        scan_expr(iterable, var_types);
                        scan_stmts(body, var_types);
                    }
                    StmtNode::Return(expr) | StmtNode::Display(expr) | StmtNode::ExprStmt(expr) => {
                        scan_expr(expr, var_types);
                    }
                    StmtNode::Send(expr, body) => {
                        scan_expr(expr, var_types);
                        scan_expr(body, var_types);
                    }
                    StmtNode::FieldSet(obj, _, val) => {
                        scan_expr(obj, var_types);
                        scan_expr(val, var_types);
                    }
                    StmtNode::Match(expr, arms) => {
                        scan_expr(expr, var_types);
                        for (_, _, body) in arms {
                            scan_stmts(body, var_types);
                        }
                    }
                    _ => {}
                }
            }
        }

        scan_stmts(body, var_types);
    }

    fn collect_var_types(&mut self, stmts: &[Stmt], var_types: &mut HashMap<String, Type>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(name, expr, type_ann) => {
                    let t = if let Some(ann) = type_ann {
                        self.type_annotation_to_type(ann)
                    } else {
                        self.infer_type(expr, var_types)
                    };
                    var_types.insert(name.clone(), t);
                    // Track element type for list literals with string elements
                    if let ExprNode::ListLiteral(elements) = &expr.node {
                        if elements.iter().any(|e| matches!(e.node, ExprNode::StringLiteral(_))) {
                            self.list_element_types.insert(name.clone(), Type::Str);
                        }
                    }
                    // Track element type when append_list(list_var, value) is called
                    // This allows for-each loops to give the loop variable the correct type
                    if let ExprNode::Call(func_name, args) = &expr.node {
                        if func_name == "append_list" && args.len() == 2 {
                            if let ExprNode::Identifier(list_name) = &args[0].node {
                                let elem_type = self.infer_type(&args[1], var_types);
                                match &elem_type {
                                    Type::Struct(_) | Type::Enum(_) | Type::Str => {
                                        self.list_element_types.insert(list_name.clone(), elem_type);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    // Pre-register closure variable → C function name mapping
                    if matches!(expr.node, ExprNode::Closure(_, _)) {
                        let c_name = format!("_ep_closure_{}", Self::sanitize_c_name(name));
                        self.closure_c_names.insert(name.clone(), c_name);
                    }
                }
                StmtNode::FieldSet(_, _, _) => {}
                StmtNode::If(_, then_branch, else_branch) => {
                    self.collect_var_types(then_branch, var_types);
                    if let Some(eb) = else_branch {
                        self.collect_var_types(eb, var_types);
                    }
                }
                StmtNode::RepeatWhile(_, body) => {
                    self.collect_var_types(body, var_types);
                }
                StmtNode::ForEach(loop_var, iterable, body) => {
                    let mut elem_type = Type::Int;
                    let iterable_type = self.infer_type(iterable, var_types);
                    let mut is_iterator = false;
                    let mut iter_type_name = String::new();
                    
                    match &iterable_type {
                        Type::Struct(name) | Type::Enum(name) => {
                            if self.trait_impls.contains(&("Iterator".to_string(), name.clone())) {
                                is_iterator = true;
                                iter_type_name = name.clone();
                            }
                        }
                        _ => {}
                    }
                    
                    if is_iterator {
                        let next_key = format!("{}_next", iter_type_name);
                        if let Some(Type::Enum(ename)) = self.func_return_types.get(&next_key) {
                            if let Some(ed) = self.enum_defs.get(ename) {
                                if let Some((_, fields)) = ed.variants.iter().find(|(vn, _)| vn == "Next") {
                                    if let Some((_, ann)) = fields.first() {
                                        elem_type = self.type_annotation_to_type(ann);
                                    }
                                }
                            }
                        }
                    } else {
                        // Determine loop variable type from the list's element type
                        elem_type = if let ExprNode::Identifier(list_name) = &iterable.node {
                            self.list_element_types.get(list_name).cloned().unwrap_or(Type::Int)
                        } else if let ExprNode::ListLiteral(elements) = &iterable.node {
                            if elements.iter().any(|e| matches!(e.node, ExprNode::StringLiteral(_))) {
                                Type::Str
                            } else {
                                Type::Int
                            }
                        } else {
                            Type::Int
                        };
                    }
                    var_types.insert(loop_var.clone(), elem_type);
                    self.collect_var_types(body, var_types);
                }
                StmtNode::Match(expr, arms) => {
                    let enum_type = self.infer_type(expr, var_types);
                    if let Type::Enum(enum_name) = &enum_type {
                         if let Some(ed) = self.enum_defs.get(enum_name).cloned() {
                            for (variant_name, bindings, body) in arms {
                                if let Some((_, fields)) = ed.variants.iter().find(|(vn, _)| vn == variant_name) {
                                    for (i, binding) in bindings.iter().enumerate() {
                                        if i < fields.len() {
                                            let t = match &fields[i].1 {
                                                TypeAnnotation::Int => Type::Int,
                                                TypeAnnotation::Float => Type::Float,
                                                TypeAnnotation::Bool => Type::Bool,
                                                TypeAnnotation::Str => Type::Str,
                                                TypeAnnotation::DynStr => Type::DynStr,
                                                TypeAnnotation::List => Type::List,
                                                TypeAnnotation::UserDefined(n) => {
                                                    if self.enum_defs.contains_key(n) {
                                                        Type::Enum(n.clone())
                                                    } else {
                                                        Type::Struct(n.clone())
                                                    }
                                                }
                                                TypeAnnotation::Generic(n, _) => {
                                                    if self.enum_defs.contains_key(n) {
                                                        Type::Enum(n.clone())
                                                    } else {
                                                        Type::Struct(n.clone())
                                                    }
                                                }
                                            };
                                            var_types.insert(binding.clone(), t);
                                        }
                                    }
                                }
                                self.collect_var_types(body, var_types);
                            }
                        }
                    } else {
                        for (_, _, body) in arms {
                            self.collect_var_types(body, var_types);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn determine_ret_type(&self, stmts: &[Stmt], var_types: &HashMap<String, Type>) -> Option<Type> {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Return(expr) => {
                    return Some(self.infer_type(expr, var_types));
                }
                StmtNode::If(_, then_branch, else_branch) => {
                    if let Some(t) = self.determine_ret_type(then_branch, var_types) {
                        return Some(t);
                    }
                    if let Some(eb) = else_branch {
                        if let Some(t) = self.determine_ret_type(eb, var_types) {
                            return Some(t);
                        }
                    }
                }
                StmtNode::RepeatWhile(_, body) => {
                    if let Some(t) = self.determine_ret_type(body, var_types) {
                        return Some(t);
                    }
                }
                StmtNode::ForEach(_, _, body) => {
                    if let Some(t) = self.determine_ret_type(body, var_types) {
                        return Some(t);
                    }
                }
                StmtNode::Match(_, arms) => {
                    for (_, _, body) in arms {
                        if let Some(t) = self.determine_ret_type(body, var_types) {
                            return Some(t);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn infer_type(&self, expr: &Expr, var_types: &HashMap<String, Type>) -> Type {
        match &expr.node {
            ExprNode::Integer(_) => Type::Int,
            ExprNode::FloatLiteral(_) => Type::Float,
            ExprNode::BoolLiteral(_) => Type::Bool,
            ExprNode::StringLiteral(_) => Type::Str,
            ExprNode::Identifier(name) => {
                // Check if it's a known enum variant (bare variant without data)
                if let Some(enum_name) = self.variant_to_enum.get(name) {
                    Type::Enum(enum_name.clone())
                } else {
                    var_types.get(name).cloned().unwrap_or(Type::Int)
                }
            }
            ExprNode::Binary(left, _, right) => {
                let lt = self.infer_type(left, var_types);
                let rt = self.infer_type(right, var_types);
                if lt == Type::Float || rt == Type::Float { Type::Float } else { Type::Int }
            }
            ExprNode::Comparison(_, _, _) => Type::Int,
            ExprNode::Logical(_, _, _) => Type::Int,
            ExprNode::Call(name, _) => {
                self.func_return_types.get(name).cloned().unwrap_or(Type::Int)
            }
            ExprNode::Channel => Type::Int,
            ExprNode::Receive(_) => Type::Int,
            ExprNode::Borrow(inner) => {
                let t = self.infer_type(inner, var_types);
                match t {
                    Type::List | Type::RefList => Type::RefList,
                    Type::Str | Type::DynStr | Type::RefStr => Type::RefStr,
                    _ => Type::RefList,
                }
            }
            ExprNode::FieldAccess(obj, field_name) => {
                let obj_type = self.infer_type(obj, var_types);
                if let Type::Struct(struct_name) = obj_type {
                    if let Some(sd) = self.struct_defs.get(&struct_name) {
                        for (fname, ftype, _) in &sd.fields {
                            if fname == field_name {
                                return match ftype {
                                    TypeAnnotation::Int => Type::Int,
                                    TypeAnnotation::Float => Type::Float,
                                    TypeAnnotation::Bool => Type::Bool,
                                    TypeAnnotation::Str => Type::Str,
                                    TypeAnnotation::DynStr => Type::DynStr,
                                    TypeAnnotation::List => Type::List,
                                    TypeAnnotation::UserDefined(name) => Type::Struct(name.clone()),
                                    TypeAnnotation::Generic(name, _) => Type::Struct(name.clone()),
                                };
                            }
                        }
                    }
                }
                Type::Int
            }
            ExprNode::StructCreate(struct_name, _) => Type::Struct(struct_name.clone()),
            ExprNode::EnumCreate(enum_name, variant_name, _) => {
                // If enum_name is empty, look it up from variant_to_enum
                if enum_name.is_empty() {
                    if let Some(en) = self.variant_to_enum.get(variant_name) {
                        Type::Enum(en.clone())
                    } else {
                        Type::Int
                    }
                } else {
                    Type::Enum(enum_name.clone())
                }
            }
            ExprNode::MethodCall(obj, method_name, _) => {
                let obj_type = self.infer_type(obj, var_types);
                let type_name = match &obj_type {
                    Type::Struct(s) => Some(s.clone()),
                    Type::Enum(e) => Some(e.clone()),
                    _ => None,
                };
                if let Some(name) = type_name {
                    let key = format!("{}_{}", name, method_name);
                    self.func_return_types.get(&key).cloned().unwrap_or(Type::Int)
                } else {
                    Type::Int
                }
            }
            ExprNode::UnaryNot(_) => Type::Int,
            ExprNode::TryExpr(inner) => {
                // Preserve the inner expression's type
                self.infer_type(inner, var_types)
            }
            ExprNode::Closure(_, _) => Type::Int, // closure is a function pointer (long long)
            ExprNode::Await(inner) => self.infer_type(inner, var_types),
            ExprNode::ListLiteral(_) => Type::List,
        }
    }

    fn check_expr_reads(
        &self,
        expr: &Expr,
        var_types: &HashMap<String, Type>,
        owner_states: &HashMap<String, OwnerState>,
    ) -> Result<(), String> {
        match &expr.node {
            ExprNode::Integer(_) | ExprNode::FloatLiteral(_) | ExprNode::BoolLiteral(_) | ExprNode::StringLiteral(_) | ExprNode::Channel => Ok(()),
            ExprNode::Identifier(name) => {
                let t = var_types.get(name).cloned().unwrap_or(Type::Int);
                if is_tracked(&t) {
                    if let Some(OwnerState::Moved) = owner_states.get(name) {
                        return Err(format!("Safety Error: Use of moved value: {}", name));
                    }
                }
                Ok(())
            }
            ExprNode::Borrow(inner) => {
                self.check_expr_reads(inner, var_types, owner_states)
            }
            ExprNode::Binary(left, _, right) | ExprNode::Comparison(left, _, right) | ExprNode::Logical(left, _, right) => {
                self.check_expr_reads(left, var_types, owner_states)?;
                self.check_expr_reads(right, var_types, owner_states)?;
                Ok(())
            }
            ExprNode::Call(_, args) => {
                for arg in args {
                    self.check_expr_reads(arg, var_types, owner_states)?;
                }
                Ok(())
            }
            ExprNode::Receive(inner) => {
                self.check_expr_reads(inner, var_types, owner_states)
            }
            ExprNode::FieldAccess(inner, _) => {
                self.check_expr_reads(inner, var_types, owner_states)
            }
            ExprNode::StructCreate(_, fields) => {
                for (_, expr) in fields {
                    self.check_expr_reads(expr, var_types, owner_states)?;
                }
                Ok(())
            }
            ExprNode::EnumCreate(_, _, args) => {
                for arg in args {
                    self.check_expr_reads(arg, var_types, owner_states)?;
                }
                Ok(())
            }
            ExprNode::MethodCall(obj, _, args) => {
                self.check_expr_reads(obj, var_types, owner_states)?;
                for arg in args {
                    self.check_expr_reads(arg, var_types, owner_states)?;
                }
                Ok(())
            }
            ExprNode::UnaryNot(inner) => {
                self.check_expr_reads(inner, var_types, owner_states)?;
                Ok(())
            }
            ExprNode::TryExpr(inner) => {
                self.check_expr_reads(inner, var_types, owner_states)?;
                Ok(())
            }
            ExprNode::Closure(_, _) => Ok(()), // closures don't read from outer scope at safety-check time
            ExprNode::ListLiteral(elements) => {
                for e in elements { self.check_expr_reads(e, var_types, owner_states)?; }
                Ok(())
            }
            ExprNode::Await(inner) => {
                self.check_expr_reads(inner, var_types, owner_states)?;
                Ok(())
            }
        }
    }

    fn check_safety_stmts(
        &self,
        func: &Function,
        stmts: &[Stmt],
        var_types: &HashMap<String, Type>,
        owner_states: &mut HashMap<String, OwnerState>,
        borrows: &mut HashMap<String, String>,
        borrow_counts: &mut HashMap<String, usize>,
    ) -> Result<(), String> {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(name, expr, _type_ann) => {
                    self.check_expr_reads(expr, var_types, owner_states)?;

                    if let Some(old_target) = borrows.remove(name) {
                        if let Some(count) = borrow_counts.get_mut(&old_target) {
                            if *count > 0 {
                                *count -= 1;
                            }
                        }
                    }

                    match &expr.node {
                        ExprNode::Borrow(inner) => {
                            if let ExprNode::Identifier(target) = &inner.node {
                                if let Some(OwnerState::Moved) = owner_states.get(target) {
                                    return Err(format!("Safety Error: Cannot borrow moved variable: {}", target));
                                }
                                borrows.insert(name.clone(), target.clone());
                                *borrow_counts.entry(target.clone()).or_insert(0) += 1;
                            } else {
                                return Err("Safety Error: Expected identifier in borrow expression".to_string());
                            }
                        }
                        _ => {
                            let bc = *borrow_counts.get(name).unwrap_or(&0);
                            if bc > 0 {
                                return Err(format!("Safety Error: Cannot modify variable because it is currently borrowed: {}", name));
                            }

                            let t = var_types.get(name).cloned().unwrap_or(Type::Int);
                            if is_tracked(&t) {
                                if let ExprNode::Identifier(src) = &expr.node {
                                    let src_t = var_types.get(src).cloned().unwrap_or(Type::Int);
                                    if is_tracked(&src_t) {
                                        if let Some(OwnerState::Moved) = owner_states.get(src) {
                                            return Err(format!("Safety Error: Use of moved value: {}", src));
                                        }
                                        let src_bc = *borrow_counts.get(src).unwrap_or(&0);
                                        if src_bc > 0 {
                                            return Err(format!("Safety Error: Cannot move variable because it is currently borrowed: {}", src));
                                        }
                                        if src_t != Type::RefList && src_t != Type::RefStr
                                            && src_t != Type::Str && src_t != Type::DynStr {
                                            owner_states.insert(src.clone(), OwnerState::Moved);
                                        }
                                    }
                                }
                                owner_states.insert(name.clone(), OwnerState::Owned);
                            }
                        }
                    }
                }
                StmtNode::Send(chan, val) => {
                    self.check_expr_reads(chan, var_types, owner_states)?;
                    if let ExprNode::Identifier(src) = &val.node {
                        let src_t = var_types.get(src).cloned().unwrap_or(Type::Int);
                        if is_tracked(&src_t) {
                            if let Some(OwnerState::Moved) = owner_states.get(src) {
                                return Err(format!("Safety Error: Use of moved value: {}", src));
                            }
                            let src_bc = *borrow_counts.get(src).unwrap_or(&0);
                            if src_bc > 0 {
                                return Err(format!("Safety Error: Cannot move variable because it is currently borrowed: {}", src));
                            }
                            if src_t != Type::RefList && src_t != Type::RefStr
                                && src_t != Type::Str && src_t != Type::DynStr {
                                owner_states.insert(src.clone(), OwnerState::Moved);
                            }
                        } else {
                            self.check_expr_reads(val, var_types, owner_states)?;
                        }
                    } else {
                        self.check_expr_reads(val, var_types, owner_states)?;
                    }
                }
                StmtNode::Spawn(_func_name, args) => {
                    for arg in args {
                        if let ExprNode::Identifier(src) = &arg.node {
                            let src_t = var_types.get(src).cloned().unwrap_or(Type::Int);
                            if is_tracked(&src_t) {
                                if let Some(OwnerState::Moved) = owner_states.get(src) {
                                    return Err(format!("Safety Error: Use of moved value: {}", src));
                                }
                                let src_bc = *borrow_counts.get(src).unwrap_or(&0);
                                if src_bc > 0 {
                                    return Err(format!("Safety Error: Cannot move variable because it is currently borrowed: {}", src));
                                }
                                if src_t != Type::RefList && src_t != Type::RefStr
                                    && src_t != Type::Str && src_t != Type::DynStr {
                                    owner_states.insert(src.clone(), OwnerState::Moved);
                                }
                            } else {
                                self.check_expr_reads(arg, var_types, owner_states)?;
                            }
                        } else {
                            self.check_expr_reads(arg, var_types, owner_states)?;
                        }
                    }
                }
                StmtNode::Display(expr) => {
                    self.check_expr_reads(expr, var_types, owner_states)?;
                }
                StmtNode::Return(expr) => {
                    self.check_expr_reads(expr, var_types, owner_states)?;
                    if let ExprNode::Borrow(inner) = &expr.node {
                        if let ExprNode::Identifier(target) = &inner.node {
                            let is_borrowed_param = func.params.iter().any(|(p_name, is_borrow, _)| {
                                p_name == target && *is_borrow
                            });
                            if !is_borrowed_param {
                                return Err(format!("Safety Error: Cannot return reference to local variable: {}", target));
                            }
                        }
                    } else if let ExprNode::Identifier(name) = &expr.node {
                        let t = var_types.get(name).cloned().unwrap_or(Type::Int);
                        if t == Type::RefList || t == Type::RefStr {
                            let target = borrows.get(name).unwrap_or(name);
                            let is_borrowed_param = func.params.iter().any(|(p_name, is_borrow, _)| {
                                p_name == target && *is_borrow
                            });
                            if !is_borrowed_param {
                                return Err(format!("Safety Error: Cannot return reference to local variable: {}", target));
                            }
                        }
                    }
                }
                StmtNode::If(cond, then_branch, else_branch) => {
                    self.check_expr_reads(cond, var_types, owner_states)?;

                    let mut then_owner_states = owner_states.clone();
                    let mut then_borrows = borrows.clone();
                    let mut then_borrow_counts = borrow_counts.clone();

                    self.check_safety_stmts(
                        func,
                        then_branch,
                        var_types,
                        &mut then_owner_states,
                        &mut then_borrows,
                        &mut then_borrow_counts,
                    )?;

                    let mut else_owner_states = owner_states.clone();
                    let mut else_borrows = borrows.clone();
                    let mut else_borrow_counts = borrow_counts.clone();

                    if let Some(eb) = else_branch {
                        self.check_safety_stmts(
                            func,
                            eb,
                            var_types,
                            &mut else_owner_states,
                            &mut else_borrows,
                            &mut else_borrow_counts,
                        )?;
                    }

                    // Merge owner states
                    for (var_name, state) in owner_states.iter_mut() {
                        let then_val = then_owner_states.get(var_name).unwrap_or(&OwnerState::Owned);
                        let else_val = else_owner_states.get(var_name).unwrap_or(&OwnerState::Owned);
                        if *then_val == OwnerState::Moved || *else_val == OwnerState::Moved {
                            *state = OwnerState::Moved;
                        }
                    }

                    // Merge borrows
                    for (k, v) in then_borrows {
                        borrows.insert(k, v);
                    }
                    for (k, v) in else_borrows {
                        borrows.insert(k, v);
                    }

                    // Merge borrow counts
                    for (k, v) in then_borrow_counts {
                        let cur_v = borrow_counts.entry(k).or_insert(0);
                        if v > *cur_v {
                            *cur_v = v;
                        }
                    }
                    for (k, v) in else_borrow_counts {
                        let cur_v = borrow_counts.entry(k).or_insert(0);
                        if v > *cur_v {
                            *cur_v = v;
                        }
                    }
                }
                StmtNode::RepeatWhile(cond, body) => {
                    self.check_expr_reads(cond, var_types, owner_states)?;

                    let start_owner_states = owner_states.clone();

                    self.check_safety_stmts(
                        func,
                        body,
                        var_types,
                        owner_states,
                        borrows,
                        borrow_counts,
                    )?;

                    for (var_name, start_state) in start_owner_states {
                        let end_state = owner_states.get(&var_name).unwrap_or(&OwnerState::Owned);
                        if start_state == OwnerState::Owned && *end_state == OwnerState::Moved {
                            return Err(format!("Safety Error: Variable is moved inside a loop and not reinitialized: {}", var_name));
                        }
                    }

                    self.check_expr_reads(cond, var_types, owner_states)?;
                }
                StmtNode::ForEach(loop_var, iterable, body) => {
                    self.check_expr_reads(iterable, var_types, owner_states)?;

                    // The loop variable is owned within the loop body
                    owner_states.insert(loop_var.clone(), OwnerState::Owned);

                    let start_owner_states = owner_states.clone();

                    self.check_safety_stmts(
                        func,
                        body,
                        var_types,
                        owner_states,
                        borrows,
                        borrow_counts,
                    )?;

                    for (var_name, start_state) in start_owner_states {
                        let end_state = owner_states.get(&var_name).unwrap_or(&OwnerState::Owned);
                        if start_state == OwnerState::Owned && *end_state == OwnerState::Moved {
                            return Err(format!("Safety Error: Variable is moved inside a loop and not reinitialized: {}", var_name));
                        }
                    }
                }
                StmtNode::FieldSet(obj, _, expr) => {
                    self.check_expr_reads(obj, var_types, owner_states)?;
                    self.check_expr_reads(expr, var_types, owner_states)?;
                }
                StmtNode::Match(expr, arms) => {
                    self.check_expr_reads(expr, var_types, owner_states)?;
                    for (_, _, body) in arms {
                        let mut arm_owner_states = owner_states.clone();
                        let mut arm_borrows = borrows.clone();
                        let mut arm_borrow_counts = borrow_counts.clone();
                        self.check_safety_stmts(
                            func,
                            body,
                            var_types,
                            &mut arm_owner_states,
                            &mut arm_borrows,
                            &mut arm_borrow_counts,
                        )?;
                        // Merge: if moved in any arm, mark as moved
                        for (var_name, state) in owner_states.iter_mut() {
                            if let Some(OwnerState::Moved) = arm_owner_states.get(var_name) {
                                *state = OwnerState::Moved;
                            }
                        }
                    }
                }
                StmtNode::Break | StmtNode::Continue => {}
                StmtNode::ExprStmt(expr) => {
                    self.check_expr_reads(expr, var_types, owner_states)?;
                }
            }
        }
        Ok(())
    }

    fn gen_statement(
        &mut self,
        stmt: &Stmt,
        var_types: &HashMap<String, Type>,
    ) -> Result<(), String> {
        match &stmt.node {
            StmtNode::Set(name, expr, _type_ann) => {
                let safe_name = if self.is_async_func {
                    format!("args->{}", Self::sanitize_c_name(name))
                } else {
                    Self::sanitize_c_name(name)
                };
                let _t = var_types.get(name);
                // If this is a closure assignment, pass the variable name to Closure codegen
                if matches!(expr.node, ExprNode::Closure(_, _)) {
                    self.pending_closure_name = Some(name.clone());
                }
                let expr_str = self.gen_expr(expr, var_types)?;

                // For all types: simple assignment.
                // We intentionally do NOT pre-free lists/enums/structs here because
                // the new value may alias the old value through another variable
                // (e.g. set current_level to next_level; set next_level to create_list()
                // would free the list that current_level still references).
                // The GC handles the old allocations.
                self.out.push_str(&format!("    {} = {};\n", safe_name, expr_str));

                // After free_list/free_map calls, null-out the freed variable to prevent
                // double-free when the variable is later reassigned (pre-free pattern)
                if let ExprNode::Call(func_name, args) = &expr.node {
                    if (func_name == "free_list" || func_name == "free_map") && !args.is_empty() {
                        if let ExprNode::Identifier(arg_name) = &args[0].node {
                            let safe_arg = if self.is_async_func {
                                format!("args->{}", Self::sanitize_c_name(arg_name))
                            } else {
                                Self::sanitize_c_name(arg_name)
                            };
                            self.out.push_str(&format!("    {} = 0;\n", safe_arg));
                        }
                    }
                }
            }
            StmtNode::Return(expr) => {
                let expr_str = self.gen_expr(expr, var_types)?;
                if self.is_async_func {
                    self.out.push_str(&format!("    return {};\n", expr_str));
                } else {
                    self.out.push_str(&format!("    ret_val = {};\n", expr_str));

                    if let ExprNode::Identifier(name) = &expr.node {
                        let t = var_types.get(name);
                        if t == Some(&Type::List) || t == Some(&Type::Map) {
                            self.out.push_str(&format!("    {} = 0;\n", name));
                        } else if matches!(t, Some(Type::Enum(_))) {
                            self.out.push_str(&format!("    {} = 0;\n", name));
                        }
                    }
                    self.out.push_str("    goto L_cleanup;\n");
                }
            }
            StmtNode::Display(expr) => {
                let t = self.infer_type(expr, var_types);
                let expr_str = self.gen_expr(expr, var_types)?;
                match t {
                    Type::Str | Type::DynStr | Type::RefStr => {
                        self.out.push_str(&format!("    printf(\"%s\\n\", (char*){});\n", expr_str));
                    }
                    Type::Float => {
                        self.out.push_str(&format!("    {{ long long _ftmp = {}; double _dv; memcpy(&_dv, &_ftmp, sizeof(double)); printf(\"%.15g\\n\", _dv); }}\n", expr_str));
                    }
                    Type::Bool => {
                        self.out.push_str(&format!("    printf(\"%s\\n\", ({}) ? \"true\" : \"false\");\n", expr_str));
                    }
                    Type::Enum(ref enum_name) => {
                        self.out.push_str(&format!("    printf(\"%s\\n\", display_enum_{}({}));\n", enum_name, expr_str));
                    }
                    _ => {
                        // Smart display: detect if value is a string pointer or an integer
                        // ep_auto_to_string returns the pointer unchanged if it's a string,
                        // or allocates a new buffer with the int-to-string conversion
                        self.out.push_str(&format!(
                            "    {{ long long _dv = (long long){0}; long long _sv = ep_auto_to_string(_dv); printf(\"%s\\n\", (char*)_sv); }}\n",
                            expr_str
                        ));
                    }
                }
            }
            StmtNode::If(cond, then_branch, else_branch) => {
                let cond_str = self.gen_expr(cond, var_types)?;
                self.out.push_str(&format!("    if ({}) {{\n", cond_str));
                for s in then_branch {
                    self.gen_statement(s, var_types)?;
                }
                self.out.push_str("    }");
                if let Some(eb) = else_branch {
                    self.out.push_str(" else {\n");
                    for s in eb {
                        self.gen_statement(s, var_types)?;
                    }
                    self.out.push_str("    }\n");
                } else {
                    self.out.push_str("\n");
                }
            }
            StmtNode::RepeatWhile(cond, body) => {
                let cond_str = self.gen_expr(cond, var_types)?;
                self.out.push_str(&format!("    while ({}) {{\n", cond_str));
                for s in body {
                    self.gen_statement(s, var_types)?;
                }
                self.out.push_str("    }\n");
            }
            StmtNode::Spawn(_func_name, args) => {
                let idx = self.spawn_wrapper_index;
                self.spawn_wrapper_index += 1;
                self.out.push_str("    {\n");
                self.out.push_str(&format!("        spawn_args_{}* s_args = malloc(sizeof(spawn_args_{}));\n", idx, idx));
                for (j, arg) in args.iter().enumerate() {
                    let arg_str = self.gen_expr(arg, var_types)?;
                    self.out.push_str(&format!("        s_args->arg{} = {};\n", j, arg_str));
                }
                self.out.push_str("        #ifdef _WIN32\n");
                self.out.push_str(&format!("        HANDLE t = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)spawn_wrapper_{}, s_args, 0, NULL);\n", idx));
                self.out.push_str("        if (t) CloseHandle(t);\n");
                self.out.push_str("        #else\n");
                self.out.push_str("        pthread_t t;\n");
                self.out.push_str(&format!("        int rc = pthread_create(&t, NULL, spawn_wrapper_{}, s_args);\n", idx));
                self.out.push_str("        if (rc != 0) { printf(\"DEBUG: pthread_create failed: %d\\n\", rc); }\n");
                self.out.push_str("        pthread_detach(t);\n");
                self.out.push_str("        #endif\n");
                self.out.push_str("    }\n");
            }
            StmtNode::Send(chan, val) => {
                let chan_str = self.gen_expr(chan, var_types)?;
                let val_str = self.gen_expr(val, var_types)?;
                self.out.push_str(&format!("    send_channel({}, {});\n", chan_str, val_str));
                if let ExprNode::Identifier(name) = &val.node {
                    let vt = var_types.get(name);
                    if vt == Some(&Type::List) || vt == Some(&Type::Map) {
                        self.out.push_str(&format!("    {} = 0;\n", name));
                    }
                }
            }
            StmtNode::FieldSet(obj, field_name, expr) => {
                let obj_str = self.gen_expr(obj, var_types)?;
                let expr_str = self.gen_expr(expr, var_types)?;
                let obj_type = self.infer_type(obj, var_types);
                if let Type::Struct(struct_name) = obj_type {
                    self.out.push_str(&format!("    ((EpStruct_{}*)({}))->{} = {};\n", struct_name, obj_str, field_name, expr_str));
                    self.out.push_str(&format!("    ep_gc_write_barrier((void*)({}), {});\n", obj_str, expr_str));
                } else {
                    // Fail loudly rather than silently dropping the assignment —
                    // emitting nothing here produced a struct whose field was
                    // never written (silent miscompile). Mirrors FieldAccess reads.
                    let var_name = if let ExprNode::Identifier(n) = &obj.node { n.clone() } else { "?".to_string() };
                    return Err(format!("Field assignment '.{}' on non-struct variable '{}' (type: {:?}) at line {}:{}. Add a type annotation like 'with {} as StructName'",
                        field_name, var_name, obj_type, stmt.span.line, stmt.span.col, var_name));
                }
            }
            StmtNode::Match(match_expr, arms) => {
                let expr_str = self.gen_expr(match_expr, var_types)?;
                let enum_type = self.infer_type(match_expr, var_types);
                if let Type::Enum(enum_name) = enum_type {
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        EpEnum_{}* _match_ptr = (EpEnum_{}*){};\n", enum_name, enum_name, expr_str));
                    self.out.push_str("        long long _match_tag = _match_ptr->tag;\n");

                    let ed = self.enum_defs.get(&enum_name).cloned();
                    for (arm_idx, (variant_name, bindings, body)) in arms.iter().enumerate() {
                        let keyword = if arm_idx == 0 { "if" } else { "else if" };
                        self.out.push_str(&format!("        {} (_match_tag == EP_TAG_{}_{}) {{\n", keyword, enum_name, variant_name));

                        if let Some(ref ed) = ed {
                            if let Some((_, fields)) = ed.variants.iter().find(|(vn, _)| vn == variant_name) {
                                for (j, binding) in bindings.iter().enumerate() {
                                    if j < fields.len() {
                                        self.out.push_str(&format!("            long long {} = _match_ptr->data{};\n", binding, j));
                                    }
                                }
                            }
                        }

                        for s in body {
                            self.gen_statement(s, var_types)?;
                        }
                        self.out.push_str("        }\n");
                    }

                    // Exhaustive match check: warn about missing variants
                    if let Some(ref ed) = ed {
                        let matched_variants: Vec<&String> = arms.iter().map(|(vn, _, _)| vn).collect();
                        let missing: Vec<&String> = ed.variants.iter()
                            .map(|(vn, _)| vn)
                            .filter(|vn| !matched_variants.contains(vn))
                            .collect();
                        if !missing.is_empty() {
                            let missing_names: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
                            eprintln!("\x1b[1;33mWarning\x1b[0m: Non-exhaustive check on '{}' — missing variants: {}", 
                                enum_name, missing_names.join(", "));
                        }
                    }

                    self.out.push_str("    }\n");
                } else if matches!(enum_type, Type::Str | Type::DynStr) {
                    // String matching via strcmp
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        const char* _match_val = (const char*){};\n", expr_str));
                    self.out.push_str("        if (_match_val) {\n");
                    for (arm_idx, (pattern, _bindings, body)) in arms.iter().enumerate() {
                        let keyword = if arm_idx == 0 { "if" } else { "else if" };
                        // Escape the pattern string for C
                        let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
                        self.out.push_str(&format!("        {} (strcmp(_match_val, \"{}\") == 0) {{\n", keyword, escaped));
                        for s in body {
                            self.gen_statement(s, var_types)?;
                        }
                        self.out.push_str("        }\n");
                    }
                    self.out.push_str("        }\n"); // close if (_match_val)
                    self.out.push_str("    }\n");
                } else if matches!(enum_type, Type::Int) {
                    // Integer matching via ==
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        long long _match_val = {};\n", expr_str));
                    for (arm_idx, (pattern, _bindings, body)) in arms.iter().enumerate() {
                        let keyword = if arm_idx == 0 { "if" } else { "else if" };
                        self.out.push_str(&format!("        {} (_match_val == {}) {{\n", keyword, pattern));
                        for s in body {
                            self.gen_statement(s, var_types)?;
                        }
                        self.out.push_str("        }\n");
                    }
                    self.out.push_str("    }\n");
                } else {
                    return Err("Match/check statement on non-enum type".to_string());
                }
            }
            StmtNode::ForEach(loop_var, iterable, body) => {
                let iterable_type = self.infer_type(iterable, var_types);
                let mut is_iterator = false;
                let mut iter_type_name = String::new();
                
                match &iterable_type {
                    Type::Struct(name) | Type::Enum(name) => {
                        if self.trait_impls.contains(&("Iterator".to_string(), name.clone())) {
                            is_iterator = true;
                            iter_type_name = name.clone();
                        }
                    }
                    _ => {}
                }
                
                if is_iterator {
                    let iter_expr_str = self.gen_expr(iterable, var_types)?;
                    let next_key = format!("{}_next", iter_type_name);
                    let return_enum_name = match self.func_return_types.get(&next_key) {
                        Some(Type::Enum(ename)) => ename.clone(),
                        _ => "IterResult".to_string(),
                    };
                    
                    let unique_idx = self.spawn_index;
                    self.spawn_index += 1;
                    
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        long long _foreach_iter_{} = {};\n", unique_idx, iter_expr_str));
                    self.out.push_str("        while (1) {\n");
                    self.out.push_str(&format!("            long long _res_{} = {}__next(_foreach_iter_{});\n", unique_idx, iter_type_name, unique_idx));
                    self.out.push_str(&format!("            if (_res_{} == 0) break;\n", unique_idx));
                    self.out.push_str(&format!("            EpEnum_{}* _res_ptr_{} = (EpEnum_{}*)_res_{};\n", return_enum_name, unique_idx, return_enum_name, unique_idx));
                    self.out.push_str(&format!("            if (_res_ptr_{}->tag == EP_TAG_{}_Done) break;\n", unique_idx, return_enum_name));
                    self.out.push_str(&format!("            {} = _res_ptr_{}->data0;\n", loop_var, unique_idx));
                    for s in body {
                        self.gen_statement(s, var_types)?;
                    }
                    self.out.push_str("        }\n");
                    self.out.push_str("    }\n");
                } else if let ExprNode::Call(func_name, args) = &iterable.node {
                    if func_name == "range" {
                        let (start_str, end_str) = if args.len() == 2 {
                            let s = self.gen_expr(&args[0], var_types)?;
                            let e = self.gen_expr(&args[1], var_types)?;
                            (s, e)
                        } else if args.len() == 1 {
                            let e = self.gen_expr(&args[0], var_types)?;
                            ("0".to_string(), e)
                        } else {
                            return Err("range() expects 1 or 2 arguments".to_string());
                        };
                        self.out.push_str(&format!("    for ({} = {}; {} < {}; {}++) {{\n",
                            loop_var, start_str, loop_var, end_str, loop_var));
                        for s in body {
                            self.gen_statement(s, var_types)?;
                        }
                        self.out.push_str("    }\n");
                    } else {
                        let list_str = self.gen_expr(iterable, var_types)?;
                        self.out.push_str("    {\n");
                        self.out.push_str(&format!("        long long _foreach_list = {};\n", list_str));
                        self.out.push_str("        long long _foreach_len = length_list(_foreach_list);\n");
                        self.out.push_str("        for (long long _foreach_i = 0; _foreach_i < _foreach_len; _foreach_i++) {\n");
                        self.out.push_str(&format!("            {} = get_list(_foreach_list, _foreach_i);\n", loop_var));
                        for s in body {
                            self.gen_statement(s, var_types)?;
                        }
                        self.out.push_str("        }\n");
                        self.out.push_str("    }\n");
                    }
                } else {
                    let list_str = self.gen_expr(iterable, var_types)?;
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        long long _foreach_len = length_list({});\n", list_str));
                    self.out.push_str("        for (long long _foreach_i = 0; _foreach_i < _foreach_len; _foreach_i++) {\n");
                    self.out.push_str(&format!("            {} = get_list({}, _foreach_i);\n", loop_var, list_str));
                    for s in body {
                        self.gen_statement(s, var_types)?;
                    }
                    self.out.push_str("        }\n");
                    self.out.push_str("    }\n");
                }
            }
            StmtNode::Break => {
                self.out.push_str("    break;\n");
            }
            StmtNode::Continue => {
                self.out.push_str("    continue;\n");
            }
            StmtNode::ExprStmt(expr) => {
                let expr_str = self.gen_expr(expr, var_types)?;
                self.out.push_str(&format!("    {};\n", expr_str));

                // After free_list/free_map calls, null-out the freed variable to prevent
                // double-free when the variable is later reassigned (pre-free pattern)
                if let ExprNode::Call(func_name, args) = &expr.node {
                    if (func_name == "free_list" || func_name == "free_map") && !args.is_empty() {
                        if let ExprNode::Identifier(arg_name) = &args[0].node {
                            let safe_arg = Self::sanitize_c_name(arg_name);
                            self.out.push_str(&format!("    {} = 0;\n", safe_arg));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn gen_expr(
        &mut self,
        expr: &Expr,
        var_types: &HashMap<String, Type>,
    ) -> Result<String, String> {
        match &expr.node {
            ExprNode::Integer(val) => Ok(format!("{}", val)),
            ExprNode::FloatLiteral(val) => {
                Ok(format!("({{ double _fl = {:.17}; long long _fv; memcpy(&_fv, &_fl, sizeof(double)); _fv; }})", val))
            }
            ExprNode::BoolLiteral(b) => Ok(if *b { "1LL".to_string() } else { "0LL".to_string() }),
            ExprNode::StringLiteral(s) => {
                let escaped = s
                    .replace("\\", "\\\\")
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace("\r", "\\r");
                Ok(format!("(long long)\"{}\"", escaped))
            }
            ExprNode::Identifier(name) => {
                // Check if this is a bare enum variant (no data)
                if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    if let Some(enum_name) = self.variant_to_enum.get(name) {
                        // It's a bare variant — generate enum creation with no args
                        return Ok(format!("create_{}_{}()", enum_name, name));
                    }
                }
                // If this identifier is a known function name (not a variable),
                // cast to (long long) so it can be passed as a HOF argument
                if !var_types.contains_key(name) && self.func_return_types.contains_key(name) {
                    return Ok(format!("(long long){}", Self::sanitize_c_name(name)));
                }
                if self.is_async_func && (var_types.contains_key(name) || self.current_async_func_locals.contains(name)) {
                    return Ok(format!("args->{}", Self::sanitize_c_name(name)));
                }
                Ok(name.clone())
            }
            ExprNode::Binary(left, op, right) => {
                let left_str = self.gen_expr(left, var_types)?;
                let right_str = self.gen_expr(right, var_types)?;
                let lt = self.infer_type(left, var_types);
                let rt = self.infer_type(right, var_types);
                let is_float = lt == Type::Float || rt == Type::Float;
                if is_float {
                    let op_str = match op {
                        Op::Add => "+",
                        Op::Sub => "-",
                        Op::Mul => "*",
                        Op::Div => "/",
                        Op::Mod => return Err("Modulo is not supported on floats".to_string()),
                    };
                    // Unpack both as double, compute, repack as long long
                    let l_unpack = if lt == Type::Float {
                        format!("({{ long long _lt = {}; double _d; memcpy(&_d, &_lt, sizeof(double)); _d; }})", left_str)
                    } else {
                        format!("(double)({})", left_str)
                    };
                    let r_unpack = if rt == Type::Float {
                        format!("({{ long long _rt = {}; double _d; memcpy(&_d, &_rt, sizeof(double)); _d; }})", right_str)
                    } else {
                        format!("(double)({})", right_str)
                    };
                    Ok(format!("({{ double _r = {} {} {}; long long _v; memcpy(&_v, &_r, sizeof(double)); _v; }})", l_unpack, op_str, r_unpack))
                } else {
                    let op_str = match op {
                        Op::Add => "+",
                        Op::Sub => "-",
                        Op::Mul => "*",
                        Op::Div => "/",
                        Op::Mod => "%",
                    };
                    Ok(format!("({} {} {})", left_str, op_str, right_str))
                }
            }
            ExprNode::Comparison(left, op, right) => {
                let left_str = self.gen_expr(left, var_types)?;
                let right_str = self.gen_expr(right, var_types)?;
                
                let left_type = self.infer_type(left, var_types);
                let right_type = self.infer_type(right, var_types);
                let inferred_string = left_type == Type::Str || left_type == Type::DynStr
                    || right_type == Type::Str || right_type == Type::DynStr;
                let has_string_literal = matches!(&left.node, ExprNode::StringLiteral(_))
                    || matches!(&right.node, ExprNode::StringLiteral(_));
                let is_string = inferred_string || has_string_literal;
                // Check if comparing a string to integer 0 (null check pattern from try expressions)
                let right_is_zero = matches!(&right.node, ExprNode::Integer(0));
                let left_is_zero = matches!(&left.node, ExprNode::Integer(0));
                if is_string && (right_is_zero || left_is_zero) {
                    // Null check: use pointer comparison, not strcmp (strcmp(str, NULL) crashes)
                    let op_str = match op {
                        CompOp::Equals => "==",
                        CompOp::NotEquals => "!=",
                        _ => "==",
                    };
                    Ok(format!("({} {} {})", left_str, op_str, right_str))
                } else if is_string {
                    match op {
                        CompOp::Equals => {
                            Ok(format!("ep_str_equals({}, {})", left_str, right_str))
                        }
                        CompOp::NotEquals => {
                            Ok(format!("(!ep_str_equals({}, {}))", left_str, right_str))
                        }
                        _ => {
                            // Ordering comparisons — null-safe: if either is null, return 0 (false)
                            let cmp_op = match op {
                                CompOp::LessThan => "< 0",
                                CompOp::GreaterThan => "> 0",
                                CompOp::LessEqual => "<= 0",
                                CompOp::GreaterEqual => ">= 0",
                                _ => unreachable!(),
                            };
                            Ok(format!("((char*){l} && (char*){r} ? strcmp((char*){l}, (char*){r}) {cmp} : 0)",
                                l=left_str, r=right_str, cmp=cmp_op))
                        }
                    }
                } else {
                    match op {
                        CompOp::Equals => {
                            Ok(format!("{} == {}", left_str, right_str))
                        }
                        CompOp::NotEquals => {
                            Ok(format!("{} != {}", left_str, right_str))
                        }
                        _ => {
                            let op_str = match op {
                                CompOp::LessThan => "<",
                                CompOp::GreaterThan => ">",
                                CompOp::LessEqual => "<=",
                                CompOp::GreaterEqual => ">=",
                                _ => unreachable!(),
                            };
                            Ok(format!("{} {} {}", left_str, op_str, right_str))
                        }
                    }
                }
            }
            ExprNode::Logical(left, op, right) => {
                let left_str = self.gen_expr(left, var_types)?;
                let right_str = self.gen_expr(right, var_types)?;
                let op_str = match op {
                    LogicalOp::And => "&&",
                    LogicalOp::Or => "||",
                };
                Ok(format!("({} {} {})", left_str, op_str, right_str))
            }
            ExprNode::Call(name, args) => {
                let mut args_str = Vec::new();
                for arg in args {
                    args_str.push(self.gen_expr(arg, var_types)?);
                }

                let mut formatted_args = Vec::new();
                for (i, arg_val) in args_str.iter().enumerate() {
                    let casted = match name.as_str() {
                        "read_file_content" | "string_length" | "display_string" | "run_command" | "ep_md5" | "ep_sha256" | "ep_net_connect" if i == 0 => {
                            format!("(char*){}", arg_val)
                        }
                        "get_character" | "substring" | "string_to_list" if i == 0 => {
                            format!("(char*){}", arg_val)
                        }
                        "write_file_content" => {
                            if i == 0 || i == 1 {
                                format!("(char*){}", arg_val)
                            } else {
                                arg_val.clone()
                            }
                        }
                        "ep_net_send" if i == 1 => {
                            format!("(char*){}", arg_val)
                        }
                        _ => arg_val.clone(),
                    };
                    formatted_args.push(casted);
                }

                let safe_name = Self::sanitize_c_name(name);
                let call_str = format!("{}({})", safe_name, formatted_args.join(", "));
                
                // Check if this is a closure call (variable holding a function pointer)
                if let Some(c_func_name) = self.closure_c_names.get(name).cloned() {
                    // Direct call to a known closure — pass env from its EpClosure struct
                    let args_str = formatted_args.join(", ");
                    let env_arg = format!("(long long)((EpClosure*){})->env", safe_name);
                    if args_str.is_empty() {
                        Ok(format!("{}({})", c_func_name, env_arg))
                    } else {
                        Ok(format!("{}({}, {})", c_func_name, env_arg, args_str))
                    }
                } else if var_types.contains_key(name) && !self.func_return_types.contains_key(name) {
                    // Indirect call through a variable — could be an EpClosure or a raw fn ptr
                    // Use magic number dispatch to handle both
                    let n_args = args.len();
                    let mut env_arg_types = vec!["long long".to_string()]; // env param
                    for _ in 0..n_args { env_arg_types.push("long long".to_string()); }
                    let closure_fn_type = format!("long long(*)({})", env_arg_types.join(", "));

                    let mut raw_arg_types: Vec<String> = Vec::new();
                    for _ in 0..n_args { raw_arg_types.push("long long".to_string()); }
                    let raw_fn_type = format!("long long(*)({})", if raw_arg_types.is_empty() { "void".to_string() } else { raw_arg_types.join(", ") });

                    let args_str = formatted_args.join(", ");
                    let closure_call = if args_str.is_empty() {
                        format!("(({})_cl->fn_ptr)((long long)_cl->env)", closure_fn_type)
                    } else {
                        format!("(({})_cl->fn_ptr)((long long)_cl->env, {})", closure_fn_type, args_str)
                    };
                    let raw_call = if args_str.is_empty() {
                        format!("(({}){})()", raw_fn_type, safe_name)
                    } else {
                        format!("(({}){safe_name})({})", raw_fn_type, args_str, safe_name = safe_name)
                    };

                    Ok(format!(
                        "({{ long long _fv = {sv}; EpClosure* _cl = (EpClosure*)_fv; (_fv != 0 && _cl->magic == EP_CLOSURE_MAGIC) ? {cl_call} : {raw_call}; }})",
                        sv = safe_name, cl_call = closure_call, raw_call = raw_call
                    ))
                } else {
                    match name.as_str() {
                        "read_file_content" | "get_argument" | "substring" | "string_from_list" | "ep_net_recv" | "ep_md5" | "ep_sha256" => {
                            Ok(format!("(long long){}", call_str))
                        }
                        _ => Ok(call_str),
                    }
                }
            }
            ExprNode::Channel => Ok("create_channel()".to_string()),
            ExprNode::Receive(chan) => {
                let chan_str = self.gen_expr(chan, var_types)?;
                Ok(format!("receive_channel({})", chan_str))
            }
            ExprNode::Borrow(inner) => self.gen_expr(inner, var_types),
            ExprNode::FieldAccess(obj, field_name) => {
                let obj_str = self.gen_expr(obj, var_types)?;
                let obj_type = self.infer_type(obj, var_types);
                if let Type::Struct(struct_name) = obj_type {
                    // Null pointer protection: check before dereferencing
                    Ok(format!(
                        "({{ long long _fap = {}; if (_fap == 0) {{ fprintf(stderr, \"Error: Null pointer when accessing field '{}' on '{}'\\n\"); exit(1); }} ((EpStruct_{}*)(_fap))->{}; }})",
                        obj_str, field_name, struct_name, struct_name, field_name
                    ))
                } else {
                    let var_name = if let ExprNode::Identifier(n) = &obj.node { n.clone() } else { "?".to_string() };
                    Err(format!("Field access '.{}' on non-struct variable '{}' (type: {:?}) at line {}:{}. Add a type annotation like 'with {} as StructName'",
                        field_name, var_name, obj_type, expr.span.line, expr.span.col, var_name))
                }
            }
            ExprNode::StructCreate(struct_name, fields) => {
                let c_name = format!("EpStruct_{}", struct_name);
                let mut lines = Vec::new();
                lines.push(format!("({{"));
                lines.push(format!("    {}* _s = ({}*)malloc(sizeof({}));", c_name, c_name, c_name));
                
                // Collect explicitly provided field names
                let provided: Vec<&str> = fields.iter().map(|(n, _)| n.as_str()).collect();
                
                // Set explicitly provided fields
                for (fname, fexpr) in fields {
                    let fval = self.gen_expr(fexpr, var_types)?;
                    lines.push(format!("    _s->{} = {};", fname, fval));
                }
                
                // Fill in missing fields with defaults from struct definition
                if let Some(sd) = self.struct_defs.get(struct_name).cloned() {
                    for (fname, ftype, default_expr) in &sd.fields {
                        if !provided.contains(&fname.as_str()) {
                            if let Some(def_expr) = default_expr {
                                let def_val = self.gen_expr(def_expr, var_types)?;
                                lines.push(format!("    _s->{} = {};", fname, def_val));
                            } else {
                                // Use type-appropriate zero value
                                let zero = match ftype {
                                    TypeAnnotation::Str | TypeAnnotation::DynStr => "(long long)\"\"".to_string(),
                                    TypeAnnotation::List => "create_list()".to_string(),
                                    _ => "0".to_string(),
                                };
                                lines.push(format!("    _s->{} = {};", fname, zero));
                            }
                        }
                    }
                }
                
                let total_fields = if let Some(sd) = self.struct_defs.get(struct_name) {
                    sd.fields.len()
                } else {
                    fields.len()
                };
                lines.push(format!("    {{ EpGCObject* _go = ep_gc_register(_s, EP_OBJ_STRUCT); if(_go) _go->num_fields = {}; }}", total_fields));
                lines.push(format!("    (long long)_s;"));
                lines.push(format!("}})")); 
                Ok(lines.join("\n"))
            }
            ExprNode::EnumCreate(enum_name, variant_name, args) => {
                let resolved_enum = if enum_name.is_empty() {
                    self.variant_to_enum.get(variant_name).cloned().unwrap_or_default()
                } else {
                    enum_name.clone()
                };
                let mut args_str = Vec::new();
                for arg in args {
                    args_str.push(self.gen_expr(arg, var_types)?);
                }
                Ok(format!("create_{}_{}({})", resolved_enum, variant_name, args_str.join(", ")))
            }
            ExprNode::MethodCall(obj, method_name, args) => {
                let obj_str = self.gen_expr(obj, var_types)?;
                let obj_type = self.infer_type(obj, var_types);
                let struct_name = match &obj_type {
                    Type::Struct(s) => s.clone(),
                    Type::Enum(e) => e.clone(),
                    _ => {
                        let var_name = if let ExprNode::Identifier(n) = &obj.node { n.clone() } else { "?".to_string() };
                        return Err(format!("Method '.{}()' called on non-struct/enum variable '{}' (type: {:?}) at line {}:{}",
                            method_name, var_name, obj_type, expr.span.line, expr.span.col));
                    }
                };
                let mut all_args = vec![obj_str];
                for arg in args {
                    all_args.push(self.gen_expr(arg, var_types)?);
                }
                Ok(format!("{}__{}({})", struct_name, method_name, all_args.join(", ")))
            }
            ExprNode::UnaryNot(inner) => {
                let inner_str = self.gen_expr(inner, var_types)?;
                Ok(format!("(!({}))", inner_str))
            }
            ExprNode::TryExpr(inner) => {
                let inner_str = self.gen_expr(inner, var_types)?;
                let try_id = self.spawn_index;
                self.spawn_index += 1;
                let inner_type = self.infer_type(inner, var_types);
                
                // If the inner expression returns an Enum (Result-style), do proper unwrapping
                if let Type::Enum(ref enum_name) = inner_type {
                    if let Some(ed) = self.enum_defs.get(enum_name).cloned() {
                        // Convention: first variant is the "success" variant (Ok),
                        // second variant is the "error" variant (Error) that gets propagated
                        if ed.variants.len() >= 2 {
                            return Ok(format!(
                                "({{ \
                                    long long _try_enum_{id} = {inner}; \
                                    EpEnum_{ename}* _try_ptr_{id} = (EpEnum_{ename}*)_try_enum_{id}; \
                                    if (_try_ptr_{id}->tag != EP_TAG_{ename}_{ok_variant}) {{ \
                                        ret_val = _try_enum_{id}; \
                                        goto L_cleanup; \
                                    }} \
                                    _try_ptr_{id}->data0; \
                                }})",
                                id = try_id,
                                inner = inner_str,
                                ename = enum_name,
                                ok_variant = ed.variants[0].0,
                            ));
                        }
                    }
                }
                
                // Fallback: non-enum try — use setjmp/longjmp crash guard
                let zero_val = match inner_type {
                    Type::Str | Type::DynStr | Type::RefStr => "(long long)\"\"",
                    _ => "0",
                };
                Ok(format!(
                    "({{ volatile long long _try_r_{id} = {zero}; \
                    if (setjmp(ep_try_buf) == 0) {{ \
                        ep_try_active = 1; \
                        _try_r_{id} = {inner}; \
                        ep_try_active = 0; \
                    }} else {{ \
                        _try_r_{id} = {zero}; \
                    }} \
                    _try_r_{id}; }})",
                    id = try_id,
                    inner = inner_str,
                    zero = zero_val
                ))
            }
            ExprNode::Await(inner) => {
                if self.is_async_func {
                    self.await_counter += 1;
                    Ok(format!("(args->awaited_fut_{} ? args->awaited_fut_{}->value : 0)", self.await_counter, self.await_counter))
                } else {
                    let inner_str = self.gen_expr(inner, var_types)?;
                    Ok(format!("ep_await_future((EpFuture*){})", inner_str))
                }
            }
            ExprNode::Closure(params, body) => {
                // Generate a static closure function
                // Use pre-registered name if this closure is being assigned to a named variable
                let closure_name = if let Some(var_name) = self.pending_closure_name.take() {
                    self.closure_c_names.get(&var_name)
                        .cloned()
                        .unwrap_or_else(|| {
                            let name = format!("_ep_closure_{}", self.spawn_index);
                            self.spawn_index += 1;
                            name
                        })
                } else {
                    let name = format!("_ep_closure_{}", self.spawn_index);
                    self.spawn_index += 1;
                    name
                };

                // Detect captured variables: identifiers in body that exist in outer
                // var_types but are NOT closure parameters and NOT known functions
                let param_set: std::collections::HashSet<&String> = params.iter().collect();
                let mut captured: Vec<String> = Vec::new();
                let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
                Self::collect_identifiers_in_stmts(body, &mut |ident| {
                    if !param_set.contains(ident)
                        && !seen.contains(ident)
                        && var_types.contains_key(ident)
                        && !self.func_return_types.contains_key(ident)
                        && ident != "ret_val"
                    {
                        captured.push(ident.clone());
                        seen.insert(ident.clone());
                    }
                });

                // Build parameter list: _ep_env (first), then explicit params
                let mut c_params: Vec<String> = vec!["long long _ep_env".to_string()];
                for p in params.iter() {
                    c_params.push(format!("long long {}", p));
                }
                let c_param_str = c_params.join(", ");

                let n_captures = captured.len();

                // Store captures for this closure so call sites can pass them
                self.closure_captures.insert(closure_name.clone(), captured.clone());

                // Generate the closure body using buffer swapping
                // Start with a COPY of outer scope's var_types so closures can reference
                // outer variables (including other closures)
                let mut closure_var_types = var_types.clone();
                for p in params {
                    closure_var_types.insert(p.clone(), Type::Int);
                }

                // Generate closure body into a fresh buffer
                let saved_out = std::mem::take(&mut self.out);

                self.out.push_str(&format!("long long {}({}) {{\n", closure_name, c_param_str));
                self.out.push_str("    long long ret_val = 0;\n");

                // Unpack captured variables from _ep_env array
                for (i, cap) in captured.iter().enumerate() {
                    let safe_cap = Self::sanitize_c_name(cap);
                    self.out.push_str(&format!("    long long {} = ((long long*)_ep_env)[{}];\n", safe_cap, i));
                }

                // Declare local variables in the closure body
                // (variables defined by set/for-each that aren't params or captures)
                let mut body_var_types = HashMap::new();
                self.collect_var_types(body, &mut body_var_types);
                for (var_name, _) in &body_var_types {
                    let safe = Self::sanitize_c_name(var_name);
                    let is_param = params.iter().any(|p| p == var_name);
                    let is_capture = captured.iter().any(|c| c == var_name);
                    if !is_param && !is_capture && var_name != "ret_val" {
                        self.out.push_str(&format!("    long long {} = 0;\n", safe));
                    }
                }

                for stmt in body {
                    self.gen_statement(stmt, &closure_var_types)?;
                }
                self.out.push_str("L_cleanup:\n");
                self.out.push_str("    return ret_val;\n");
                self.out.push_str("}\n\n");

                // Insert closure function at the bodies marker (after runtime defs and forward decls)
                let closure_code = std::mem::take(&mut self.out);
                let bodies_marker = "/* EP_CLOSURE_BODIES */\n";
                if let Some(marker_pos) = saved_out.find(bodies_marker) {
                    let insert_pos = marker_pos + bodies_marker.len();
                    let (before, after) = saved_out.split_at(insert_pos);
                    self.out = format!("{}{}{}", before, closure_code, after);
                } else {
                    // Fallback: append closure before current content
                    self.out = closure_code + &saved_out;
                }

                // Return an EpClosure struct allocation with captures packed in env[]
                let mut create_parts = Vec::new();
                create_parts.push(format!(
                    "({{ EpClosure* _cl_{0} = (EpClosure*)malloc(sizeof(EpClosure) + {1} * sizeof(long long)); _cl_{0}->magic = EP_CLOSURE_MAGIC; _cl_{0}->fn_ptr = (long long){2};",
                    self.spawn_index, n_captures, closure_name
                ));
                for (i, cap) in captured.iter().enumerate() {
                    create_parts.push(format!(" _cl_{}->env[{}] = {};", self.spawn_index, i, Self::sanitize_c_name(cap)));
                }
                create_parts.push(format!(" (long long)_cl_{}; }})", self.spawn_index));
                self.spawn_index += 1;
                Ok(create_parts.join(""))
            }
            ExprNode::ListLiteral(elements) => {
                // Generate: ({ long long _lit = create_list(); append_list(_lit, e1); ... _lit; })
                let mut parts = Vec::new();
                parts.push("({ long long _lit_list = create_list();".to_string());
                for elem in elements {
                    let elem_str = self.gen_expr(elem, var_types)?;
                    let elem_type = self.infer_type(elem, var_types);
                    let cast = match elem_type {
                        Type::Str | Type::DynStr => format!("(long long){}", elem_str),
                        _ => elem_str,
                    };
                    parts.push(format!(" append_list(_lit_list, {});", cast));
                }
                parts.push(" _lit_list; })".to_string());
                Ok(parts.join(""))
            }
        }
    }
}

// ========== Closure capture helpers ==========

impl Codegen {
    /// Walk statements and call `callback` with each identifier found
    fn collect_identifiers_in_stmts(stmts: &[Stmt], callback: &mut dyn FnMut(&String)) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(_, expr, _) | StmtNode::ExprStmt(expr) | StmtNode::Return(expr) | StmtNode::Display(expr) => {
                    Self::collect_identifiers_in_expr(expr, callback);
                }
                StmtNode::FieldSet(obj, _, val) => {
                    Self::collect_identifiers_in_expr(obj, callback);
                    Self::collect_identifiers_in_expr(val, callback);
                }
                StmtNode::If(cond, then_b, else_b) => {
                    Self::collect_identifiers_in_expr(cond, callback);
                    Self::collect_identifiers_in_stmts(then_b, callback);
                    if let Some(eb) = else_b {
                        Self::collect_identifiers_in_stmts(eb, callback);
                    }
                }
                StmtNode::RepeatWhile(cond, body) => {
                    Self::collect_identifiers_in_expr(cond, callback);
                    Self::collect_identifiers_in_stmts(body, callback);
                }
                StmtNode::ForEach(_, iter, body) => {
                    Self::collect_identifiers_in_expr(iter, callback);
                    Self::collect_identifiers_in_stmts(body, callback);
                }
                StmtNode::Match(expr, arms) => {
                    Self::collect_identifiers_in_expr(expr, callback);
                    for (_, _, body) in arms {
                        Self::collect_identifiers_in_stmts(body, callback);
                    }
                }
                StmtNode::Spawn(_, args) => {
                    for a in args { Self::collect_identifiers_in_expr(a, callback); }
                }
                StmtNode::Send(a, b) => {
                    Self::collect_identifiers_in_expr(a, callback);
                    Self::collect_identifiers_in_expr(b, callback);
                }
                StmtNode::Break | StmtNode::Continue => {}
            }
        }
    }

    /// Walk an expression and call `callback` with each identifier found
    fn collect_identifiers_in_expr(expr: &Expr, callback: &mut dyn FnMut(&String)) {
        match &expr.node {
            ExprNode::Identifier(name) => callback(name),
            ExprNode::Binary(l, _, r) | ExprNode::Comparison(l, _, r) | ExprNode::Logical(l, _, r) => {
                Self::collect_identifiers_in_expr(l, callback);
                Self::collect_identifiers_in_expr(r, callback);
            }
            ExprNode::Call(_, args) | ExprNode::EnumCreate(_, _, args) => {
                for a in args { Self::collect_identifiers_in_expr(a, callback); }
            }
            ExprNode::UnaryNot(inner) | ExprNode::TryExpr(inner) | ExprNode::Await(inner)
            | ExprNode::Borrow(inner) | ExprNode::Receive(inner) => {
                Self::collect_identifiers_in_expr(inner, callback);
            }
            ExprNode::FieldAccess(obj, _) => {
                Self::collect_identifiers_in_expr(obj, callback);
            }
            ExprNode::MethodCall(obj, _, args) => {
                Self::collect_identifiers_in_expr(obj, callback);
                for a in args { Self::collect_identifiers_in_expr(a, callback); }
            }
            ExprNode::StructCreate(_, field_inits) => {
                for (_, fexpr) in field_inits { Self::collect_identifiers_in_expr(fexpr, callback); }
            }
            ExprNode::ListLiteral(elems) => {
                for e in elems { Self::collect_identifiers_in_expr(e, callback); }
            }
            ExprNode::Closure(_, body) => {
                Self::collect_identifiers_in_stmts(body, callback);
            }
            _ => {} // Integer, Float, Bool, String literals, Channel
        }
    }
}

// ========== analyze_safety, generate, gen_function ==========

impl Codegen {
    fn analyze_safety(&mut self, program: &Program) -> Result<(), String> {
        for func in &program.functions {
            let mut var_types = HashMap::new();
            for param in &func.params {
                let param_type = if let Some(ref ann) = param.2 {
                    self.type_annotation_to_type(ann)
                } else if param.1 {
                    Type::RefList
                } else {
                    Type::Int
                };
                var_types.insert(param.0.clone(), param_type);
            }
            self.collect_var_types(&func.body, &mut var_types);

            let mut owner_states = HashMap::new();
            for param in &func.params {
                let t = var_types.get(&param.0).cloned().unwrap_or(Type::Int);
                if is_tracked(&t) {
                    owner_states.insert(param.0.clone(), OwnerState::Owned);
                }
            }

            let mut borrows = HashMap::new();
            let mut borrow_counts = HashMap::new();

            self.check_safety_stmts(
                func,
                &func.body,
                &var_types,
                &mut owner_states,
                &mut borrows,
                &mut borrow_counts,
            )?;
        }
        Ok(())
    }

    fn collect_spawns_in_stmts(&self, stmts: &[Stmt], spawn_list: &mut Vec<Stmt>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Spawn(_, _) => spawn_list.push(stmt.clone()),
                StmtNode::If(_, then_branch, else_branch) => {
                    self.collect_spawns_in_stmts(then_branch, spawn_list);
                    if let Some(eb) = else_branch {
                        self.collect_spawns_in_stmts(eb, spawn_list);
                    }
                }
                StmtNode::RepeatWhile(_, body) => {
                    self.collect_spawns_in_stmts(body, spawn_list);
                }
                StmtNode::ForEach(_, _, body) => {
                    self.collect_spawns_in_stmts(body, spawn_list);
                }
                StmtNode::Match(_, arms) => {
                    for (_, _, body) in arms {
                        self.collect_spawns_in_stmts(body, spawn_list);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_all_spawns(&self, program: &Program) -> Vec<Stmt> {
        let mut spawn_list = Vec::new();
        for func in &program.functions {
            self.collect_spawns_in_stmts(&func.body, &mut spawn_list);
        }
        spawn_list
    }

    fn get_c_test_main_source(&self, program: &Program) -> String {
        let mut test_cases = Vec::new();
        for func in &program.functions {
            if func.name.starts_with("test_") {
                test_cases.push(func.name.clone());
            }
        }
        
        let test_count = test_cases.len();
        let mut lines = Vec::new();
        lines.push("\n/* Test runner C main */\n".to_string());
        lines.push("#ifndef __wasm__\n".to_string());
        lines.push("#include <sys/types.h>\n".to_string());
        lines.push("#include <sys/wait.h>\n".to_string());
        lines.push("#include <unistd.h>\n".to_string());
        lines.push("#endif\n".to_string());
        lines.push("#include <stdio.h>\n".to_string());
        lines.push("#include <stdlib.h>\n".to_string());
        lines.push("#include <stdint.h>\n\n".to_string());
        
        lines.push("#ifdef __wasm__\n".to_string());
        lines.push("int run_test(long long (*test_func)(void), const char* name) {\n".to_string());
        lines.push("    printf(\"test_%s ... \", name);\n".to_string());
        lines.push("    fflush(stdout);\n".to_string());
        lines.push("    long long res = test_func();\n".to_string());
        lines.push("    if (res == 0) {\n".to_string());
        lines.push("        printf(\"OK\\n\");\n".to_string());
        lines.push("        return 1;\n".to_string());
        lines.push("    } else {\n".to_string());
        lines.push("        printf(\"FAILED\\n\");\n".to_string());
        lines.push("        return 0;\n".to_string());
        lines.push("    }\n".to_string());
        lines.push("}\n".to_string());
        lines.push("#else\n".to_string());
        lines.push("int run_test(long long (*test_func)(void), const char* name) {\n".to_string());
        lines.push("    printf(\"test_%s ... \", name);\n".to_string());
        lines.push("    fflush(stdout);\n".to_string());
        lines.push("    pid_t pid = fork();\n".to_string());
        lines.push("    if (pid < 0) {\n".to_string());
        lines.push("        printf(\"FAILED (fork failed)\\n\");\n".to_string());
        lines.push("        return 0;\n".to_string());
        lines.push("    }\n".to_string());
        lines.push("    if (pid == 0) {\n".to_string());
        lines.push("        exit((int)test_func());\n".to_string());
        lines.push("    } else {\n".to_string());
        lines.push("        int status;\n".to_string());
        lines.push("        waitpid(pid, &status, 0);\n".to_string());
        lines.push("        if (WIFEXITED(status)) {\n".to_string());
        lines.push("            int exit_code = WEXITSTATUS(status);\n".to_string());
        lines.push("            if (exit_code == 0) {\n".to_string());
        lines.push("                printf(\"OK\\n\");\n".to_string());
        lines.push("                return 1;\n".to_string());
        lines.push("            } else {\n".to_string());
        lines.push("                printf(\"FAILED (exit code %d)\\n\", exit_code);\n".to_string());
        lines.push("                return 0;\n".to_string());
        lines.push("            }\n".to_string());
        lines.push("        } else if (WIFSIGNALED(status)) {\n".to_string());
        lines.push("            int sig = WTERMSIG(status);\n".to_string());
        lines.push("            printf(\"FAILED (crashed/signal %d)\\n\", sig);\n".to_string());
        lines.push("            return 0;\n".to_string());
        lines.push("        } else {\n".to_string());
        lines.push("            printf(\"FAILED\\n\");\n".to_string());
        lines.push("            return 0;\n".to_string());
        lines.push("        }\n".to_string());
        lines.push("    }\n".to_string());
        lines.push("}\n".to_string());
        lines.push("#endif\n\n".to_string());
        
        lines.push("int main(int argc, char** argv) {\n".to_string());
        lines.push("    init_ep_args(argc, argv);\n".to_string());
        lines.push(format!("    printf(\"Running {} tests...\\n\");\n", test_count));
        lines.push("    int passed = 0;\n".to_string());
        lines.push("    int failed = 0;\n".to_string());
        lines.push("    int total = 0;\n\n".to_string());
        
        for name in &test_cases {
            lines.push("    total++;\n".to_string());
            lines.push(format!("    if (run_test((long long (*)(void)){}, \"{}\")) passed++; else failed++;\n", name, name));
        }
        
        lines.push("\n    printf(\"\\nResult: %d passed; %d failed\\n\", passed, failed);\n".to_string());
        lines.push("    if (failed > 0) return 1;\n".to_string());
        lines.push("    return 0;\n".to_string());
        lines.push("}\n".to_string());
        
        lines.concat()
    }

    /// Emit a standalone C file containing ONLY the runtime and program-specific
    /// struct/enum definitions. No user function code, no main wrapper.
    /// Used by the native backend: the runtime compiles to a .o that links
    /// with the native assembly .o to produce a complete binary.
    pub fn emit_runtime_c(&mut self, program: &Program) -> String {
        let mut out = String::new();

        // Register struct/enum definitions (needed for struct/enum codegen)
        for sd in &program.struct_defs {
            self.struct_defs.insert(sd.name.clone(), sd.clone());
        }
        for ed in &program.enum_defs {
            self.enum_defs.insert(ed.name.clone(), ed.clone());
            for (variant_name, _) in &ed.variants {
                self.variant_to_enum.insert(variant_name.clone(), ed.name.clone());
            }
        }

        // 1. The full C runtime header + implementation
        out.push_str(RUNTIME_HEADER_AND_SRC);

        // 2. User-defined struct typedefs and free functions
        if !program.struct_defs.is_empty() {
            out.push_str("\n/* User-Defined Structures */\n");
            for sd in &program.struct_defs {
                out.push_str(&format!("typedef struct {{\n"));
                for (fname, _ftype, _) in &sd.fields {
                    out.push_str(&format!("    long long {};\n", fname));
                }
                out.push_str(&format!("}} EpStruct_{};\n\n", sd.name));

                out.push_str(&format!("void free_struct_{}(long long ptr) {{\n", sd.name));
                out.push_str("    if (ptr == 0) return;\n");
                out.push_str("    if (!ep_gc_find((void*)ptr)) return;\n");
                out.push_str(&format!("    EpStruct_{}* s = (EpStruct_{}*)ptr;\n", sd.name, sd.name));
                for (fname, ftype, _) in &sd.fields {
                    match ftype {
                        TypeAnnotation::List => {
                            out.push_str(&format!("    free_list(s->{});\n", fname));
                        }
                        TypeAnnotation::DynStr => {
                            out.push_str(&format!("    if (s->{}) free((void*)s->{});\n", fname, fname));
                        }
                        TypeAnnotation::UserDefined(inner_name) => {
                            out.push_str(&format!("    free_struct_{}(s->{});\n", inner_name, fname));
                        }
                        TypeAnnotation::Generic(inner_name, _) => {
                            out.push_str(&format!("    free_struct_{}(s->{});\n", inner_name, fname));
                        }
                        _ => {}
                    }
                }
                out.push_str("    ep_gc_unregister(s);\n");
                out.push_str("    free(s);\n");
                out.push_str("}\n\n");
            }
        }

        // 3. User-defined enum typedefs, constructors, and free functions
        if !program.enum_defs.is_empty() {
            out.push_str("\n/* User-Defined Choices (Enums) */\n");
            for ed in &program.enum_defs {
                for (i, (vname, _)) in ed.variants.iter().enumerate() {
                    out.push_str(&format!("#define EP_TAG_{}_{} {}\n", ed.name, vname, i));
                }
                out.push_str("\n");

                let max_fields = ed.variants.iter().map(|(_, fields)| fields.len()).max().unwrap_or(0);
                out.push_str("typedef struct {\n");
                out.push_str("    long long tag;\n");
                for j in 0..max_fields {
                    out.push_str(&format!("    long long data{};\n", j));
                }
                out.push_str(&format!("}} EpEnum_{};\n\n", ed.name));

                out.push_str(&format!("void free_enum_{}(long long ptr) {{\n", ed.name));
                out.push_str("    if (ptr == 0) return;\n");
                out.push_str("    if (!ep_gc_find((void*)ptr)) return;\n");
                out.push_str("    ep_gc_unregister((void*)ptr);\n");
                out.push_str("    free((void*)ptr);\n");
                out.push_str("}\n\n");

                for (_i, (vname, fields)) in ed.variants.iter().enumerate() {
                    let mut params = Vec::new();
                    for (j, _) in fields.iter().enumerate() {
                        params.push(format!("long long arg{}", j));
                    }
                    let params_str = if params.is_empty() { "void".to_string() } else { params.join(", ") };
                    out.push_str(&format!("long long create_{}_{}({}) {{\n", ed.name, vname, params_str));
                    out.push_str(&format!("    EpEnum_{}* e = (EpEnum_{}*)malloc(sizeof(EpEnum_{}));\n", ed.name, ed.name, ed.name));
                    out.push_str(&format!("    e->tag = EP_TAG_{}_{};\n", ed.name, vname));
                    for (j, _) in fields.iter().enumerate() {
                        out.push_str(&format!("    e->data{} = arg{};\n", j, j));
                    }
                    out.push_str("    ep_gc_register(e, EP_OBJ_CLOSURE);\n");
                    out.push_str("    return (long long)e;\n");
                    out.push_str("}\n\n");
                }

                out.push_str(&format!("const char* display_enum_{}(long long ptr) {{\n", ed.name));
                out.push_str("    if (ptr == 0) return \"(null)\";\n");
                out.push_str(&format!("    EpEnum_{}* e = (EpEnum_{}*)ptr;\n", ed.name, ed.name));
                for (i, (vname, _)) in ed.variants.iter().enumerate() {
                    out.push_str(&format!("    if (e->tag == {}) return \"{}\";\n", i, vname));
                }
                out.push_str("    return \"(unknown)\";\n");
                out.push_str("}\n\n");
            }
        }

        // 4. Builtin helper functions that are emitted inline by generate()
        out.push_str("\n/* Built-in: string concatenation */\n");
        out.push_str("long long concat(long long a, long long b) {\n");
        out.push_str("    const char* sa = (const char*)a;\n");
        out.push_str("    const char* sb = (const char*)b;\n");
        out.push_str("    long long la = strlen(sa);\n");
        out.push_str("    long long lb = strlen(sb);\n");
        out.push_str("    char* result = malloc(la + lb + 1);\n");
        out.push_str("    memcpy(result, sa, la);\n");
        out.push_str("    memcpy(result + la, sb, lb);\n");
        out.push_str("    result[la + lb] = '\\0';\n");
        out.push_str("    ep_gc_register(result, EP_OBJ_STRING);\n");
        out.push_str("    return (long long)result;\n");
        out.push_str("}\n\n");

        out.push_str("long long int_to_string(long long val) {\n");
        out.push_str("    char* buf = malloc(32);\n");
        out.push_str("    snprintf(buf, 32, \"%lld\", val);\n");
        out.push_str("    ep_gc_register(buf, EP_OBJ_STRING);\n");
        out.push_str("    return (long long)buf;\n");
        out.push_str("}\n\n");

        // ep_int_to_str alias
        out.push_str("long long ep_int_to_str(long long val) { return int_to_string(val); }\n\n");

        // FFI pointer/byte builtins
        out.push_str("long long str_to_ptr(long long s) { return s; }\n");
        out.push_str("long long ptr_to_str(long long p) {\n");
        out.push_str("    if (p == 0) return (long long)strdup(\"\");\n");
        out.push_str("    char* copy = strdup((const char*)p);\n");
        out.push_str("    ep_gc_register(copy, EP_OBJ_STRING);\n");
        out.push_str("    return (long long)copy;\n");
        out.push_str("}\n\n");
        out.push_str("long long peek_byte(long long ptr, long long offset) {\n");
        out.push_str("    return (long long)((unsigned char*)ptr)[offset];\n");
        out.push_str("}\n");
        out.push_str("long long poke_byte(long long ptr, long long offset, long long value) {\n");
        out.push_str("    ((unsigned char*)ptr)[offset] = (unsigned char)value;\n");
        out.push_str("    return 0;\n");
        out.push_str("}\n");
        out.push_str("long long alloc_bytes(long long size) {\n");
        out.push_str("    return (long long)calloc((size_t)size, 1);\n");
        out.push_str("}\n");
        out.push_str("long long free_bytes(long long ptr) {\n");
        out.push_str("    free((void*)ptr);\n");
        out.push_str("    return 0;\n");
        out.push_str("}\n");
        out.push_str("long long list_to_bytes(long long list_ptr) {\n");
        out.push_str("    long long len = length_list(list_ptr);\n");
        out.push_str("    unsigned char* buf = (unsigned char*)malloc(len);\n");
        out.push_str("    for (long long i = 0; i < len; i++) {\n");
        out.push_str("        buf[i] = (unsigned char)get_list(list_ptr, i);\n");
        out.push_str("    }\n");
        out.push_str("    return (long long)buf;\n");
        out.push_str("}\n");
        out.push_str("long long bytes_to_list(long long ptr, long long len) {\n");
        out.push_str("    long long list = create_list();\n");
        out.push_str("    unsigned char* buf = (unsigned char*)ptr;\n");
        out.push_str("    for (long long i = 0; i < len; i++) {\n");
        out.push_str("        append_list(list, (long long)buf[i]);\n");
        out.push_str("    }\n");
        out.push_str("    return list;\n");
        out.push_str("}\n\n");
        out.push_str("long long ep_gc_get_minor_count() {\n");
        out.push_str("    return ep_gc_minor_count;\n");
        out.push_str("}\n");
        out.push_str("long long ep_gc_get_major_count() {\n");
        out.push_str("    return ep_gc_major_count;\n");
        out.push_str("}\n");
        out.push_str("long long ep_gc_get_nursery_count() {\n");
        out.push_str("    return ep_gc_nursery_count;\n");
        out.push_str("}\n\n");

        out.push_str("long long string_to_int(long long s) {\n");
        out.push_str("    if (s == 0) return 0;\n");
        out.push_str("    return atoll((const char*)s);\n");
        out.push_str("}\n\n");

        out.push_str("long long read_line() {\n");
        out.push_str("    char buf[4096];\n");
        out.push_str("    if (fgets(buf, sizeof(buf), stdin) == NULL) { buf[0] = '\\0'; }\n");
        out.push_str("    size_t len = strlen(buf);\n");
        out.push_str("    if (len > 0 && buf[len-1] == '\\n') buf[len-1] = '\\0';\n");
        out.push_str("    char* result = strdup(buf);\n");
        out.push_str("    ep_gc_register(result, EP_OBJ_STRING);\n");
        out.push_str("    return (long long)result;\n");
        out.push_str("}\n\n");

        out.push_str("long long read_int() {\n");
        out.push_str("    long long val = 0;\n");
        out.push_str("    scanf(\"%lld\", &val);\n");
        out.push_str("    while(getchar() != '\\n');\n");
        out.push_str("    return val;\n");
        out.push_str("}\n\n");

        out
    }

    pub fn generate(&mut self, program: &Program) -> Result<String, String> {
        self.out.clear();

        // Register struct definitions
        for sd in &program.struct_defs {
            self.struct_defs.insert(sd.name.clone(), sd.clone());
        }

        // Register enum definitions and build variant lookup
        for ed in &program.enum_defs {
            self.enum_defs.insert(ed.name.clone(), ed.clone());
            for (variant_name, _) in &ed.variants {
                self.variant_to_enum.insert(variant_name.clone(), ed.name.clone());
            }
        }

        // Register trait implementations
        for ti in &program.trait_impls {
            self.trait_impls.insert((ti.trait_name.clone(), ti.for_type.clone()));
        }

        self.analyze_return_types(program);
        self.analyze_safety(program)?;

        // Register top-level constant names as globals
        for stmt in &program.top_level_constants {
            if let StmtNode::Set(name, _, _) = &stmt.node {
                self.global_constants.insert(name.clone());
            }
        }

        // Register method return types
        for md in &program.method_defs {
            let key = format!("{}_{}", md.struct_name, md.name);
            if let Some(ref rt) = md.return_type {
                self.func_return_types.insert(key, self.type_annotation_to_type(rt));
            }
        }
        for ti in &program.trait_impls {
            for m in &ti.methods {
                let key = format!("{}_{}", ti.for_type, m.name);
                if let Some(ref rt) = m.return_type {
                    self.func_return_types.insert(key, self.type_annotation_to_type(rt));
                }
            }
        }

        // Write C Runtime
        self.out.push_str(RUNTIME_HEADER_AND_SRC);

        // Emit C struct typedefs
        if !program.struct_defs.is_empty() {
            self.out.push_str("\n/* User-Defined Structures */\n");
            for sd in &program.struct_defs {
                self.out.push_str(&format!("typedef struct {{\n"));
                for (fname, ftype, _) in &sd.fields {
                    let c_type = match ftype {
                        TypeAnnotation::Int => "long long",
                        TypeAnnotation::Float => "long long",
                        TypeAnnotation::Bool => "long long",
                        TypeAnnotation::Str => "long long",
                        TypeAnnotation::DynStr => "long long",
                        TypeAnnotation::List => "long long",
                        TypeAnnotation::UserDefined(_) => "long long",
                        TypeAnnotation::Generic(_, _) => "long long",
                    };
                    self.out.push_str(&format!("    {} {};\n", c_type, fname));
                }
                self.out.push_str(&format!("}} EpStruct_{};\n\n", sd.name));

                self.out.push_str(&format!("void free_struct_{}(long long ptr) {{\n", sd.name));
                self.out.push_str(&format!("    if (ptr == 0) return;\n"));
                self.out.push_str("    /* Skip if already freed (idempotent — prevents double-free with shared refs) */\n");
                self.out.push_str("    if (!ep_gc_find((void*)ptr)) return;\n");
                self.out.push_str(&format!("    EpStruct_{}* s = (EpStruct_{}*)ptr;\n", sd.name, sd.name));
                for (fname, ftype, _) in &sd.fields {
                    match ftype {
                        TypeAnnotation::List => {
                            self.out.push_str(&format!("    free_list(s->{});\n", fname));
                        }
                        TypeAnnotation::DynStr => {
                            self.out.push_str(&format!("    if (s->{}) free((void*)s->{});\n", fname, fname));
                        }
                        TypeAnnotation::UserDefined(inner_name) => {
                            if inner_name != "Any" {
                                if self.enum_defs.contains_key(inner_name) {
                                    self.out.push_str(&format!("    free_enum_{}(s->{});\n", inner_name, fname));
                                } else if self.struct_defs.contains_key(inner_name) {
                                    self.out.push_str(&format!("    free_struct_{}(s->{});\n", inner_name, fname));
                                }
                            }
                        }
                        TypeAnnotation::Generic(inner_name, _) => {
                            if inner_name != "Any" {
                                if self.enum_defs.contains_key(inner_name) {
                                    self.out.push_str(&format!("    free_enum_{}(s->{});\n", inner_name, fname));
                                } else if self.struct_defs.contains_key(inner_name) {
                                    self.out.push_str(&format!("    free_struct_{}(s->{});\n", inner_name, fname));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                self.out.push_str("    ep_gc_unregister(s);\n");
                self.out.push_str("    free(s);\n");
                self.out.push_str("}\n\n");
            }
        }

        // Emit C tagged-union structs for enums
        if !program.enum_defs.is_empty() {
            self.out.push_str("\n/* User-Defined Choices (Enums) */\n");
            for ed in &program.enum_defs {
                for (i, (vname, _)) in ed.variants.iter().enumerate() {
                    self.out.push_str(&format!("#define EP_TAG_{}_{} {}\n", ed.name, vname, i));
                }
                self.out.push_str("\n");

                let max_fields = ed.variants.iter().map(|(_, fields)| fields.len()).max().unwrap_or(0);
                self.out.push_str(&format!("typedef struct {{\n"));
                self.out.push_str("    long long tag;\n");
                for j in 0..max_fields {
                    self.out.push_str(&format!("    long long data{};\n", j));
                }
                self.out.push_str(&format!("}} EpEnum_{};\n\n", ed.name));

                self.out.push_str(&format!("void free_enum_{}(long long ptr) {{\n", ed.name));
                self.out.push_str("    if (ptr == 0) return;\n");
                self.out.push_str("    /* Skip if already freed (idempotent — prevents double-free in recursive enums) */\n");
                self.out.push_str("    if (!ep_gc_find((void*)ptr)) return;\n");
                self.out.push_str("    ep_gc_unregister((void*)ptr);\n");
                self.out.push_str(&format!("    EpEnum_{}* e = (EpEnum_{}*)ptr;\n", ed.name, ed.name));
                for (_i, (vname, fields)) in ed.variants.iter().enumerate() {
                    if fields.iter().any(|(_, ft)| matches!(ft, TypeAnnotation::List | TypeAnnotation::DynStr | TypeAnnotation::UserDefined(_) | TypeAnnotation::Generic(_, _))) {
                        self.out.push_str(&format!("    if (e->tag == EP_TAG_{}_{}) {{\n", ed.name, vname));
                        for (j, (_fname, ftype)) in fields.iter().enumerate() {
                            match ftype {
                                TypeAnnotation::List => {
                                    self.out.push_str(&format!("        free_list(e->data{});\n", j));
                                }
                                TypeAnnotation::DynStr => {
                                    self.out.push_str(&format!("        if (e->data{}) free((void*)e->data{});\n", j, j));
                                }
                                TypeAnnotation::UserDefined(inner_name) => {
                                    if inner_name != "Any" {
                                        if self.enum_defs.contains_key(inner_name) {
                                            self.out.push_str(&format!("        free_enum_{}(e->data{});\n", inner_name, j));
                                        } else if self.struct_defs.contains_key(inner_name) {
                                            self.out.push_str(&format!("        free_struct_{}(e->data{});\n", inner_name, j));
                                        }
                                    }
                                }
                                TypeAnnotation::Generic(inner_name, _) => {
                                    if inner_name != "Any" {
                                        if self.enum_defs.contains_key(inner_name) {
                                            self.out.push_str(&format!("        free_enum_{}(e->data{});\n", inner_name, j));
                                        } else if self.struct_defs.contains_key(inner_name) {
                                            self.out.push_str(&format!("        free_struct_{}(e->data{});\n", inner_name, j));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        self.out.push_str("    }\n");
                    }
                }
                self.out.push_str("    free(e);\n");
                self.out.push_str("}\n\n");

                for (_i, (vname, fields)) in ed.variants.iter().enumerate() {
                    let mut params = Vec::new();
                    for (j, _) in fields.iter().enumerate() {
                        params.push(format!("long long arg{}", j));
                    }
                    let params_str = if params.is_empty() { "void".to_string() } else { params.join(", ") };
                    self.out.push_str(&format!("long long create_{}_{}({}) {{\n", ed.name, vname, params_str));
                    self.out.push_str(&format!("    EpEnum_{}* e = (EpEnum_{}*)malloc(sizeof(EpEnum_{}));\n", ed.name, ed.name, ed.name));
                    self.out.push_str(&format!("    e->tag = EP_TAG_{}_{};
", ed.name, vname));
                    for (j, _) in fields.iter().enumerate() {
                        self.out.push_str(&format!("    e->data{} = arg{};\n", j, j));
                    }
                    self.out.push_str("    ep_gc_register(e, EP_OBJ_CLOSURE);\n"); // reuse CLOSURE kind for enums
                    self.out.push_str("    return (long long)e;\n");
                    self.out.push_str("}\n\n");
                }

                // Generate display helper: maps tag to variant name string
                self.out.push_str(&format!("const char* display_enum_{}(long long ptr) {{\n", ed.name));
                self.out.push_str(&format!("    if (ptr == 0) return \"(null)\";\n"));
                self.out.push_str(&format!("    EpEnum_{}* e = (EpEnum_{}*)ptr;\n", ed.name, ed.name));
                for (i, (vname, _)) in ed.variants.iter().enumerate() {
                    self.out.push_str(&format!("    if (e->tag == {}) return \"{}\";\n", i, vname));
                }
                self.out.push_str("    return \"(unknown)\";\n");
                self.out.push_str("}\n\n");
            }
        }

        // Emit concat built-in
        self.out.push_str("\n/* Built-in: string concatenation */\n");
        self.out.push_str("long long concat(long long a, long long b) {\n");
        self.out.push_str("    const char* sa = (const char*)a;\n");
        self.out.push_str("    const char* sb = (const char*)b;\n");
        self.out.push_str("    long long la = strlen(sa);\n");
        self.out.push_str("    long long lb = strlen(sb);\n");
        self.out.push_str("    char* result = malloc(la + lb + 1);\n");
        self.out.push_str("    memcpy(result, sa, la);\n");
        self.out.push_str("    memcpy(result + la, sb, lb);\n");
        self.out.push_str("    result[la + lb] = '\\0';\n");
        self.out.push_str("    ep_gc_register(result, EP_OBJ_STRING);\n");
        self.out.push_str("    return (long long)result;\n");
        self.out.push_str("}\n\n");

        self.out.push_str("long long int_to_string(long long val) {\n");
        self.out.push_str("    char* buf = malloc(32);\n");
        self.out.push_str("    snprintf(buf, 32, \"%lld\", val);\n");
        self.out.push_str("    ep_gc_register(buf, EP_OBJ_STRING);\n");
        self.out.push_str("    return (long long)buf;\n");
        self.out.push_str("}\n\n");

        // ep_int_to_str alias
        self.out.push_str("long long ep_int_to_str(long long val) { return int_to_string(val); }\n\n");

        // Native String Builder — realloc-based for O(n) amortized appends
        self.out.push_str("typedef struct { char* data; long long len; long long cap; } EpStringBuilder;\n\n");
        self.out.push_str("long long ep_sb_create(long long dummy) {\n");
        self.out.push_str("    (void)dummy;\n");
        self.out.push_str("    EpStringBuilder* sb = (EpStringBuilder*)malloc(sizeof(EpStringBuilder));\n");
        self.out.push_str("    sb->cap = 256;\n");
        self.out.push_str("    sb->len = 0;\n");
        self.out.push_str("    sb->data = (char*)malloc(sb->cap);\n");
        self.out.push_str("    sb->data[0] = '\\0';\n");
        self.out.push_str("    return (long long)sb;\n");
        self.out.push_str("}\n\n");

        self.out.push_str("long long ep_sb_append(long long sb_ptr, long long str_ptr) {\n");
        self.out.push_str("    EpStringBuilder* sb = (EpStringBuilder*)sb_ptr;\n");
        self.out.push_str("    const char* s = (const char*)str_ptr;\n");
        self.out.push_str("    if (!s) return sb_ptr;\n");
        self.out.push_str("    long long slen = strlen(s);\n");
        self.out.push_str("    while (sb->len + slen + 1 > sb->cap) {\n");
        self.out.push_str("        sb->cap *= 2;\n");
        self.out.push_str("        sb->data = (char*)realloc(sb->data, sb->cap);\n");
        self.out.push_str("    }\n");
        self.out.push_str("    memcpy(sb->data + sb->len, s, slen);\n");
        self.out.push_str("    sb->len += slen;\n");
        self.out.push_str("    sb->data[sb->len] = '\\0';\n");
        self.out.push_str("    return sb_ptr;\n");
        self.out.push_str("}\n\n");

        self.out.push_str("long long ep_sb_append_int(long long sb_ptr, long long val) {\n");
        self.out.push_str("    char buf[32];\n");
        self.out.push_str("    snprintf(buf, sizeof(buf), \"%lld\", val);\n");
        self.out.push_str("    return ep_sb_append(sb_ptr, (long long)buf);\n");
        self.out.push_str("}\n\n");

        self.out.push_str("long long ep_sb_to_string(long long sb_ptr) {\n");
        self.out.push_str("    EpStringBuilder* sb = (EpStringBuilder*)sb_ptr;\n");
        self.out.push_str("    char* result = (char*)malloc(sb->len + 1);\n");
        self.out.push_str("    memcpy(result, sb->data, sb->len + 1);\n");
        self.out.push_str("    ep_gc_register(result, EP_OBJ_STRING);\n");
        self.out.push_str("    free(sb->data);\n");
        self.out.push_str("    free(sb);\n");
        self.out.push_str("    return (long long)result;\n");
        self.out.push_str("}\n\n");

        self.out.push_str("long long ep_sb_length(long long sb_ptr) {\n");
        self.out.push_str("    return ((EpStringBuilder*)sb_ptr)->len;\n");
        self.out.push_str("}\n\n");

        // FFI pointer/byte builtins
        self.out.push_str("long long str_to_ptr(long long s) { return s; }\n");
        self.out.push_str("long long ptr_to_str(long long p) {\n");
        self.out.push_str("    if (p == 0) return (long long)strdup(\"\");\n");
        self.out.push_str("    char* copy = strdup((const char*)p);\n");
        self.out.push_str("    ep_gc_register(copy, EP_OBJ_STRING);\n");
        self.out.push_str("    return (long long)copy;\n");
        self.out.push_str("}\n\n");
        self.out.push_str("long long peek_byte(long long ptr, long long offset) {\n");
        self.out.push_str("    return (long long)((unsigned char*)ptr)[offset];\n");
        self.out.push_str("}\n");
        self.out.push_str("long long poke_byte(long long ptr, long long offset, long long value) {\n");
        self.out.push_str("    ((unsigned char*)ptr)[offset] = (unsigned char)value;\n");
        self.out.push_str("    return 0;\n");
        self.out.push_str("}\n");
        self.out.push_str("long long alloc_bytes(long long size) {\n");
        self.out.push_str("    return (long long)calloc((size_t)size, 1);\n");
        self.out.push_str("}\n");
        self.out.push_str("long long free_bytes(long long ptr) {\n");
        self.out.push_str("    free((void*)ptr);\n");
        self.out.push_str("    return 0;\n");
        self.out.push_str("}\n");
        self.out.push_str("long long list_to_bytes(long long list_ptr) {\n");
        self.out.push_str("    long long len = length_list(list_ptr);\n");
        self.out.push_str("    unsigned char* buf = (unsigned char*)malloc(len);\n");
        self.out.push_str("    for (long long i = 0; i < len; i++) {\n");
        self.out.push_str("        buf[i] = (unsigned char)get_list(list_ptr, i);\n");
        self.out.push_str("    }\n");
        self.out.push_str("    return (long long)buf;\n");
        self.out.push_str("}\n");
        self.out.push_str("long long bytes_to_list(long long ptr, long long len) {\n");
        self.out.push_str("    long long list = create_list();\n");
        self.out.push_str("    unsigned char* buf = (unsigned char*)ptr;\n");
        self.out.push_str("    for (long long i = 0; i < len; i++) {\n");
        self.out.push_str("        append_list(list, (long long)buf[i]);\n");
        self.out.push_str("    }\n");
        self.out.push_str("    return list;\n");
        self.out.push_str("}\n\n");
        self.out.push_str("long long ep_gc_get_minor_count() {\n");
        self.out.push_str("    return ep_gc_minor_count;\n");
        self.out.push_str("}\n");
        self.out.push_str("long long ep_gc_get_major_count() {\n");
        self.out.push_str("    return ep_gc_major_count;\n");
        self.out.push_str("}\n");
        self.out.push_str("long long ep_gc_get_nursery_count() {\n");
        self.out.push_str("    return ep_gc_nursery_count;\n");
        self.out.push_str("}\n\n");

        self.out.push_str("long long string_to_int(long long s) {\n");
        self.out.push_str("    if (s == 0) return 0;\n");
        self.out.push_str("    return atoll((const char*)s);\n");
        self.out.push_str("}\n\n");

        // read_line: reads a line from stdin, returns dynamically allocated string
        self.out.push_str("long long read_line() {\n");
        self.out.push_str("    char buf[4096];\n");
        self.out.push_str("    if (fgets(buf, sizeof(buf), stdin) == NULL) { buf[0] = '\\0'; }\n");
        self.out.push_str("    size_t len = strlen(buf);\n");
        self.out.push_str("    if (len > 0 && buf[len-1] == '\\n') buf[len-1] = '\\0';\n");
        self.out.push_str("    char* result = strdup(buf);\n");
        self.out.push_str("    ep_gc_register(result, EP_OBJ_STRING);\n");
        self.out.push_str("    return (long long)result;\n");
        self.out.push_str("}\n\n");

        // read_int: reads an integer from stdin
        self.out.push_str("long long read_int() {\n");
        self.out.push_str("    long long val = 0;\n");
        self.out.push_str("    scanf(\"%lld\", &val);\n");
        self.out.push_str("    while(getchar() != '\\n');\n");
        self.out.push_str("    return val;\n");
        self.out.push_str("}\n\n");

        // read_float: reads a float from stdin, returns as type-punned long long
        self.out.push_str("long long read_float() {\n");
        self.out.push_str("    double val = 0.0;\n");
        self.out.push_str("    scanf(\"%lf\", &val);\n");
        self.out.push_str("    while(getchar() != '\\n');\n");
        self.out.push_str("    long long result; memcpy(&result, &val, sizeof(double));\n");
        self.out.push_str("    return result;\n");
        self.out.push_str("}\n\n");

        // int_to_float: converts int to float (type-punned as long long)
        self.out.push_str("long long int_to_float(long long val) {\n");
        self.out.push_str("    double d = (double)val;\n");
        self.out.push_str("    long long result; memcpy(&result, &d, sizeof(double));\n");
        self.out.push_str("    return result;\n");
        self.out.push_str("}\n\n");

        // float_to_int: converts float (type-punned long long) back to int
        self.out.push_str("long long float_to_int(long long val) {\n");
        self.out.push_str("    double d; memcpy(&d, &val, sizeof(double));\n");
        self.out.push_str("    return (long long)d;\n");
        self.out.push_str("}\n\n");

        self.out.push_str("\n/* External Function Prototypes (FFI) */\n");
        for ext in &program.externals {
            // Skip forward declarations for functions already in the C runtime
            if self.builtin_c_funcs.contains(&ext.name) {
                continue;
            }
            let mut params_str = Vec::new();
            for _ in &ext.params {
                params_str.push("long long");
            }
            self.out.push_str(&format!("long long {}({});\n", ext.name, params_str.join(", ")));
        }
        self.out.push_str("\n");

        // Declare top-level constant globals BEFORE function prototypes/definitions
        // so bridge constants (e.g. SDL_HANDLE) are visible to all functions
        if !program.top_level_constants.is_empty() {
            self.out.push_str("\n/* Top-Level Constant Globals (Bridge Libraries) */\n");
            for stmt in &program.top_level_constants {
                if let StmtNode::Set(name, _, _) = &stmt.node {
                    self.out.push_str(&format!("long long {} = 0;\n", name));
                }
            }
            self.out.push_str("\n");
        }

        self.out.push_str("\n/* User Function Prototypes */\n");
        for func in &program.functions {
            // Skip prototypes for functions that shadow C runtime builtins
            if self.builtin_c_funcs.contains(&func.name) {
                continue;
            }
            let mut params_str = Vec::new();
            for _ in &func.params {
                params_str.push("long long");
            }
            let name = if func.name == "main" { "_main".to_string() } else { Self::sanitize_c_name(&func.name) };
            self.out.push_str(&format!("long long {}({});\n", name, params_str.join(", ")));
            if func.is_async {
                self.out.push_str(&format!("long long {}_impl({});\n", name, params_str.join(", ")));
                self.out.push_str(&format!("void* {}_async_wrapper(void* r);\n", name));
            }
        }
        self.out.push_str("\n");

        // Method prototypes
        for md in &program.method_defs {
            let mut params_str = vec!["long long".to_string()]; // self
            for _ in &md.params {
                params_str.push("long long".to_string());
            }
            self.out.push_str(&format!("long long {}__{}({});\n", md.struct_name, md.name, params_str.join(", ")));
        }
        for ti in &program.trait_impls {
            for m in &ti.methods {
                let mut params_str = vec!["long long".to_string()]; // self
                for _ in &m.params {
                    params_str.push("long long".to_string());
                }
                self.out.push_str(&format!("long long {}__{}({});\n", ti.for_type, m.name, params_str.join(", ")));
            }
        }
        self.out.push_str("\n");
        // Marker for closure forward declarations (inserted after codegen)
        self.out.push_str("/* EP_CLOSURE_FWD_DECLS */\n");
        // Marker for closure function bodies (inserted during codegen)
        self.out.push_str("/* EP_CLOSURE_BODIES */\n");

        let spawn_list = self.collect_all_spawns(program);
        self.out.push_str("\n/* Thread Spawn Wrappers */\n");
        for (idx, spawn_stmt) in spawn_list.iter().enumerate() {
            if let StmtNode::Spawn(func_name, args) = &spawn_stmt.node {
                self.out.push_str(&format!("typedef struct {{\n"));
                for j in 0..args.len() {
                    self.out.push_str(&format!("    long long arg{};\n", j));
                }
                if args.is_empty() {
                    self.out.push_str("    int dummy;\n");
                }
                self.out.push_str(&format!("}} spawn_args_{};\n\n", idx));

                let c_name = if func_name == "main" { "_main".to_string() } else { func_name.clone() };
                self.out.push_str(&format!("void* spawn_wrapper_{}(void* r) {{\n", idx));
                self.out.push_str("    int stack_dummy;\n");
                self.out.push_str("    ep_gc_register_thread(&stack_dummy);\n");
                self.out.push_str(&format!("    spawn_args_{}* args = (spawn_args_{}*)r;\n", idx, idx));
                
                let mut args_joined = Vec::new();
                for j in 0..args.len() {
                    args_joined.push(format!("args->arg{}", j));
                }
                self.out.push_str(&format!("    {}({});\n", c_name, args_joined.join(", ")));
                self.out.push_str("    free(args);\n");
                self.out.push_str("    ep_gc_unregister_thread();\n");
                self.out.push_str("    return NULL;\n");
                self.out.push_str("}\n\n");
            }
        }
        self.out.push_str("\n");

        self.spawn_index = 0;
        self.spawn_wrapper_index = 0;

        // Pre-scan all functions to collect closure names and emit forward declarations
        self.closure_c_names.clear();
        self.closure_captures.clear();
        for func in &program.functions {
            let mut var_types = HashMap::new();
            for param in &func.params {
                let t = if let Some(ann) = &param.2 {
                    self.type_annotation_to_type(ann)
                } else {
                    Type::Int
                };
                var_types.insert(param.0.clone(), t);
            }
            self.collect_var_types(&func.body, &mut var_types);
        }
        
        // Note: closure forward declarations are deferred until after gen_function
        // because capture analysis happens during codegen
        let closure_fwd_decls_placeholder = true; // marker for post-codegen generation

        for func in &program.functions {
            // Skip user-defined functions that shadow C runtime builtins
            if self.builtin_c_funcs.contains(&func.name) {
                continue;
            }
            self.gen_function(func)?;
        }

        // Generate method implementations
        for md in &program.method_defs {
            self.gen_method(md)?;
        }
        for ti in &program.trait_impls {
            for m in &ti.methods {
                let md = MethodDef {
                    name: m.name.clone(),
                    struct_name: ti.for_type.clone(),
                    params: m.params.clone(),
                    return_type: m.return_type.clone(),
                    body: m.body.clone(),
                    doc_comment: m.doc_comment.clone(),
                };
                self.gen_method(&md)?;
            }
        }

        if self.is_test_mode {
            self.out.push_str(&self.get_c_test_main_source(program));
        } else {
            // Emit top-level constant init function (globals already declared earlier)
            if !program.top_level_constants.is_empty() {
                // Generate GC marking functions for top-level globals. These are
                // roots that live outside any function frame, so the collector
                // must trace them or objects reachable only via a global get swept.
                let global_names: Vec<&String> = program.top_level_constants.iter()
                    .filter_map(|stmt| match &stmt.node {
                        StmtNode::Set(name, _, _) => Some(name),
                        _ => None,
                    })
                    .collect();
                self.out.push_str("\nstatic void __ep_mark_globals_major(void) {\n");
                for name in &global_names {
                    self.out.push_str(&format!("    if ({} != 0) ep_gc_mark_object((void*){});\n", name, name));
                }
                self.out.push_str("}\n");
                self.out.push_str("static void __ep_mark_globals_minor(void) {\n");
                for name in &global_names {
                    self.out.push_str(&format!("    if ({} != 0) ep_gc_mark_object_minor((void*){});\n", name, name));
                }
                self.out.push_str("}\n");

                // Generate init function
                self.out.push_str("\nvoid __ep_init_constants(void) {\n");
                self.out.push_str("    ep_gc_mark_globals_major = __ep_mark_globals_major;\n");
                self.out.push_str("    ep_gc_mark_globals_minor = __ep_mark_globals_minor;\n");
                for stmt in &program.top_level_constants {
                    if let StmtNode::Set(name, expr, _) = &stmt.node {
                        let empty_types: HashMap<String, Type> = HashMap::new();
                        let val = self.gen_expr(expr, &empty_types)?;
                        self.out.push_str(&format!("    {} = {};\n", name, val));
                    }
                }
                self.out.push_str("}\n\n");
                let main_is_async = program.functions.iter().any(|f| f.name == "main" && f.is_async);
                let main_call = if main_is_async {
                    "    EpFuture* fut = (EpFuture*)_main();\n    ep_async_loop_run();\n    int result = (int)(fut ? fut->value : 0);"
                } else {
                    "    int result = (int)_main();\n    ep_async_loop_run();"
                };
                self.out.push_str(&format!(r#"/* Bootstrapper C main */
int main(int argc, char** argv) {{
    {{
        unsigned int seed;
        FILE* urand = fopen("/dev/urandom", "rb");
        if (urand && fread(&seed, sizeof(seed), 1, urand) == 1) {{
            fclose(urand);
        }} else {{
            if (urand) fclose(urand);
            seed = (unsigned int)time(NULL) ^ (unsigned int)getpid();
        }}
        srand(seed);
    }}
    init_ep_args(argc, argv);
    __ep_init_constants();
{}
    ep_gc_shutdown();
    return result;
}}
"#, main_call));
            } else {
                let main_is_async = program.functions.iter().any(|f| f.name == "main" && f.is_async);
                let main_call = if main_is_async {
                    "    EpFuture* fut = (EpFuture*)_main();\n    ep_async_loop_run();\n    int result = (int)(fut ? fut->value : 0);"
                } else {
                    "    int result = (int)_main();\n    ep_async_loop_run();"
                };
                self.out.push_str(&format!(r#"
/* Bootstrapper C main */
int main(int argc, char** argv) {{
    {{
        unsigned int seed;
        FILE* urand = fopen("/dev/urandom", "rb");
        if (urand && fread(&seed, sizeof(seed), 1, urand) == 1) {{
            fclose(urand);
        }} else {{
            if (urand) fclose(urand);
            seed = (unsigned int)time(NULL) ^ (unsigned int)getpid();
        }}
        srand(seed);
    }}
    init_ep_args(argc, argv);
{}
    ep_gc_shutdown();
    return result;
}}
"#, main_call));
            }
        }

        // Generate closure forward declarations now that captures are known
        let _ = closure_fwd_decls_placeholder; // suppress unused warning
        if !self.closure_c_names.is_empty() {
            let mut closure_fwd_decls = String::new();
            closure_fwd_decls.push_str("\n/* Forward declarations for closures */\n");
            for func in &program.functions {
                for stmt in &func.body {
                    if let StmtNode::Set(name, expr, _) = &stmt.node {
                        if let ExprNode::Closure(params, _) = &expr.node {
                            if let Some(c_name) = self.closure_c_names.get(name) {
                                // Always: _ep_env + user params
                                let param_count = 1 + params.len();
                                let param_str = (0..param_count).map(|_| "long long").collect::<Vec<_>>().join(", ");
                                closure_fwd_decls.push_str(&format!("long long {}({});\n", c_name, param_str));
                            }
                        }
                    }
                }
            }
            closure_fwd_decls.push('\n');
            // Insert at marker position (after runtime defs, before user functions)
            self.out = self.out.replace("/* EP_CLOSURE_FWD_DECLS */\n", &closure_fwd_decls);
        } else {
            // Remove the markers if no closures
            self.out = self.out.replace("/* EP_CLOSURE_FWD_DECLS */\n", "");
        }
        // Clean up bodies marker (already consumed during codegen, but might remain)
        self.out = self.out.replace("/* EP_CLOSURE_BODIES */\n", "");

        Ok(self.out.clone())
    }

    fn gen_function(&mut self, func: &Function) -> Result<(), String> {
        let mut var_types = HashMap::new();

        /* Clear per-function state that would otherwise leak across functions.
           list_element_types: bare variable names like "votes" collide between
           functions, causing wrong for-each loop variable types.
           closure_c_names: a closure named "x" in function A would cause
           function B's call to a regular function "x" to be misrouted.
           We clear both and re-collect for THIS function only. */
        self.list_element_types.clear();
        self.closure_c_names.clear();

        for param in &func.params {
            let param_type = if let Some(ref ann) = param.2 {
                self.type_annotation_to_type(ann)
            } else if param.1 {
                Type::RefList
            } else {
                // Try to infer struct type from field access patterns
                self.infer_param_struct_type(&param.0, &func.body).unwrap_or(Type::Int)
            };
            var_types.insert(param.0.clone(), param_type);
        }
        /* collect_var_types populates list_element_types and closure_c_names
           for THIS function's body only (since we cleared them above). */
        self.collect_var_types(&func.body, &mut var_types);
        self.infer_param_types_from_usage(&func.body, &mut var_types);

        self.current_return_type = self.func_return_types.get(&func.name).cloned().unwrap_or(Type::Int);

        let name = if func.name == "main" {
            "_main".to_string()
        } else {
            Self::sanitize_c_name(&func.name)
        };
        
        let mut params_decl = Vec::new();
        for param in &func.params {
            params_decl.push(format!("long long {}", param.0));
        }

        if func.is_async {
            // 1. Generate the argument struct
            self.out.push_str(&format!("typedef struct {{\n"));
            self.out.push_str("    int state;\n");
            self.out.push_str("    EpFuture* fut;\n");
            // Parameters:
            for param in &func.params {
                self.out.push_str(&format!("    long long {};\n", Self::sanitize_c_name(&param.0)));
            }
            // Local variables:
            for (var_name, _) in &var_types {
                let is_param = func.params.iter().any(|p| &p.0 == var_name);
                let is_global = self.global_constants.contains(var_name);
                if !is_param && !is_global {
                    self.out.push_str(&format!("    long long {};\n", Self::sanitize_c_name(var_name)));
                }
            }
            // Awaited futures:
            let await_count = self.count_awaits_in_stmts(&func.body);
            for i in 1..=await_count {
                self.out.push_str(&format!("    EpFuture* awaited_fut_{};\n", i));
            }
            if func.params.is_empty() && var_types.len() == func.params.len() && await_count == 0 {
                self.out.push_str("    int dummy;\n");
            }
            self.out.push_str(&format!("}} {}_async_args;\n\n", name));

            // 2. Generate the step function
            self.out.push_str(&format!("long long {}_step(void* r) {{\n", name));
            self.out.push_str(&format!("    {}_async_args* args = ({}_async_args*)r;\n", name, name));
            self.out.push_str("    switch (args->state) {\n");
            self.out.push_str("        case 0:\n");

            // Compile the statements:
            self.await_counter = 0;
            self.is_async_func = true;
            self.current_async_func_locals.clear();
            for k in var_types.keys() {
                self.current_async_func_locals.insert(k.clone());
            }

            for stmt in &func.body {
                let saved_await_counter = self.await_counter;
                self.emit_yields_for_statement(stmt, &var_types)?;
                let post_scan_await_counter = self.await_counter;
                self.await_counter = saved_await_counter;

                self.gen_statement(stmt, &var_types)?;

                self.await_counter = post_scan_await_counter;
            }

            self.out.push_str("            args->state = -1;\n");
            self.out.push_str("            return 0;\n");
            self.out.push_str("    }\n");
            self.out.push_str("    return 0;\n");
            self.out.push_str("}\n\n");

            // 3. Generate the public wrapper function (the entry point for calling the async function)
            self.out.push_str(&format!("long long {}({}) {{\n", name, params_decl.join(", ")));
            self.out.push_str("    EpFuture* fut = (EpFuture*)malloc(sizeof(EpFuture));\n");
            self.out.push_str("    fut->completed = 0;\n");
            self.out.push_str("    fut->value = 0;\n");
            self.out.push_str("    fut->waiting_task = NULL;\n");
            self.out.push_str("    { EpGCObject* _go = ep_gc_register(fut, EP_OBJ_STRUCT); if(_go) _go->num_fields = 3; }\n");
            self.out.push_str(&format!("    {}_async_args* args = ({}_async_args*)malloc(sizeof({}_async_args));\n", name, name, name));
            self.out.push_str(&format!("    memset(args, 0, sizeof({}_async_args));\n", name));
            self.out.push_str("    args->state = 0;\n");
            self.out.push_str("    args->fut = fut;\n");
            for param in &func.params {
                self.out.push_str(&format!("    args->{} = {};\n", Self::sanitize_c_name(&param.0), Self::sanitize_c_name(&param.0)));
            }
            self.out.push_str("    EpTask* task = (EpTask*)malloc(sizeof(EpTask));\n");
            self.out.push_str(&format!("    task->step = {}_step;\n", name));
            self.out.push_str("    task->args = args;\n");
            self.out.push_str(&format!("    task->args_size_bytes = sizeof({}_async_args);\n", name));
            self.out.push_str("    task->fut = fut;\n");
            self.out.push_str("    task->state = 0;\n");
            self.out.push_str("    task->is_cancelled = 0;\n");
            self.out.push_str("    task->parent = ep_current_task;\n");
            self.out.push_str("    ep_task_enqueue(task);\n");
            self.out.push_str("    return (long long)fut;\n");
            self.out.push_str("}\n\n");

            self.is_async_func = false;
            self.current_async_func_locals.clear();
            return Ok(());
        }

        let impl_name = if func.is_async {
            format!("{}_impl", name)
        } else {
            name.clone()
        };
        self.out.push_str(&format!("long long {}({}) {{\n", impl_name, params_decl.join(", ")));

        let mut borrowed_vars = std::collections::HashSet::new();
        for param in &func.params {
            borrowed_vars.insert(param.0.clone());
        }

        fn is_borrow_expr(expr: &Expr, borrowed: &std::collections::HashSet<String>) -> bool {
            match &expr.node {
                ExprNode::Borrow(_) => true,
                ExprNode::FieldAccess(_, _) => true,
                ExprNode::Identifier(name) => borrowed.contains(name),
                ExprNode::Call(name, _) => {
                    let name_lower = name.to_lowercase();
                    name_lower.contains("get") || name_lower.contains("peek")
                }
                ExprNode::MethodCall(_, name, _) => {
                    let name_lower = name.to_lowercase();
                    name_lower.contains("get") || name_lower.contains("peek")
                }
                _ => false,
            }
        }

        fn scan_stmts_for_borrows(
            stmts: &[Stmt],
            borrowed: &mut std::collections::HashSet<String>,
        ) {
            for stmt in stmts {
                match &stmt.node {
                    StmtNode::Set(name, expr, _) => {
                        if is_borrow_expr(expr, borrowed) {
                            borrowed.insert(name.clone());
                        } else {
                            borrowed.remove(name);
                        }
                    }
                    StmtNode::If(_, then_branch, else_branch) => {
                        scan_stmts_for_borrows(then_branch, borrowed);
                        if let Some(else_stmts) = else_branch {
                            scan_stmts_for_borrows(else_stmts, borrowed);
                        }
                    }
                    StmtNode::RepeatWhile(_, body) => {
                        scan_stmts_for_borrows(body, borrowed);
                    }
                    StmtNode::ForEach(loop_var, _, body) => {
                        borrowed.insert(loop_var.clone());
                        scan_stmts_for_borrows(body, borrowed);
                    }
                    StmtNode::Match(_, arms) => {
                        for (_, bindings, body) in arms {
                            for binding in bindings {
                                borrowed.insert(binding.clone());
                            }
                            scan_stmts_for_borrows(body, borrowed);
                        }
                    }
                    _ => {}
                }
            }
        }

        scan_stmts_for_borrows(&func.body, &mut borrowed_vars);
        
        for (var_name, _) in &var_types {
            let is_param = func.params.iter().any(|p| &p.0 == var_name);
            let is_global = self.global_constants.contains(var_name);
            if !is_param && !is_global {
                let safe_var = Self::sanitize_c_name(var_name);
                self.out.push_str(&format!("    long long {} = 0;\n", safe_var));
            }
        }
        self.out.push_str("    long long ret_val = 0;\n\n");

        // Push GC roots for all locals
        let mut gc_root_count = 0;
        for (var_name, _) in &var_types {
            let is_param = func.params.iter().any(|p| &p.0 == var_name);
            let is_global = self.global_constants.contains(var_name);
            if !is_param && !is_global {
                let t = var_types.get(var_name);
                let should_root = t.map(|ty| {
                    if !needs_gc_root(ty) {
                        func.body.iter().any(|s| stmt_contains_non_primitive_usage(s, var_name))
                    } else {
                        true
                    }
                }).unwrap_or(true);
                if should_root {
                    self.out.push_str(&format!("    ep_gc_push_root(&{});\n", Self::sanitize_c_name(var_name)));
                    gc_root_count += 1;
                }
            }
        }
        // Also push params as GC roots
        for param in &func.params {
            let t = var_types.get(&param.0);
            let should_root = t.map(|ty| {
                if !needs_gc_root(ty) {
                    func.body.iter().any(|s| stmt_contains_non_primitive_usage(s, &param.0))
                } else {
                    true
                }
            }).unwrap_or(true);
            if should_root {
                self.out.push_str(&format!("    ep_gc_push_root(&{});\n", param.0));
                gc_root_count += 1;
            }
        }
        if gc_root_count > 0 {
            self.out.push_str("\n");
        }

        // GC safe point: collect only if this function uses heap-allocated data
        let needs_gc = gc_root_count > 0 || var_types.values().any(|t| 
            matches!(t, Type::List | Type::Map | Type::DynStr | Type::Struct(_) | Type::Enum(_) | Type::RefList)
        );
        if needs_gc {
            self.out.push_str("    ep_gc_maybe_collect();\n\n");
        }

        for stmt in &func.body {
            self.gen_statement(stmt, &var_types)?;
        }

        self.out.push_str("L_cleanup:\n");
        // Pop GC roots
        if gc_root_count > 0 {
            self.out.push_str(&format!("    ep_gc_pop_roots({});\n", gc_root_count));
        }

        // Scan function body for explicit free_list/free_map calls
        // to avoid double-freeing in auto-cleanup
        let mut user_freed: std::collections::HashSet<String> = std::collections::HashSet::new();
        Self::collect_user_freed(&func.body, &mut user_freed);

        // Scan for ownership transfers: variables passed to container-insertion
        // functions (map_insert, append_list, set_list) should NOT be auto-freed
        // because the container now owns them. Also tracks variables passed to
        // user-defined functions, since the callee could transfer ownership.
        let mut transferred: std::collections::HashSet<String> = std::collections::HashSet::new();
        Self::collect_transferred(&func.body, &mut transferred, &self.builtin_c_funcs);

        // Get the function's return type to avoid freeing the returned value
        let func_ret_type = self.func_return_types.get(&func.name).cloned();
        for (var_name, _) in &var_types {
            let is_param = func.params.iter().any(|p| &p.0 == var_name);
            let is_global = self.global_constants.contains(var_name);
            let is_borrowed = borrowed_vars.contains(var_name);
            if !is_param && !is_global && !is_borrowed {
                let t = var_types.get(var_name);
                // Skip freeing if this local has the same struct/enum type as the return type
                // because it may hold the value being returned via ret_val
                if let Some(ref frt) = func_ret_type {
                    if t == Some(frt) {
                        continue;
                    }
                }
                // Skip if user already explicitly freed this variable
                if user_freed.contains(var_name) {
                    continue;
                }
                // Skip if ownership was transferred to a container
                if transferred.contains(var_name) {
                    continue;
                }
                if t == Some(&Type::List) {
                    self.out.push_str(&format!("    free_list({});\n", var_name));
                    self.out.push_str(&format!("    {} = 0;\n", var_name));
                } else if t == Some(&Type::Map) {
                    self.out.push_str(&format!("    free_map({});\n", var_name));
                    self.out.push_str(&format!("    {} = 0;\n", var_name));
                } else if let Some(Type::Struct(sname)) = t {
                    if sname == "Map" {
                        self.out.push_str(&format!("    free_map({});\n", var_name));
                        self.out.push_str(&format!("    {} = 0;\n", var_name));
                    } else if sname == "Deque" {
                        // Deque is a built-in, skip struct free
                    } else {
                        self.out.push_str(&format!("    free_struct_{}({});\n", sname, var_name));
                        self.out.push_str(&format!("    {} = 0;\n", var_name));
                    }
                } else if let Some(Type::Enum(ename)) = t {
                    self.out.push_str(&format!("    free_enum_{}({});\n", ename, var_name));
                    self.out.push_str(&format!("    {} = 0;\n", var_name));
                }
            }
        }
        self.out.push_str("    return ret_val;\n}\n\n");

        Ok(())
    }

    /// Scan statements for explicit free_list/free_map calls and collect the variable names
    fn collect_user_freed(stmts: &[Stmt], freed: &mut std::collections::HashSet<String>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(_, expr, _) | StmtNode::ExprStmt(expr) => {
                    // Check if the expression is a call to free_list or free_map
                    if let ExprNode::Call(name, args) = &expr.node {
                        if (name == "free_list" || name == "free_map") && !args.is_empty() {
                            if let ExprNode::Identifier(var_name) = &args[0].node {
                                freed.insert(var_name.clone());
                            }
                        }
                    }
                }
                StmtNode::If(_, then_branch, else_branch) => {
                    Self::collect_user_freed(then_branch, freed);
                    if let Some(else_stmts) = else_branch {
                        Self::collect_user_freed(else_stmts, freed);
                    }
                }
                StmtNode::RepeatWhile(_, body) => {
                    Self::collect_user_freed(body, freed);
                }
                StmtNode::ForEach(_, _, body) => {
                    Self::collect_user_freed(body, freed);
                }
                _ => {}
            }
        }
    }

    /// Scan statements for ownership-transferring calls and collect variable names.
    /// When a list/map/struct variable is passed as a value to a container-insertion
    /// function, ownership is transferred — the container now owns it.
    /// Also tracks variables passed to user-defined functions, since the caller
    /// can't know whether the callee stores the value in a container.
    fn collect_transferred(stmts: &[Stmt], transferred: &mut std::collections::HashSet<String>,
                           builtins: &std::collections::HashSet<String>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(_, expr, _) | StmtNode::ExprStmt(expr) => {
                    Self::collect_transferred_from_expr(expr, transferred, builtins);
                }
                StmtNode::If(_, then_branch, else_branch) => {
                    Self::collect_transferred(then_branch, transferred, builtins);
                    if let Some(else_stmts) = else_branch {
                        Self::collect_transferred(else_stmts, transferred, builtins);
                    }
                }
                StmtNode::RepeatWhile(_, body) => {
                    Self::collect_transferred(body, transferred, builtins);
                }
                StmtNode::ForEach(_, _, body) => {
                    Self::collect_transferred(body, transferred, builtins);
                }
                StmtNode::Send(_, val) => {
                    // Sending a value over a channel transfers ownership
                    if let ExprNode::Identifier(var_name) = &val.node {
                        transferred.insert(var_name.clone());
                    }
                }
                StmtNode::Return(expr) => {
                    // A return expression may call user functions that consume locals
                    Self::collect_transferred_from_expr(expr, transferred, builtins);
                    // Also mark any identifier directly returned as transferred
                    // (it's being moved out of the function via ret_val)
                    if let ExprNode::Identifier(var_name) = &expr.node {
                        transferred.insert(var_name.clone());
                    }
                }
                StmtNode::Display(expr) => {
                    Self::collect_transferred_from_expr(expr, transferred, builtins);
                }
                StmtNode::Match(_, arms) => {
                    for (_, _, body) in arms {
                        Self::collect_transferred(body, transferred, builtins);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_transferred_from_expr(expr: &Expr, transferred: &mut std::collections::HashSet<String>,
                                     builtins: &std::collections::HashSet<String>) {
        if let ExprNode::Call(name, args) = &expr.node {
            match name.as_str() {
                // map_insert(map, key, value) — 3rd arg (index 2) is transferred
                "map_insert" if args.len() >= 3 => {
                    if let ExprNode::Identifier(var_name) = &args[2].node {
                        transferred.insert(var_name.clone());
                    }
                }
                // append_list(list, value) — 2nd arg (index 1) is transferred
                "append_list" if args.len() >= 2 => {
                    if let ExprNode::Identifier(var_name) = &args[1].node {
                        transferred.insert(var_name.clone());
                    }
                }
                // set_list(list, index, value) — 3rd arg (index 2) is transferred
                "set_list" if args.len() >= 3 => {
                    if let ExprNode::Identifier(var_name) = &args[2].node {
                        transferred.insert(var_name.clone());
                    }
                }
                _ => {
                    // For user-defined functions (not builtins), any argument
                    // could be stored in a container by the callee — we can't
                    // know, so mark all identifier args as potentially transferred.
                    if !builtins.contains(name) {
                        for arg in args {
                            if let ExprNode::Identifier(var_name) = &arg.node {
                                transferred.insert(var_name.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    fn gen_method(&mut self, md: &MethodDef) -> Result<(), String> {
        let mut var_types = HashMap::new();
        self.list_element_types.clear();
        self.closure_c_names.clear();

        // Type self correctly based on whether the target is an enum or struct
        if self.enum_defs.contains_key(&md.struct_name) {
            var_types.insert("self".to_string(), Type::Enum(md.struct_name.clone()));
        } else {
            var_types.insert("self".to_string(), Type::Struct(md.struct_name.clone()));
        }

        for param in &md.params {
            let param_type = if let Some(ref ann) = param.2 {
                self.type_annotation_to_type(ann)
            } else if param.1 {
                Type::RefList
            } else {
                self.infer_param_struct_type(&param.0, &md.body).unwrap_or(Type::Int)
            };
            var_types.insert(param.0.clone(), param_type);
        }
        self.collect_var_types(&md.body, &mut var_types);
        self.infer_param_types_from_usage(&md.body, &mut var_types);

        let key = format!("{}_{}", md.struct_name, md.name);
        self.current_return_type = self.func_return_types.get(&key).cloned().unwrap_or(Type::Int);

        let mut params_decl = vec!["long long self".to_string()];
        for param in &md.params {
            params_decl.push(format!("long long {}", param.0));
        }

        self.out.push_str(&format!("long long {}__{}({}) {{\n", md.struct_name, md.name, params_decl.join(", ")));

        for (var_name, _) in &var_types {
            let is_param = var_name == "self" || md.params.iter().any(|p| &p.0 == var_name);
            if !is_param {
                let safe_var = Self::sanitize_c_name(var_name);
                self.out.push_str(&format!("    long long {} = 0;\n", safe_var));
            }
        }
        self.out.push_str("    long long ret_val = 0;\n\n");

        // Push GC roots for all locals
        let mut gc_root_count = 0;
        for (var_name, _) in &var_types {
            let is_param = var_name == "self" || md.params.iter().any(|p| &p.0 == var_name);
            if !is_param {
                let t = var_types.get(var_name);
                let should_root = t.map(|ty| {
                    if !needs_gc_root(ty) {
                        md.body.iter().any(|s| stmt_contains_non_primitive_usage(s, var_name))
                    } else {
                        true
                    }
                }).unwrap_or(true);
                if should_root {
                    self.out.push_str(&format!("    ep_gc_push_root(&{});\n", Self::sanitize_c_name(var_name)));
                    gc_root_count += 1;
                }
            }
        }
        // Also push self as GC root
        {
            let t = var_types.get("self");
            let should_root = t.map(|ty| {
                if !needs_gc_root(ty) {
                    md.body.iter().any(|s| stmt_contains_non_primitive_usage(s, "self"))
                } else {
                    true
                }
            }).unwrap_or(true);
            if should_root {
                self.out.push_str("    ep_gc_push_root(&self);\n");
                gc_root_count += 1;
            }
        }
        // Also push params as GC roots
        for param in &md.params {
            let t = var_types.get(&param.0);
            let should_root = t.map(|ty| {
                if !needs_gc_root(ty) {
                    md.body.iter().any(|s| stmt_contains_non_primitive_usage(s, &param.0))
                } else {
                    true
                }
            }).unwrap_or(true);
            if should_root {
                self.out.push_str(&format!("    ep_gc_push_root(&{});\n", param.0));
                gc_root_count += 1;
            }
        }
        if gc_root_count > 0 {
            self.out.push_str("\n");
        }

        // GC safe point: collect only if this function uses heap-allocated data
        let needs_gc = gc_root_count > 0 || var_types.values().any(|t| 
            matches!(t, Type::List | Type::Map | Type::DynStr | Type::Struct(_) | Type::Enum(_) | Type::RefList)
        );
        if needs_gc {
            self.out.push_str("    ep_gc_maybe_collect();\n\n");
        }

        for stmt in &md.body {
            self.gen_statement(stmt, &var_types)?;
        }

        self.out.push_str("L_cleanup:\n");
        // Pop GC roots
        if gc_root_count > 0 {
            self.out.push_str(&format!("    ep_gc_pop_roots({});\n", gc_root_count));
        }
        self.out.push_str("    return ret_val;\n}\n\n");

        Ok(())
    }
}

const RUNTIME_HEADER_AND_SRC: &str = r#"#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#ifdef __wasm__
#define _SETJMP_H
typedef int jmp_buf[1];
#define setjmp(buf) (0)
#define longjmp(buf, val) abort()

// Mock pthreads for single-threaded WASM
typedef struct { int lock_state; } pthread_mutex_t;
typedef struct { int cond_state; } pthread_cond_t;
typedef struct { int rw_state; } pthread_rwlock_t;
typedef int pthread_t;
typedef int pthread_attr_t;
#define PTHREAD_MUTEX_INITIALIZER {0}
#define PTHREAD_COND_INITIALIZER {0}
#define PTHREAD_RWLOCK_INITIALIZER {0}
#define pthread_mutex_init(m, a) ((void)(a), (m)->lock_state = 0, 0)
#define pthread_mutex_lock(m) ((m)->lock_state = 1, 0)
#define pthread_mutex_unlock(m) ((m)->lock_state = 0, 0)
#define pthread_mutex_trylock(m) ((m)->lock_state == 0 ? ((m)->lock_state = 1, 0) : 1)
#define pthread_mutex_destroy(m) ((void)(m), 0)
#define pthread_cond_init(c, a) ((void)(a), (c)->cond_state = 0, 0)
#define pthread_cond_wait(c, m) ((void)(c), (void)(m), 0)
#define pthread_cond_signal(c) ((void)(c), 0)
#define pthread_cond_broadcast(c) ((void)(c), 0)
#define pthread_cond_destroy(c) ((void)(c), 0)
#define pthread_rwlock_init(r, a) ((void)(a), (r)->rw_state = 0, 0)
#define pthread_rwlock_rdlock(r) ((r)->rw_state = 1, 0)
#define pthread_rwlock_wrlock(r) ((r)->rw_state = 2, 0)
#define pthread_rwlock_unlock(r) ((r)->rw_state = 0, 0)
#define pthread_rwlock_destroy(r) ((void)(r), 0)
#define pthread_create(t, a, f, arg) ((void)(t), (void)(a), (void)(f), (void)(arg), 0)
#define pthread_join(t, r) ((void)(t), (void)(r), 0)
#define pthread_detach(t) ((void)(t), 0)
#else
#include <setjmp.h>
#endif
#include <signal.h>
#include <time.h>
#ifndef _WIN32
#include <unistd.h>
#endif
#if defined(__APPLE__)
#include <mach/mach.h>
#endif
#if defined(__linux__)
#include <sys/random.h>
#endif
#include <fcntl.h>

/* Cryptographically secure random bytes. Uses the OS CSPRNG: arc4random on
   Apple/BSD, getrandom(2) on Linux (falling back to /dev/urandom), and a
   /dev/urandom read elsewhere. Only if all of those are unavailable does it
   fall back to rand() — never on a supported platform. */
static void ep_secure_random_bytes(unsigned char* buf, size_t n) {
#if defined(__APPLE__) || defined(__FreeBSD__) || defined(__OpenBSD__) || defined(__NetBSD__)
    arc4random_buf(buf, n);
#else
    size_t got = 0;
  #if defined(__linux__)
    while (got < n) {
        ssize_t r = getrandom(buf + got, n - got, 0);
        if (r <= 0) break;
        got += (size_t)r;
    }
  #endif
    if (got < n) {
        FILE* f = fopen("/dev/urandom", "rb");
        if (f) {
            got += fread(buf + got, 1, n - got, f);
            fclose(f);
        }
    }
    while (got < n) {
        buf[got++] = (unsigned char)(rand() & 0xFF);
    }
#endif
}

/* Try/catch infrastructure */
static jmp_buf ep_try_buf;
static volatile int ep_try_active = 0;

static void ep_signal_handler(int sig) {
    if (ep_try_active) {
        ep_try_active = 0;
        longjmp(ep_try_buf, sig);
    }
    /* Outside try: print error and exit */
    const char* name = sig == SIGSEGV ? "segmentation fault (null pointer or invalid memory access)"
                     : sig == SIGFPE  ? "arithmetic error (division by zero)"
                     : sig == SIGABRT ? "aborted"
                     : "unknown signal";
    fprintf(stderr, "\nRuntime Error: %s (signal %d)\n", name, sig);

    /* Write to daemon/general log file if environment variable is set */
    const char* daemon_log = getenv("ERNOS_DAEMON_LOG");
    if (!daemon_log || daemon_log[0] == '\0') {
        daemon_log = getenv("ERNOS_LOG_FILE");
    }
    if (daemon_log && daemon_log[0] != '\0') {
        FILE* f = fopen(daemon_log, "ab");
        if (f) {
            time_t rawtime;
            time(&rawtime);
            struct tm * timeinfo = localtime(&rawtime);
            char time_buf[80];
            if (timeinfo) {
                strftime(time_buf, sizeof(time_buf), "%Y-%m-%d %H:%M:%S", timeinfo);
            } else {
                snprintf(time_buf, sizeof(time_buf), "%lld", (long long)rawtime);
            }
            fprintf(f, "[%s] FATAL: Runtime Error: %s (signal %d)\n", time_buf, name, sig);
            fclose(f);
        }
    }

    _exit(128 + sig);
}

#ifdef _MSC_VER
static void ep_install_signal_handlers(void);
#pragma section(".CRT$XCU", read)
__declspec(allocate(".CRT$XCU")) static void (*_ep_init_signals)(void) = ep_install_signal_handlers;
static void ep_install_signal_handlers(void) {
#else
__attribute__((constructor))
static void ep_install_signal_handlers(void) {
#endif
    signal(SIGFPE, ep_signal_handler);
    signal(SIGSEGV, ep_signal_handler);
    signal(SIGABRT, ep_signal_handler);
#ifdef _WIN32
    { WSADATA wsa; WSAStartup(MAKEWORD(2,2), &wsa); }
#endif
}

#if defined(__wasm__)
  typedef int ep_thread_t;
  typedef int ep_mutex_t;
  typedef int ep_cond_t;
  #define ep_mutex_init(m) (void)(0)
  #define ep_mutex_lock(m) (void)(0)
  #define ep_mutex_unlock(m) (void)(0)
  #define ep_cond_init(c) (void)(0)
  #define ep_cond_wait(c, m) (void)(0)
  #define ep_cond_signal(c) (void)(0)
#elif defined(_WIN32)
  #include <winsock2.h>
  #include <ws2tcpip.h>
  #include <windows.h>
  #pragma comment(lib, "ws2_32.lib")
  typedef HANDLE ep_thread_t;
  typedef CRITICAL_SECTION ep_mutex_t;
  typedef CONDITION_VARIABLE ep_cond_t;
  #define ep_mutex_init(m) InitializeCriticalSection(m)
  #define ep_mutex_lock(m) EnterCriticalSection(m)
  #define ep_mutex_unlock(m) LeaveCriticalSection(m)
  #define ep_cond_init(c) InitializeConditionVariable(c)
  #define ep_cond_wait(c, m) SleepConditionVariableCS(c, m, INFINITE)
  #define ep_cond_signal(c) WakeConditionVariable(c)
#else
  #include <sys/socket.h>
  #include <netinet/in.h>
  #include <arpa/inet.h>
  #include <unistd.h>
  #include <netdb.h>
  #include <fcntl.h>
  #include <errno.h>
  #include <sys/select.h>
  #include <pthread.h>
  typedef pthread_t ep_thread_t;
  typedef pthread_mutex_t ep_mutex_t;
  typedef pthread_cond_t ep_cond_t;
  #define ep_mutex_init(m) pthread_mutex_init(m, NULL)
  #define ep_mutex_lock(m) pthread_mutex_lock(m)
  #define ep_mutex_unlock(m) pthread_mutex_unlock(m)
  #define ep_cond_init(c) pthread_cond_init(c, NULL)
  #define ep_cond_wait(c, m) pthread_cond_wait(c, m)
  #define ep_cond_signal(c) pthread_cond_signal(c)
#endif

/* ========== Ernos Mark-and-Sweep Garbage Collector ========== */

#include <setjmp.h>
#if !defined(__wasm__) && !defined(_WIN32)
#include <pthread.h>
#endif

typedef enum {
    EP_OBJ_LIST,
    EP_OBJ_STRING,
    EP_OBJ_STRUCT,
    EP_OBJ_CLOSURE,
    EP_OBJ_MAP
} EpObjKind;

typedef struct EpGCObject {
    EpObjKind kind;
    int marked;
    void* ptr;                /* actual allocation pointer */
    long long size;           /* payload size for structs */
    long long num_fields;     /* number of fields for structs (each is long long) */
    int generation;           /* 0 = Nursery/young, 1 = Old */
    struct EpGCObject* next;  /* intrusive linked list */
} EpGCObject;

long long ep_time_now_ms(void);
long long ep_sleep_ms(long long ms);

typedef struct EpTask EpTask;
typedef struct {
    long long chan;
    int completed;
    long long value;
    EpTask* waiting_task;
} EpFuture;

static long long ep_await_future(EpFuture* fut);

struct EpTask {
    long long (*step)(void*); /* pointer to step function */
    void* args;               /* pointer to step state arguments */
    long long args_size_bytes; /* size of args struct for GC tracing */
    EpTask* next;             /* run-queue link pointer */
    EpFuture* fut;            /* future associated with this task */
    int state;                /* coroutine execution state */
    int is_cancelled;         /* cancellation flag for structured concurrency */
    struct EpTask* parent;    /* parent task for structured concurrency cancellation */
};

/* Event Loop Scheduler Globals & Functions */
static EpTask* ep_run_queue_head = NULL;
static EpTask* ep_run_queue_tail = NULL;
static EpTask* ep_current_task = NULL;
static int ep_event_loop_fd = -1; /* epoll or kqueue fd */
static int ep_active_io_sources = 0;

static void ep_task_enqueue(EpTask* task) {
    if (!task) return;
    task->next = NULL;
    if (ep_run_queue_tail) {
        ep_run_queue_tail->next = task;
        ep_run_queue_tail = task;
    } else {
        ep_run_queue_head = ep_run_queue_tail = task;
    }
}

static EpTask* ep_task_dequeue(void) {
    if (!ep_run_queue_head) return NULL;
    EpTask* task = ep_run_queue_head;
    ep_run_queue_head = ep_run_queue_head->next;
    if (!ep_run_queue_head) ep_run_queue_tail = NULL;
    return task;
}

#ifndef __wasm__
#ifdef __APPLE__
#include <sys/event.h>
#else
#include <sys/epoll.h>
#endif
#endif

static void ep_async_loop_init(void) {
    if (ep_event_loop_fd != -1) return;
#ifdef __wasm__
    ep_event_loop_fd = 999;
#elif defined(__APPLE__)
    ep_event_loop_fd = kqueue();
#else
    ep_event_loop_fd = epoll_create1(0);
#endif
}

typedef struct EpTimer {
    long long expiry_ms;
    EpTask* task;
    struct EpTimer* next;
} EpTimer;
static EpTimer* ep_timers_head = NULL;

static void ep_async_register_timer(long long timeout_ms, EpTask* task) {
    long long expiry = ep_time_now_ms() + timeout_ms;
    EpTimer* timer = (EpTimer*)malloc(sizeof(EpTimer));
    timer->expiry_ms = expiry;
    timer->task = task;
    timer->next = NULL;

    /* Insert sorted */
    if (!ep_timers_head || expiry < ep_timers_head->expiry_ms) {
        timer->next = ep_timers_head;
        ep_timers_head = timer;
    } else {
        EpTimer* cur = ep_timers_head;
        while (cur->next && cur->next->expiry_ms <= expiry) {
            cur = cur->next;
        }
        timer->next = cur->next;
        cur->next = timer;
    }
}

static long long ep_get_next_timer_timeout(void) {
    if (!ep_timers_head) return -1; /* block indefinitely */
    long long now = ep_time_now_ms();
    long long diff = ep_timers_head->expiry_ms - now;
    return diff < 0 ? 0 : diff;
}

static void ep_process_expired_timers(void) {
    long long now = ep_time_now_ms();
    while (ep_timers_head && ep_timers_head->expiry_ms <= now) {
        EpTimer* expired = ep_timers_head;
        ep_timers_head = ep_timers_head->next;
        ep_task_enqueue(expired->task);
        free(expired);
    }
}

static void ep_async_register_read(int fd, EpTask* task) {
#ifdef __wasm__
    (void)fd;
    (void)task;
#else
    ep_async_loop_init();
    ep_active_io_sources++;
#ifdef __APPLE__
    struct kevent ev;
    EV_SET(&ev, fd, EVFILT_READ, EV_ADD | EV_ONESHOT, 0, 0, task);
    kevent(ep_event_loop_fd, &ev, 1, NULL, 0, NULL);
#else
    struct epoll_event ev;
    ev.events = EPOLLIN | EPOLLONESHOT;
    ev.data.ptr = task;
    if (epoll_ctl(ep_event_loop_fd, EPOLL_CTL_ADD, fd, &ev) < 0) {
        epoll_ctl(ep_event_loop_fd, EPOLL_CTL_MOD, fd, &ev);
    }
#endif
#endif
}

static void ep_async_wait_step(long long timeout) {
#ifdef __wasm__
    if (timeout > 0) {
        ep_sleep_ms(timeout);
    }
#else
#ifdef __APPLE__
    struct kevent events[16];
    struct timespec ts;
    struct timespec* p_ts = NULL;
    if (timeout >= 0) {
        ts.tv_sec = timeout / 1000;
        ts.tv_nsec = (timeout % 1000) * 1000000;
        p_ts = &ts;
    }
    int n = kevent(ep_event_loop_fd, NULL, 0, events, 16, p_ts);
    for (int i = 0; i < n; i++) {
        EpTask* t = (EpTask*)events[i].udata;
        ep_task_enqueue(t);
        ep_active_io_sources--;
    }
#else
    struct epoll_event events[16];
    int n = epoll_wait(ep_event_loop_fd, events, 16, (int)timeout);
    for (int i = 0; i < n; i++) {
        EpTask* t = (EpTask*)events[i].data.ptr;
        ep_task_enqueue(t);
        ep_active_io_sources--;
    }
#endif
#endif
    ep_process_expired_timers();
}

static void ep_async_loop_run(void) {
    ep_async_loop_init();
    while (ep_run_queue_head || ep_timers_head || ep_active_io_sources > 0) {
        /* 1. Run all runnable tasks */
        while (ep_run_queue_head) {
            EpTask* task = ep_task_dequeue();
            if (task->is_cancelled) {
                if (task->fut) {
                    task->fut->completed = 1;
                    task->fut->value = -1;
                }
                free(task->args);
                free(task);
                continue;
            }
            ep_current_task = task;
            long long res = task->step(task->args);
            ep_current_task = NULL;
            if (res != -999999) {
                if (task->fut) {
                    task->fut->value = res;
                    task->fut->completed = 1;
                    if (task->fut->waiting_task) {
                        ep_task_enqueue(task->fut->waiting_task);
                        task->fut->waiting_task = NULL;
                    }
                }
                free(task->args);
                free(task);
            }
        }

        /* 2. If no tasks runnable, wait for I/O / timers */
        if (!ep_run_queue_head) {
            long long timeout = ep_get_next_timer_timeout();
            if (timeout == -1 && !ep_timers_head && ep_active_io_sources == 0) {
                break;
            }

            if (ep_event_loop_fd == -1) {
                if (timeout > 0) {
                    ep_sleep_ms(timeout);
                }
                ep_process_expired_timers();
                continue;
            }

            ep_async_wait_step(timeout);
        }
    }
}

static long long ep_await_future(EpFuture* fut) {
    if (!fut) return 0;
    while (!fut->completed) {
        if (ep_run_queue_head) {
            EpTask* task = ep_task_dequeue();
            if (task) {
                if (task->is_cancelled) {
                    if (task->fut) {
                        task->fut->completed = 1;
                        task->fut->value = -1;
                    }
                    free(task->args);
                    free(task);
                } else {
                    EpTask* saved_current = ep_current_task;
                    ep_current_task = task;
                    long long res = task->step(task->args);
                    ep_current_task = saved_current;
                    if (res != -999999) {
                        if (task->fut) {
                            task->fut->value = res;
                            task->fut->completed = 1;
                            if (task->fut->waiting_task) {
                                ep_task_enqueue(task->fut->waiting_task);
                                task->fut->waiting_task = NULL;
                            }
                        }
                        free(task->args);
                        free(task);
                    }
                }
            }
        } else {
            long long timeout = ep_get_next_timer_timeout();
            if (timeout == -1 && !ep_timers_head && ep_active_io_sources == 0) {
                fprintf(stderr, "Deadlock detected: awaiting incomplete future with no active tasks or timers.\n");
                exit(1);
            }
            if (ep_event_loop_fd == -1) {
                if (timeout > 0) {
                    ep_sleep_ms(timeout);
                }
                ep_process_expired_timers();
            } else {
                ep_async_wait_step(timeout);
            }
        }
    }
    return fut->value;
}

static EpGCObject* ep_gc_register(void* ptr, EpObjKind kind);
long long create_list(void);
long long append_list(long long list_ptr, long long value);

typedef struct {
    EpFuture* futures[128];
    int count;
    int has_error;
} EpTaskGroup;

typedef struct {
    EpFuture* fut;
    int timer_fired;
} EpTimeoutArgs;

static EpTask* ep_find_task_by_future(EpFuture* fut) {
    if (!fut) return NULL;
    EpTask* cur = ep_run_queue_head;
    while (cur) {
        if (cur->fut == fut) return cur;
        cur = cur->next;
    }
    EpTimer* timer = ep_timers_head;
    while (timer) {
        if (timer->task && timer->task->fut == fut) return timer->task;
        timer = timer->next;
    }
    return NULL;
}

static void ep_cancel_task(EpTask* task) {
    if (!task) return;
    task->is_cancelled = 1;
    if (task->fut) {
        task->fut->completed = 1;
        task->fut->value = -1;
    }
    // Cancel children in run queue
    EpTask* cur = ep_run_queue_head;
    while (cur) {
        if (cur->parent == task) {
            ep_cancel_task(cur);
        }
        cur = cur->next;
    }
    // Cancel children in timers queue
    EpTimer* timer = ep_timers_head;
    while (timer) {
        if (timer->task && timer->task->parent == task) {
            ep_cancel_task(timer->task);
        }
        timer = timer->next;
    }
}

static long long create_task_group(void) {
    EpTaskGroup* tg = (EpTaskGroup*)calloc(1, sizeof(EpTaskGroup));
    tg->count = 0;
    tg->has_error = 0;
    { EpGCObject* _go = ep_gc_register(tg, EP_OBJ_STRUCT); if(_go) _go->num_fields = 0; }
    return (long long)tg;
}

static long long add_task_group(long long group_ptr, long long fut_ptr) {
    EpTaskGroup* tg = (EpTaskGroup*)group_ptr;
    EpFuture* fut = (EpFuture*)fut_ptr;
    if (!tg || !fut) return 0;
    if (tg->count < 128) {
        tg->futures[tg->count++] = fut;
        // Associate the task's parent with the current task so it's cancellation-linked
        EpTask* task = ep_find_task_by_future(fut);
        if (task) {
            task->parent = ep_current_task;
        }
    }
    return 0;
}

static long long wait_task_group(long long group_ptr) {
    EpTaskGroup* tg = (EpTaskGroup*)group_ptr;
    if (!tg) return 0;
    
    int all_done = 0;
    while (!all_done) {
        all_done = 1;
        for (int i = 0; i < tg->count; i++) {
            EpFuture* fut = tg->futures[i];
            if (!fut->completed) {
                all_done = 0;
                break;
            }
        }
        
        if (all_done) break;
        
        if (ep_run_queue_head) {
            EpTask* task = ep_task_dequeue();
            if (task) {
                if (task->is_cancelled) {
                    if (task->fut) {
                        task->fut->completed = 1;
                        task->fut->value = -1;
                    }
                    free(task->args);
                    free(task);
                } else {
                    EpTask* saved_current = ep_current_task;
                    ep_current_task = task;
                    long long res = task->step(task->args);
                    ep_current_task = saved_current;
                    if (res != -999999) {
                        if (task->fut) {
                            task->fut->value = res;
                            task->fut->completed = 1;
                            if (task->fut->waiting_task) {
                                ep_task_enqueue(task->fut->waiting_task);
                                task->fut->waiting_task = NULL;
                            }
                        }
                        free(task->args);
                        free(task);
                    }
                }
            }
        } else {
            long long timeout = ep_get_next_timer_timeout();
            if (timeout == -1 && !ep_timers_head && ep_active_io_sources == 0) {
                fprintf(stderr, "Deadlock detected: waiting on task group with no active tasks or timers.\n");
                exit(1);
            }
            if (ep_event_loop_fd == -1) {
                if (timeout > 0) {
                    ep_sleep_ms(timeout);
                }
                ep_process_expired_timers();
            } else {
                ep_async_wait_step(timeout);
            }
        }
        
        // Propagate cancellation/failure inside task group
        for (int i = 0; i < tg->count; i++) {
            EpFuture* fut = tg->futures[i];
            if (fut->completed && fut->value == -1) {
                tg->has_error = 1;
                for (int j = 0; j < tg->count; j++) {
                    EpFuture* other_fut = tg->futures[j];
                    if (!other_fut->completed) {
                        EpTask* other_task = ep_find_task_by_future(other_fut);
                        if (other_task) {
                            ep_cancel_task(other_task);
                        } else {
                            other_fut->completed = 1;
                            other_fut->value = -1;
                        }
                    }
                }
            }
        }
    }
    
    long long list = create_list();
    for (int i = 0; i < tg->count; i++) {
        append_list(list, tg->futures[i]->value);
    }
    return list;
}

static long long ep_timeout_timer_step(void* r) {
    EpTimeoutArgs* args = (EpTimeoutArgs*)r;
    if (args && args->fut && !args->fut->completed) {
        args->timer_fired = 1;
        EpTask* task = ep_find_task_by_future(args->fut);
        if (task) {
            ep_cancel_task(task);
        } else {
            args->fut->completed = 1;
            args->fut->value = -1;
        }
    }
    return 0;
}

static long long async_timeout(long long timeout_ms, long long fut_ptr) {
    EpFuture* fut = (EpFuture*)fut_ptr;
    if (!fut) return -1;
    if (fut->completed) return fut->value;
    
    EpTimeoutArgs* args = (EpTimeoutArgs*)malloc(sizeof(EpTimeoutArgs));
    args->fut = fut;
    args->timer_fired = 0;
    
    EpTask* timer_task = (EpTask*)malloc(sizeof(EpTask));
    timer_task->step = ep_timeout_timer_step;
    timer_task->args = args;
    timer_task->args_size_bytes = sizeof(EpTimeoutArgs);
    timer_task->fut = NULL;
    timer_task->state = 0;
    timer_task->is_cancelled = 0;
    timer_task->parent = NULL;
    
    ep_async_register_timer(timeout_ms, timer_task);
    
    while (!fut->completed && !(args->timer_fired)) {
        if (ep_run_queue_head) {
            EpTask* task = ep_task_dequeue();
            if (task) {
                if (task->is_cancelled) {
                    if (task->fut) {
                        task->fut->completed = 1;
                        task->fut->value = -1;
                    }
                    free(task->args);
                    free(task);
                } else {
                    EpTask* saved_current = ep_current_task;
                    ep_current_task = task;
                    long long res = task->step(task->args);
                    ep_current_task = saved_current;
                    if (res != -999999) {
                        if (task->fut) {
                            task->fut->value = res;
                            task->fut->completed = 1;
                            if (task->fut->waiting_task) {
                                ep_task_enqueue(task->fut->waiting_task);
                                task->fut->waiting_task = NULL;
                            }
                        }
                        free(task->args);
                        free(task);
                    }
                }
            }
        } else {
            long long timeout = ep_get_next_timer_timeout();
            if (timeout == -1 && !ep_timers_head && ep_active_io_sources == 0) {
                break;
            }
            if (ep_event_loop_fd == -1) {
                if (timeout > 0) {
                    ep_sleep_ms(timeout);
                }
                ep_process_expired_timers();
            } else {
                ep_async_wait_step(timeout);
            }
        }
    }
    
    return fut->value;
}

/* ── Awaitable async socket-readability ─────────────────────────────────────
   `await async_wait_readable(fd)` suspends the calling async task until `fd` is
   readable, letting the event loop run other tasks (e.g. another agent waiting on
   its own LLM socket) meanwhile. Mirrors sleep_ms: build a future, register a
   oneshot read-readiness task with the loop, return the future. When fd becomes
   readable, ep_async_wait_step re-enqueues the task; its step completes the future
   and wakes whoever awaited it. This is what lets I/O-bound agents run concurrently
   on ONE thread — no OS threads, no shared-heap GC race. */
typedef struct { EpFuture* fut; } EpReadReadyArgs;
static long long ep_read_ready_step(void* r) {
    EpReadReadyArgs* args = (EpReadReadyArgs*)r;
    if (args && args->fut) {
        args->fut->completed = 1;
        args->fut->value = 1;
        if (args->fut->waiting_task) {
            ep_task_enqueue(args->fut->waiting_task);
            args->fut->waiting_task = NULL;
        }
    }
    return 0;
}
long long async_wait_readable(long long fd) {
    EpFuture* fut = (EpFuture*)malloc(sizeof(EpFuture));
    fut->completed = 0;
    fut->value = 0;
    fut->waiting_task = NULL;
    fut->chan = 0;
    { EpGCObject* _go = ep_gc_register(fut, EP_OBJ_STRUCT); if(_go) _go->num_fields = 3; }
    EpReadReadyArgs* args = (EpReadReadyArgs*)malloc(sizeof(EpReadReadyArgs));
    args->fut = fut;
    EpTask* task = (EpTask*)malloc(sizeof(EpTask));
    task->step = ep_read_ready_step;
    task->args = args;
    task->args_size_bytes = sizeof(EpReadReadyArgs);
    task->fut = NULL;
    task->state = 0;
    task->is_cancelled = 0;
    task->parent = ep_current_task;
    ep_async_register_read((int)fd, task);
    return (long long)fut;
}

typedef struct {
    EpFuture* fut;
} EpSleepTimerArgs;

static long long ep_sleep_timer_step(void* r) {
    EpSleepTimerArgs* args = (EpSleepTimerArgs*)r;
    if (args && args->fut) {
        args->fut->completed = 1;
        args->fut->value = 0;
        if (args->fut->waiting_task) {
            ep_task_enqueue(args->fut->waiting_task);
            args->fut->waiting_task = NULL;
        }
    }
    return 0;
}

static long long sleep_ms(long long ms) {
    EpFuture* fut = (EpFuture*)malloc(sizeof(EpFuture));
    fut->completed = 0;
    fut->value = 0;
    fut->waiting_task = NULL;
    fut->chan = 0;
    { EpGCObject* _go = ep_gc_register(fut, EP_OBJ_STRUCT); if(_go) _go->num_fields = 3; }
    
    EpSleepTimerArgs* args = (EpSleepTimerArgs*)malloc(sizeof(EpSleepTimerArgs));
    args->fut = fut;
    
    EpTask* task = (EpTask*)malloc(sizeof(EpTask));
    task->step = ep_sleep_timer_step;
    task->args = args;
    task->args_size_bytes = sizeof(EpSleepTimerArgs);
    task->fut = NULL;
    task->state = 0;
    task->is_cancelled = 0;
    task->parent = ep_current_task;
    
    ep_async_register_timer(ms, task);
    return (long long)fut;
}

static long long cancel_task(long long fut_ptr) {
    EpFuture* fut = (EpFuture*)fut_ptr;
    if (fut) {
        EpTask* task = ep_find_task_by_future(fut);
        if (task) {
            ep_cancel_task(task);
        } else {
            fut->completed = 1;
            fut->value = -1;
        }
    }
    return 0;
}

/* Closure environment — captures travel with the function pointer */
#define EP_CLOSURE_MAGIC 0x4550434C4FL
typedef struct {
    long long magic;
    long long fn_ptr;
    long long env[];  /* flexible array of captured values */
} EpClosure;

/* GC globals */
static EpGCObject* ep_gc_head = NULL;
static long long ep_gc_count = 0;
static long long ep_gc_threshold = 4096;
static int ep_gc_enabled = 1;
static long long ep_gc_nursery_count = 0;
static long long ep_gc_nursery_threshold = 512;
static int ep_gc_minor_count = 0;
static int ep_gc_major_count = 0;
static void** ep_gc_remembered_set = NULL;
static long long ep_gc_remembered_cap = 0;
static long long ep_gc_remembered_size = 0;
/* Single mutex for ALL GC and thread registry operations.
   Previous design had two mutexes (ep_gc_mutex + ep_thread_registry_mutex)
   which caused deadlock under concurrent channel load: thread A held gc_mutex
   and waited for registry_mutex, thread B held registry_mutex and waited for
   gc_mutex. Single lock eliminates the ordering problem. */
#ifdef __wasm__
#define __thread
#endif
static pthread_mutex_t ep_gc_mutex = PTHREAD_MUTEX_INITIALIZER;

/* Stop-the-world coordination. The collector sets ep_gc_stop_requested and, in
   ep_gc_stop_the_world(), waits until every *other* registered thread has parked
   at a safepoint (ep_gc_park_if_stopped). This guarantees mark/sweep never runs
   concurrently with a mutator changing its roots or an object's fields — the
   "marking races with running mutators" hazard. All three fields are touched
   only while holding ep_gc_mutex (the lock-free reads of ep_gc_stop_requested at
   safepoints are a benign optimization: a missed set just defers parking to the
   next safepoint, and the collector's bounded wait covers it). */
static volatile int ep_gc_stop_requested = 0;
static int ep_gc_parked_count = 0;
static pthread_cond_t ep_gc_resume_cond = PTHREAD_COND_INITIALIZER;

/* Function pointer for channel scanning — set after EpChannel is defined.
   GC mark calls this to scan values in-transit in channel buffers. */
static void (*ep_gc_scan_channels_major)(void) = NULL;
static void (*ep_gc_scan_channels_minor)(void) = NULL;
/* Function pointers for marking top-level constant/global variables, which are
   GC roots that live outside any function frame. Set by __ep_init_constants. */
static void (*ep_gc_mark_globals_major)(void) = NULL;
static void (*ep_gc_mark_globals_minor)(void) = NULL;
/* Function pointers for map value traversal — set after EpMap is defined.
   GC mark calls these to recursively mark values stored in maps. */
static void (*ep_gc_mark_map_values)(void* ptr) = NULL;
static void (*ep_gc_mark_map_values_minor)(void* ptr) = NULL;

/* Thread registry for GC root scanning in multi-threaded environment */
#define EP_MAX_THREADS 256
static __thread void* volatile ep_thread_local_top = NULL;
static __thread void* ep_thread_local_bottom = NULL;

static void* volatile* ep_thread_tops[EP_MAX_THREADS];
static void* ep_thread_bottoms[EP_MAX_THREADS];
static volatile int ep_thread_active[EP_MAX_THREADS];
static int ep_num_threads = 0;

/* Per-thread GC root state — heap-allocated, stable across thread lifetime.
   Previous design stored raw pointers to __thread arrays (ep_gc_root_stack,
   ep_gc_root_sp) in the global registry. When a thread exited, the __thread
   storage was freed, leaving dangling pointers that ep_gc_mark would
   dereference → segfault. Now each thread gets a heap-allocated state struct
   that survives thread exit and is only recycled when the slot is reused. */
typedef struct {
    long long* roots[4096];  /* copy of root pointers, updated under lock */
    volatile int sp;         /* current root stack pointer */
} EpThreadGCState;

static EpThreadGCState* ep_thread_gc_states[EP_MAX_THREADS];

/* Shadow stack for explicit GC roots — thread-local to prevent cross-thread corruption */
#define EP_GC_MAX_ROOTS 4096
static __thread long long* ep_gc_root_stack[EP_GC_MAX_ROOTS];
static __thread int ep_gc_root_sp = 0;
static __thread int ep_thread_slot = -1;

/* ep_gc_root_sp is the *logical* shadow-stack depth. It always advances on
   push and retreats on pop so that per-frame push/pop counts stay balanced.
   Array storage is capped at EP_GC_MAX_ROOTS: once the stack is full, further
   roots are counted but not stored (those deep-overflow locals are simply not
   traced) — crucially, we never overwrite or drop an outer frame's stored
   roots, which the old "silently skip the push but still pop" path did. */
static void ep_gc_push_root(long long* root) {
    int idx = ep_gc_root_sp;
    ep_gc_root_sp++;
    if (idx < EP_GC_MAX_ROOTS) {
        ep_gc_root_stack[idx] = root;
        /* Update the heap-allocated state so GC mark can see it safely */
        if (ep_thread_slot >= 0 && ep_thread_gc_states[ep_thread_slot]) {
            ep_thread_gc_states[ep_thread_slot]->roots[idx] = root;
            ep_thread_gc_states[ep_thread_slot]->sp =
                (ep_gc_root_sp < EP_GC_MAX_ROOTS) ? ep_gc_root_sp : EP_GC_MAX_ROOTS;
        }
    }
}
static void ep_gc_pop_roots(long long count) {
    ep_gc_root_sp -= (int)count;
    if (ep_gc_root_sp < 0) ep_gc_root_sp = 0;
    /* Update the heap-allocated state (clamped to the array bound) */
    if (ep_thread_slot >= 0 && ep_thread_gc_states[ep_thread_slot]) {
        ep_thread_gc_states[ep_thread_slot]->sp =
            (ep_gc_root_sp < EP_GC_MAX_ROOTS) ? ep_gc_root_sp : EP_GC_MAX_ROOTS;
    }
}

/* Park the calling thread if the collector has stopped the world.
   MUST be called with ep_gc_mutex held. The thread's shadow stack (its precise
   root set) is stable while parked, so the collector can scan it race-free. */
static void ep_gc_park_if_stopped(void) {
    if (!ep_gc_stop_requested) return;
    /* Spill registers onto the stack and publish this thread's current stack top
       so the collector can conservatively scan its frozen C stack while parked —
       this catches roots held only in registers/temporaries that the precise
       shadow stack does not yet record. _dummy is declared below _pregs, so its
       (lower) address bounds a scan range that covers the spilled registers. */
    jmp_buf _pregs;
    volatile char _top_marker;  /* function-scope: stays valid while parked */
    memset(&_pregs, 0, sizeof(_pregs));
    setjmp(_pregs);
    /* _top_marker is declared after _pregs, so its (lower) address bounds a scan
       range [&_top_marker, stack_bottom] that covers the spilled registers. */
    ep_thread_local_top = (void*)&_top_marker;
    __sync_synchronize();  /* publish shadow-stack + top writes before parking */
    ep_gc_parked_count++;
    while (ep_gc_stop_requested) {
        pthread_cond_wait(&ep_gc_resume_cond, &ep_gc_mutex);
    }
    ep_gc_parked_count--;
}

/* Begin a stop-the-world pause. MUST be called with ep_gc_mutex held.
   Waits (briefly releasing the lock so blocked mutators can reach a safepoint)
   until all other registered threads have parked. After a bounded fallback
   (~50ms) it proceeds anyway: any thread that hasn't parked by then is blocked
   or idle with a stable shadow stack, so scanning it is still safe in practice. */
static void ep_gc_stop_the_world(void) {
    ep_gc_stop_requested = 1;
    /* Actively-running threads reach a safepoint (every allocation and every
       function entry) within microseconds, so they park on the first spin or
       two. The bound only caps the rare case where a thread is blocked/idle
       (e.g. just entered a channel op) and won't park — those have a stable
       shadow stack, so proceeding to scan them is safe. ~40 * 250us ≈ 10ms. */
    for (int spins = 0; spins < 40; spins++) {
        int others = 0;
        for (int t = 0; t < ep_num_threads; t++) {
            if (ep_thread_active[t] && t != ep_thread_slot) others++;
        }
        if (others <= 0 || ep_gc_parked_count >= others) return;
        pthread_mutex_unlock(&ep_gc_mutex);
#ifdef _WIN32
        Sleep(1);
#elif !defined(__wasm__)
        usleep(250);
#endif
        pthread_mutex_lock(&ep_gc_mutex);
    }
}

/* End a stop-the-world pause and wake all parked threads. MUST hold ep_gc_mutex. */
static void ep_gc_start_the_world(void) {
    ep_gc_stop_requested = 0;
    pthread_cond_broadcast(&ep_gc_resume_cond);
}

static void ep_gc_register_thread(void* stack_bottom) {
    ep_thread_local_bottom = stack_bottom;
    ep_thread_local_top = stack_bottom;
    
    pthread_mutex_lock(&ep_gc_mutex);
    int slot = -1;
    for (int i = 0; i < ep_num_threads; i++) {
        if (!ep_thread_active[i]) {
            slot = i;
            break;
        }
    }
    if (slot == -1 && ep_num_threads < EP_MAX_THREADS) {
        slot = ep_num_threads++;
    }
    if (slot != -1) {
        ep_thread_tops[slot] = &ep_thread_local_top;
        ep_thread_bottoms[slot] = stack_bottom;
        /* Allocate or reuse heap state for this slot */
        if (!ep_thread_gc_states[slot]) {
            ep_thread_gc_states[slot] = (EpThreadGCState*)calloc(1, sizeof(EpThreadGCState));
        }
        ep_thread_gc_states[slot]->sp = 0;
        ep_thread_slot = slot;
        __sync_synchronize();  /* Memory barrier: state must be visible before active */
        ep_thread_active[slot] = 1;
    }
    pthread_mutex_unlock(&ep_gc_mutex);
}

static void ep_gc_unregister_thread(void) {
    pthread_mutex_lock(&ep_gc_mutex);
    for (int i = 0; i < ep_num_threads; i++) {
        if (ep_thread_active[i] && ep_thread_tops[i] == &ep_thread_local_top) {
            /* Zero root count FIRST — even if ep_gc_mark races past the
               active check, it will see sp=0 and walk no roots instead
               of dereferencing stale __thread pointers */
            if (ep_thread_gc_states[i]) {
                ep_thread_gc_states[i]->sp = 0;
            }
            __sync_synchronize();  /* Memory barrier: sp=0 visible before deactivation */
            ep_thread_active[i] = 0;
            ep_thread_slot = -1;
            break;
        }
    }
    pthread_mutex_unlock(&ep_gc_mutex);
}

#define EP_GC_UPDATE_TOP() { volatile int _dummy; ep_thread_local_top = (void*)&_dummy; }

/* Simple open-addressed hash map with linear probing for O(1) GC object lookup */
typedef struct {
    void* key;
    EpGCObject* value;
} EpGCEntry;

static EpGCEntry* ep_gc_table = NULL;
static long long ep_gc_table_cap = 0;
static long long ep_gc_table_size = 0;

/* Bucket index for a pointer key. The previous hash was ((uintptr_t)key % cap)
   with cap a power of two; malloc returns 16-byte-aligned pointers, so the low 4
   bits are always 0 and only every 16th bucket was ever a home slot. That caused
   catastrophic primary clustering -> O(n) probe runs -> ep_gc_table_remove's
   rehash became O(n^2), which (under the single global GC mutex) wedged the whole
   node when a large object list was freed. A splitmix64 finalizer avalanches all
   bits, so even the low bits taken by the (cap-1) mask are well distributed. */
static inline long long ep_gc_index(void* key, long long cap) {
    uint64_t z = (uint64_t)(uintptr_t)key;
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
    z = z ^ (z >> 31);
    return (long long)(z & (uint64_t)(cap - 1));   /* cap is always a power of two */
}

/* Insert without growing — assumes a free slot exists. Used by the resize and by
   ep_gc_table_remove's rehash, neither of which may trigger a (re)allocation of
   the table mid-iteration. */
static void ep_gc_table_place(void* key, EpGCObject* value) {
    long long idx = ep_gc_index(key, ep_gc_table_cap);
    while (ep_gc_table[idx].key != NULL) {
        if (ep_gc_table[idx].key == key) {
            ep_gc_table[idx].value = value;
            return;
        }
        idx = (idx + 1) & (ep_gc_table_cap - 1);
    }
    ep_gc_table[idx].key = key;
    ep_gc_table[idx].value = value;
    ep_gc_table_size++;
}

static void ep_gc_table_insert(void* key, EpGCObject* value) {
    if (ep_gc_table_size * 2 >= ep_gc_table_cap) {
        long long old_cap = ep_gc_table_cap;
        long long new_cap = old_cap == 0 ? 512 : old_cap * 2;
        EpGCEntry* new_table = (EpGCEntry*)calloc(new_cap, sizeof(EpGCEntry));
        EpGCEntry* old_table = ep_gc_table;
        ep_gc_table = new_table;
        ep_gc_table_cap = new_cap;
        ep_gc_table_size = 0;
        for (long long i = 0; i < old_cap; i++) {
            if (old_table[i].key != NULL) {
                ep_gc_table_place(old_table[i].key, old_table[i].value);
            }
        }
        free(old_table);
    }
    ep_gc_table_place(key, value);
}

static EpGCObject* ep_gc_table_get(void* key) {
    if (ep_gc_table_cap == 0) return NULL;
    long long idx = ep_gc_index(key, ep_gc_table_cap);
    while (ep_gc_table[idx].key != NULL) {
        if (ep_gc_table[idx].key == key) return ep_gc_table[idx].value;
        idx = (idx + 1) & (ep_gc_table_cap - 1);
    }
    return NULL;
}

static void ep_gc_table_remove(void* key) {
    if (ep_gc_table_cap == 0) return;
    long long idx = ep_gc_index(key, ep_gc_table_cap);
    while (ep_gc_table[idx].key != NULL) {
        if (ep_gc_table[idx].key == key) {
            ep_gc_table[idx].key = NULL;
            ep_gc_table[idx].value = NULL;
            ep_gc_table_size--;
            /* Backward-shift rehash of the rest of this cluster. Re-place (no
               resize: size is not growing) so a mid-iteration realloc can never
               free the table out from under this loop. */
            long long next_idx = (idx + 1) & (ep_gc_table_cap - 1);
            while (ep_gc_table[next_idx].key != NULL) {
                void* rehash_key = ep_gc_table[next_idx].key;
                EpGCObject* rehash_val = ep_gc_table[next_idx].value;
                ep_gc_table[next_idx].key = NULL;
                ep_gc_table[next_idx].value = NULL;
                ep_gc_table_size--;
                ep_gc_table_place(rehash_key, rehash_val);
                next_idx = (next_idx + 1) & (ep_gc_table_cap - 1);
            }
            return;
        }
        idx = (idx + 1) & (ep_gc_table_cap - 1);
    }
}



/* Register a new GC object */
static EpGCObject* ep_gc_register(void* ptr, EpObjKind kind) {
    if (!ptr) return NULL;
    pthread_mutex_lock(&ep_gc_mutex);
    ep_gc_park_if_stopped();  /* safepoint: don't allocate/touch the table mid-collection */
    EpGCObject* obj = (EpGCObject*)malloc(sizeof(EpGCObject));
    if (!obj) {
        pthread_mutex_unlock(&ep_gc_mutex);
        return NULL;
    }
    obj->kind = kind;
    obj->marked = 0;
    obj->ptr = ptr;
    obj->size = 0;
    obj->num_fields = 0;
    obj->generation = 0;
    obj->next = ep_gc_head;
    ep_gc_head = obj;
    ep_gc_count++;
    ep_gc_nursery_count++;
    ep_gc_table_insert(ptr, obj);
    pthread_mutex_unlock(&ep_gc_mutex);
    return obj;
}

/* Find GC object by pointer.
   Takes ep_gc_mutex because ep_gc_table_insert may realloc+free the table
   concurrently (from another thread's allocation). Mutator-side callers
   (write barrier, free_struct/free_map/free_list, to-string) must use this
   locking variant; code already holding the mutex (mark/sweep) calls
   ep_gc_table_get directly to avoid a non-recursive double-lock deadlock. */
static EpGCObject* ep_gc_find(void* ptr) {
    pthread_mutex_lock(&ep_gc_mutex);
    ep_gc_park_if_stopped();  /* safepoint */
    EpGCObject* obj = ep_gc_table_get(ptr);
    pthread_mutex_unlock(&ep_gc_mutex);
    return obj;
}

/* Write barrier for generational GC: tracks references from old objects (gen 1) to young objects (gen 0).
   The whole operation runs under ep_gc_mutex so the table lookups and the
   remembered-set update see a consistent table (no race with a concurrent
   resize) and use the no-lock ep_gc_table_get to avoid re-entering the lock. */
static void ep_gc_write_barrier(void* host_ptr, long long val) {
    if (val == 0) return;
    pthread_mutex_lock(&ep_gc_mutex);
    ep_gc_park_if_stopped();  /* safepoint: don't update the remembered set mid-collection */
    EpGCObject* host_obj = ep_gc_table_get(host_ptr);
    EpGCObject* val_obj = ep_gc_table_get((void*)val);
    if (host_obj && val_obj && host_obj->generation == 1 && val_obj->generation == 0) {
        /* Check if already in remembered set */
        int found = 0;
        for (long long i = 0; i < ep_gc_remembered_size; i++) {
            if (ep_gc_remembered_set[i] == (void*)val) {
                found = 1;
                break;
            }
        }
        if (!found) {
            if (ep_gc_remembered_size >= ep_gc_remembered_cap) {
                long long new_cap = ep_gc_remembered_cap == 0 ? 128 : ep_gc_remembered_cap * 2;
                void** new_set = (void**)realloc(ep_gc_remembered_set, new_cap * sizeof(void*));
                if (new_set) {
                    ep_gc_remembered_set = new_set;
                    ep_gc_remembered_cap = new_cap;
                }
            }
            if (ep_gc_remembered_size < ep_gc_remembered_cap) {
                ep_gc_remembered_set[ep_gc_remembered_size++] = (void*)val;
            }
        }
    }
    pthread_mutex_unlock(&ep_gc_mutex);
}

/* Forward declarations for list type (needed by GC mark) */
typedef struct {
    long long* data;
    long long length;
    long long capacity;
} EpList;

/* A real heap object (list/map/string) is malloc'd, so its address is far above
   the never-mapped first page. EP values that are NOT pointers — small ints,
   booleans, and JSON type-tags (2=string, 3=list, 4=object) — land in [0,4096).
   Guarding the object accessors with this turns "deref a non-pointer" (the cause
   of the read_transcripts segfault, and that whole class) into a safe null return
   instead of a daemon-killing SIGSEGV. One comparison; negligible on hot paths. */
#define EP_BADPTR(p) (((unsigned long long)(p)) < 4096ULL)

/* Mark a single object and recursively mark its children */
static void ep_gc_mark_object(void* ptr) {
    if (!ptr) return;
    /* Runs under ep_gc_mutex (held by the collector) — use the no-lock lookup. */
    EpGCObject* obj = ep_gc_table_get(ptr);
    if (!obj || obj->marked) return;
    obj->marked = 1;

    if (obj->kind == EP_OBJ_LIST) {
        EpList* list = (EpList*)ptr;
        for (long long i = 0; i < list->length; i++) {
            long long val = list->data[i];
            if (val != 0) {
                ep_gc_mark_object((void*)val);
            }
        }
    } else if (obj->kind == EP_OBJ_STRUCT) {
        long long* fields = (long long*)ptr;
        for (long long i = 0; i < obj->num_fields; i++) {
            if (fields[i] != 0) {
                ep_gc_mark_object((void*)fields[i]);
            }
        }
    } else if (obj->kind == EP_OBJ_MAP) {
        if (ep_gc_mark_map_values) ep_gc_mark_map_values(ptr);
    }
}

/* Mark a single object and recursively mark its children (only if it is Gen 0) */
static void ep_gc_mark_object_minor(void* ptr) {
    if (!ptr) return;
    /* Runs under ep_gc_mutex (held by the collector) — use the no-lock lookup. */
    EpGCObject* obj = ep_gc_table_get(ptr);
    if (!obj || obj->generation != 0 || obj->marked) return;
    obj->marked = 1;

    if (obj->kind == EP_OBJ_LIST) {
        EpList* list = (EpList*)ptr;
        for (long long i = 0; i < list->length; i++) {
            long long val = list->data[i];
            if (val != 0) {
                ep_gc_mark_object_minor((void*)val);
            }
        }
    } else if (obj->kind == EP_OBJ_STRUCT) {
        long long* fields = (long long*)ptr;
        for (long long i = 0; i < obj->num_fields; i++) {
            if (fields[i] != 0) {
                ep_gc_mark_object_minor((void*)fields[i]);
            }
        }
    } else if (obj->kind == EP_OBJ_MAP) {
        if (ep_gc_mark_map_values_minor) ep_gc_mark_map_values_minor(ptr);
    }
}

/* Conservatively scan every registered thread's C stack and mark any word that
   looks like a tracked pointer. The collector spills its own registers and
   publishes its top here; all other threads are parked at a safepoint with their
   registers spilled and top published (ep_gc_park_if_stopped), so their stacks
   are frozen. This complements the precise shadow stacks: it catches roots held
   only in registers/temporaries (e.g. a freshly allocated object not yet stored
   into a rooted slot). Non-pointer words are harmlessly ignored by ep_gc_find.

   Only run on MAJOR collections: minor collections rely on the precise shadow
   stacks plus the write barrier's remembered set (the standard generational
   approach), so they do no stack scan at all — which means there is no racy
   cross-thread stack read on the frequent minor path either. The expensive
   full-stack scan is paid only on the rarer major collection, where it pins
   any long-lived object reachable only via a register across many GCs.

   Marked no_sanitize_address: a conservative scan deliberately reads whole stack
   ranges (including ASAN redzones and out-of-frame slots), which is not a bug. */
#if defined(__SANITIZE_ADDRESS__)
# define EP_NO_ASAN __attribute__((no_sanitize_address))
#elif defined(__has_feature)
# if __has_feature(address_sanitizer)
#  define EP_NO_ASAN __attribute__((no_sanitize_address))
# endif
#endif
#ifndef EP_NO_ASAN
# define EP_NO_ASAN
#endif
EP_NO_ASAN
static void ep_gc_scan_thread_stacks(void) {
    jmp_buf _regs;
    volatile char _top_marker;
    memset(&_regs, 0, sizeof(_regs));
    setjmp(_regs);   /* spill the collector's own registers onto its stack */
    /* Publish the LOWEST of our own local addresses as this thread's live top, so the
       scanned range covers both the stack marker and the register-spill buffer whatever
       order the compiler laid them out (a missed _regs would drop a register-only root). */
    { char* _a = (char*)(void*)&_top_marker; char* _b = (char*)(void*)&_regs;
      ep_thread_local_top = (void*)((_a < _b) ? _a : _b); }
    for (int t = 0; t < ep_num_threads; t++) {
        if (!ep_thread_active[t]) continue;
        if (!ep_thread_tops[t]) continue;
        void** start = (void**)*ep_thread_tops[t];
        void** end = (void**)ep_thread_bottoms[t];
        if (!start || !end) continue;
        if (start > end) { void** tmp = start; start = end; end = tmp; }
        for (void** cur = start; cur < end; cur++) {
            void* p = *cur;
            if (p) ep_gc_mark_object(p);
        }
    }
}

/* Mark phase: traverse from ALL threads' explicit GC roots.
   Uses the heap-allocated EpThreadGCState instead of raw __thread pointers. */
static void ep_gc_mark(void) {
    ep_gc_scan_thread_stacks();  /* conservative C-stack scan of all (parked) threads — major only */
    for (int t = 0; t < ep_num_threads; t++) {
        if (!ep_thread_active[t]) continue;
        EpThreadGCState* state = ep_thread_gc_states[t];
        if (!state) continue;
        int sp = state->sp;
        if (sp <= 0 || sp > EP_GC_MAX_ROOTS) continue;
        for (int i = 0; i < sp; i++) {
            long long* root_ptr = state->roots[i];
            if (!root_ptr) continue;
            long long val = *root_ptr;
            if (val != 0) {
                ep_gc_mark_object((void*)val);
            }
        }
    }
    /* Also mark from main thread's local root stack (thread 0 / unregistered) */
    int local_sp = ep_gc_root_sp;
    if (local_sp > EP_GC_MAX_ROOTS) local_sp = EP_GC_MAX_ROOTS;
    for (int i = 0; i < local_sp; i++) {
        long long val = *ep_gc_root_stack[i];
        if (val != 0) {
            ep_gc_mark_object((void*)val);
        }
    }
    /* Mark active tasks in the scheduler run queue */
    EpTask* task = ep_run_queue_head;
    while (task) {
        if (task->fut) {
            ep_gc_mark_object((void*)task->fut);
        }
        if (task->args && task->args_size_bytes > 0) {
            long long* ptr = (long long*)task->args;
            for (int i = 0; i < task->args_size_bytes / 8; i++) {
                long long val = ptr[i];
                if (val != 0) ep_gc_mark_object((void*)val);
            }
        }
        task = task->next;
    }
    /* Mark active tasks in the timers queue */
    EpTimer* timer = ep_timers_head;
    while (timer) {
        if (timer->task) {
            EpTask* t = timer->task;
            if (t->fut) {
                ep_gc_mark_object((void*)t->fut);
            }
            if (t->args && t->args_size_bytes > 0) {
                long long* ptr = (long long*)t->args;
                for (int i = 0; i < t->args_size_bytes / 8; i++) {
                    long long val = ptr[i];
                    if (val != 0) ep_gc_mark_object((void*)val);
                }
            }
        }
        timer = timer->next;
    }
    /* Mark top-level constant/global variables (roots outside any frame) */
    if (ep_gc_mark_globals_major) ep_gc_mark_globals_major();
    /* Scan all registered channel buffers — values in-transit have no root */
    if (ep_gc_scan_channels_major) ep_gc_scan_channels_major();
}

/* Conservatively scan the CURRENT thread's own live C stack and mark any YOUNG object it
   finds. This closes a use-after-free on the frequent minor path: a freshly-allocated
   argument temporary — e.g. the result of g() while f(g() and h()) is still evaluating
   h() — lives only on the C stack / in registers and is not yet on the precise shadow
   stack, so a minor collection triggered mid-expression would otherwise free it. Scanning
   ONLY the collecting thread's own stack is race-free (no cross-thread read) and cheap
   (one bounded stack, current thread only). Non-pointer words are harmlessly ignored by
   ep_gc_table_get; only generation-0 objects are marked. The setjmp spills register-held
   roots onto the stack so the scan can see them. */
EP_NO_ASAN
static void ep_gc_scan_own_stack_minor(void) {
    jmp_buf _regs;
    volatile char _marker;
    memset(&_regs, 0, sizeof(_regs));
    setjmp(_regs);   /* spill callee-saved registers into _regs, on the stack */
    void* bottom = ep_thread_local_bottom;
    if (!bottom) return;
    /* Start at the LOWEST of our own local addresses so the scanned range covers both
       the current stack top (_marker) and the register-spill buffer (_regs), regardless
       of how the compiler ordered these locals on the stack. Missing _regs would drop a
       root held only in a callee-saved register -> a rare use-after-free. */
    char* a = (char*)(void*)&_marker;
    char* b = (char*)(void*)&_regs;
    char* lo = (a < b) ? a : b;
    void** start = (void**)lo;
    void** end = (void**)bottom;
    if (start > end) { void** tmp = start; start = end; end = tmp; }
    for (void** cur = start; cur < end; cur++) {
        void* p = *cur;
        if (p) ep_gc_mark_object_minor(p);
    }
}

static void ep_gc_mark_minor(void) {
    /* Conservatively scan our OWN live C stack first, to catch freshly-allocated argument
       temporaries (only on the stack / in registers, not yet on the shadow stack) that a
       minor collection mid-expression would otherwise free. Own-thread only, so race-free. */
    ep_gc_scan_own_stack_minor();
    for (int t = 0; t < ep_num_threads; t++) {
        if (!ep_thread_active[t]) continue;
        EpThreadGCState* state = ep_thread_gc_states[t];
        if (!state) continue;
        int sp = state->sp;
        if (sp <= 0 || sp > EP_GC_MAX_ROOTS) continue;
        for (int i = 0; i < sp; i++) {
            long long* root_ptr = state->roots[i];
            if (!root_ptr) continue;
            long long val = *root_ptr;
            if (val != 0) {
                ep_gc_mark_object_minor((void*)val);
            }
        }
    }
    int local_sp = ep_gc_root_sp;
    if (local_sp > EP_GC_MAX_ROOTS) local_sp = EP_GC_MAX_ROOTS;
    for (int i = 0; i < local_sp; i++) {
        long long val = *ep_gc_root_stack[i];
        if (val != 0) {
            ep_gc_mark_object_minor((void*)val);
        }
    }
    /* Mark active tasks in the scheduler run queue for minor collection */
    EpTask* task = ep_run_queue_head;
    while (task) {
        if (task->fut) {
            ep_gc_mark_object_minor((void*)task->fut);
        }
        if (task->args && task->args_size_bytes > 0) {
            long long* ptr = (long long*)task->args;
            for (int i = 0; i < task->args_size_bytes / 8; i++) {
                long long val = ptr[i];
                if (val != 0) ep_gc_mark_object_minor((void*)val);
            }
        }
        task = task->next;
    }
    /* Mark active tasks in the timers queue for minor collection */
    EpTimer* timer = ep_timers_head;
    while (timer) {
        if (timer->task) {
            EpTask* t = timer->task;
            if (t->fut) {
                ep_gc_mark_object_minor((void*)t->fut);
            }
            if (t->args && t->args_size_bytes > 0) {
                long long* ptr = (long long*)t->args;
                for (int i = 0; i < t->args_size_bytes / 8; i++) {
                    long long val = ptr[i];
                    if (val != 0) ep_gc_mark_object_minor((void*)val);
                }
            }
        }
        timer = timer->next;
    }
    /* Also mark from the remembered set */
    for (long long i = 0; i < ep_gc_remembered_size; i++) {
        ep_gc_mark_object_minor(ep_gc_remembered_set[i]);
    }
    /* Mark top-level constant/global variables (roots outside any frame) */
    if (ep_gc_mark_globals_minor) ep_gc_mark_globals_minor();
    /* Scan all registered channel buffers — values in-transit have no root */
    if (ep_gc_scan_channels_minor) ep_gc_scan_channels_minor();
}

static void ep_gc_sweep_minor(void) {
    EpGCObject** cur = &ep_gc_head;
    while (*cur) {
        if ((*cur)->generation == 0) {
            if (!(*cur)->marked) {
                EpGCObject* garbage = *cur;
                *cur = garbage->next;
                ep_gc_table_remove(garbage->ptr);
                if (garbage->kind == EP_OBJ_LIST) {
                    EpList* list = (EpList*)garbage->ptr;
                    if (list) {
                        free(list->data);
                        free(list);
                    }
                } else if (garbage->kind == EP_OBJ_STRING) {
                    free(garbage->ptr);
                } else if (garbage->kind == EP_OBJ_STRUCT) {
                    free(garbage->ptr);
                } else if (garbage->kind == EP_OBJ_CLOSURE) {
                    free(garbage->ptr);
                } else if (garbage->kind == EP_OBJ_MAP) {
                    /* EpMap layout: entries*, capacity, size. Free entries then map. */
                    void** map_fields = (void**)garbage->ptr;
                    if (map_fields && map_fields[0]) free(map_fields[0]); /* entries */
                    free(garbage->ptr);
                }
                free(garbage);
                ep_gc_count--;
                ep_gc_nursery_count--;
            } else {
                (*cur)->marked = 0;
                (*cur)->generation = 1;
                ep_gc_nursery_count--;
                cur = &(*cur)->next;
            }
        } else {
            cur = &(*cur)->next;
        }
    }
    ep_gc_remembered_size = 0;
}

static void ep_gc_sweep_major(void) {
    EpGCObject** cur = &ep_gc_head;
    while (*cur) {
        if (!(*cur)->marked) {
            EpGCObject* garbage = *cur;
            *cur = garbage->next;
            ep_gc_table_remove(garbage->ptr);
            if (garbage->generation == 0) {
                ep_gc_nursery_count--;
            }
            if (garbage->kind == EP_OBJ_LIST) {
                EpList* list = (EpList*)garbage->ptr;
                if (list) {
                    free(list->data);
                    free(list);
                }
            } else if (garbage->kind == EP_OBJ_STRING) {
                free(garbage->ptr);
            } else if (garbage->kind == EP_OBJ_STRUCT) {
                free(garbage->ptr);
            } else if (garbage->kind == EP_OBJ_CLOSURE) {
                free(garbage->ptr);
            } else if (garbage->kind == EP_OBJ_MAP) {
                void** map_fields = (void**)garbage->ptr;
                if (map_fields && map_fields[0]) free(map_fields[0]);
                free(garbage->ptr);
            }
            free(garbage);
            ep_gc_count--;
        } else {
            (*cur)->marked = 0;
            if ((*cur)->generation == 0) {
                (*cur)->generation = 1;
                ep_gc_nursery_count--;
            }
            cur = &(*cur)->next;
        }
    }
    ep_gc_remembered_size = 0;
}

static void ep_gc_collect_minor(void) {
    if (!ep_gc_enabled) return;
    ep_gc_minor_count++;
    ep_gc_mark_minor();
    ep_gc_sweep_minor();
}

static void ep_gc_collect_major(void) {
    if (!ep_gc_enabled) return;
    ep_gc_major_count++;
    ep_gc_mark();
    ep_gc_sweep_major();
    ep_gc_threshold = ep_gc_count * 2;
    if (ep_gc_threshold < 4096) ep_gc_threshold = 4096;
}

/* Run a full GC collection — caller MUST hold ep_gc_mutex */
static void ep_gc_collect(void) {
    ep_gc_collect_major();
}

/* Maybe trigger GC if we've exceeded threshold. Also serves as the per-function
   GC safepoint: if another thread has stopped the world, park here until it's done. */
static void ep_gc_maybe_collect(void) {
    if (!ep_gc_enabled) return;  /* Early exit if GC suppressed (e.g. during channel ops) */
    /* Safepoint: lock-free fast check, then park under the lock if a collection
       is in progress on another thread. Keeps the no-GC path lock-free. */
    if (ep_gc_stop_requested) {
        pthread_mutex_lock(&ep_gc_mutex);
        ep_gc_park_if_stopped();
        pthread_mutex_unlock(&ep_gc_mutex);
    }
    /* Fast path: check thresholds before acquiring mutex.
       Counters are only incremented under the mutex, so worst case
       we miss one collection cycle — safe trade-off for avoiding
       a mutex lock/unlock (~20-50ns) on every function call. */
    if (ep_gc_nursery_count < ep_gc_nursery_threshold && ep_gc_count < ep_gc_threshold) return;
    EP_GC_UPDATE_TOP();
    pthread_mutex_lock(&ep_gc_mutex);
    /* Another thread may have started collecting between the check and the lock —
       park instead of racing it, then re-check thresholds under the lock. */
    ep_gc_park_if_stopped();
    if (ep_gc_nursery_count >= ep_gc_nursery_threshold || ep_gc_count >= ep_gc_threshold) {
        ep_gc_stop_the_world();
        if (ep_gc_nursery_count >= ep_gc_nursery_threshold) {
            ep_gc_collect_minor();
        }
        if (ep_gc_count >= ep_gc_threshold) {
            ep_gc_collect_major();
        }
        ep_gc_start_the_world();
    }
    pthread_mutex_unlock(&ep_gc_mutex);
}

/* Unregister an object (for explicit free — removes from GC tracking) */
static void ep_gc_unregister(void* ptr) {
    if (!ptr) return;
    pthread_mutex_lock(&ep_gc_mutex);
    ep_gc_park_if_stopped();  /* safepoint: don't mutate the table mid-collection */
    /* Clean up references from the remembered set to prevent dangling pointers */
    for (long long i = 0; i < ep_gc_remembered_size; ) {
        if (ep_gc_remembered_set[i] == ptr) {
            for (long long j = i; j < ep_gc_remembered_size - 1; j++) {
                ep_gc_remembered_set[j] = ep_gc_remembered_set[j + 1];
            }
            ep_gc_remembered_size--;
        } else {
            i++;
        }
    }
    ep_gc_table_remove(ptr);
    EpGCObject** cur = &ep_gc_head;
    while (*cur) {
        if ((*cur)->ptr == ptr) {
            EpGCObject* found = *cur;
            *cur = found->next;
            if (found->generation == 0) {
                ep_gc_nursery_count--;
            }
            free(found);
            ep_gc_count--;
            pthread_mutex_unlock(&ep_gc_mutex);
            return;
        }
        cur = &(*cur)->next;
    }
    pthread_mutex_unlock(&ep_gc_mutex);
}

/* Cleanup all remaining GC objects (called at program exit) */
static void ep_gc_shutdown(void) {
    ep_gc_enabled = 0;
    /* Only free GC bookkeeping structures, not the tracked objects themselves.
       The RAII auto-cleanup has already freed owned objects, and the OS will
       reclaim everything else on process exit. Attempting to free individual
       objects here causes double-free aborts when RAII and GC both track
       the same allocation. */
    EpGCObject* cur = ep_gc_head;
    while (cur) {
        EpGCObject* next = cur->next;
        free(cur);  /* free the GCObject wrapper only */
        cur = next;
    }
    ep_gc_head = NULL;
    ep_gc_count = 0;
    if (ep_gc_table) {
        free(ep_gc_table);
        ep_gc_table = NULL;
    }
    ep_gc_table_cap = 0;
    ep_gc_table_size = 0;
}

/* ========== End Garbage Collector ========== */

long long create_list(void);
long long append_list(long long list_ptr, long long value);
long long get_list(long long list_ptr, long long index);
long long set_list(long long list_ptr, long long index, long long value);
long long length_list(long long list_ptr);
long long free_list(long long list_ptr);
long long pop_list(long long list_ptr);
long long remove_list(long long list_ptr, long long index);
char* string_from_list(long long list_ptr);
long long string_to_list(const char* s);
long long string_length(const char* s);
long long display_string(const char* s);
long long file_read(long long path_val);
long long file_write(long long path_val, long long content_val);
long long file_append(long long path_val, long long content_val);
long long file_exists(long long path_val);
long long string_contains(long long s_val, long long sub_val);
long long string_index_of(long long s_val, long long sub_val);
long long string_replace(long long s_val, long long old_val, long long new_val);
long long string_upper(long long s_val);
long long string_lower(long long s_val);
long long string_trim(long long s_val);
long long string_split(long long s_val, long long delim_val);
long long char_at(long long s_val, long long index);
long long char_from_code(long long code);
long long ep_abs(long long n);
long long json_get_string(long long json_val, long long key_val);
long long json_get_int(long long json_val, long long key_val);
long long json_get_bool(long long json_val, long long key_val);
long long ep_sha1(long long data_val);
long long ep_net_recv_bytes(long long fd, long long count);
long long channel_try_recv(long long chan_ptr, long long out_ptr);
long long channel_has_data(long long chan_ptr);
long long channel_select(long long channels_list, long long timeout_ms);
long long ep_auto_to_string(long long val);

typedef struct EpChannel_ {
    long long* data;
    long long capacity;
    long long head;
    long long tail;
    long long size;
    ep_mutex_t mutex;
    ep_cond_t cond_recv;
    ep_cond_t cond_send;
} EpChannel;

/* Global channel registry — allows GC to scan values in-transit in channel buffers.
   Without this, an object sent to a channel but not yet received has NO GC root:
   the sender has popped it, the receiver hasn't pushed it, and the channel buffer
   is not scanned. The GC sweeps it → receiver gets a dangling pointer. */
#define EP_MAX_CHANNELS 1024
static EpChannel* ep_channel_registry[EP_MAX_CHANNELS];
static int ep_channel_count = 0;
static pthread_mutex_t ep_channel_registry_mutex = PTHREAD_MUTEX_INITIALIZER;

static void ep_register_channel(EpChannel* chan) {
    pthread_mutex_lock(&ep_channel_registry_mutex);
    if (ep_channel_count < EP_MAX_CHANNELS) {
        ep_channel_registry[ep_channel_count++] = chan;
    }
    pthread_mutex_unlock(&ep_channel_registry_mutex);
}

/* Channel scanning implementations — called by GC mark via function pointers.
   These are defined here (after EpChannel) so they can access struct fields. */
static void ep_gc_mark_object(void* ptr);     /* forward decl */
static void ep_gc_mark_object_minor(void* ptr); /* forward decl */

static void ep_gc_scan_channels_major_impl(void) {
    pthread_mutex_lock(&ep_channel_registry_mutex);
    for (int c = 0; c < ep_channel_count; c++) {
        EpChannel* chan = ep_channel_registry[c];
        if (!chan || chan->size <= 0) continue;
        ep_mutex_lock(&chan->mutex);
        for (long long j = 0; j < chan->size; j++) {
            long long idx = (chan->head + j) % chan->capacity;
            long long val = chan->data[idx];
            if (val != 0) ep_gc_mark_object((void*)val);
        }
        ep_mutex_unlock(&chan->mutex);
    }
    pthread_mutex_unlock(&ep_channel_registry_mutex);
}

static void ep_gc_scan_channels_minor_impl(void) {
    pthread_mutex_lock(&ep_channel_registry_mutex);
    for (int c = 0; c < ep_channel_count; c++) {
        EpChannel* chan = ep_channel_registry[c];
        if (!chan || chan->size <= 0) continue;
        ep_mutex_lock(&chan->mutex);
        for (long long j = 0; j < chan->size; j++) {
            long long idx = (chan->head + j) % chan->capacity;
            long long val = chan->data[idx];
            if (val != 0) ep_gc_mark_object_minor((void*)val);
        }
        ep_mutex_unlock(&chan->mutex);
    }
    pthread_mutex_unlock(&ep_channel_registry_mutex);
}

long long create_channel(void) {
    EpChannel* chan = malloc(sizeof(EpChannel));
    if (!chan) return 0;
    chan->capacity = 1024;
    chan->data = malloc(chan->capacity * sizeof(long long));
    chan->head = 0;
    chan->tail = 0;
    chan->size = 0;
    ep_mutex_init(&chan->mutex);
    ep_cond_init(&chan->cond_recv);
    ep_cond_init(&chan->cond_send);
    ep_register_channel(chan);
    return (long long)chan;
}

long long send_channel(long long chan_ptr, long long value) {
    EpChannel* chan = (EpChannel*)chan_ptr;
    if (!chan) return 0;
    /* Suppress GC during channel operations. The blocking condvar wait
       can interleave with GC mark/sweep on another thread, causing
       use-after-free when the GC sweeps objects that are live on a
       thread currently blocked in send/receive. Channel buffers contain
       raw long long values (not GC-tracked pointers), so suppressing
       GC here is safe. */
    int gc_was_enabled = ep_gc_enabled;
    ep_gc_enabled = 0;
    ep_mutex_lock(&chan->mutex);
    while (chan->size >= chan->capacity) {
        ep_cond_wait(&chan->cond_send, &chan->mutex);
    }
    chan->data[chan->tail] = value;
    chan->tail = (chan->tail + 1) % chan->capacity;
    chan->size += 1;
    ep_cond_signal(&chan->cond_recv);
    ep_mutex_unlock(&chan->mutex);
    ep_gc_enabled = gc_was_enabled;
    return value;
}

long long receive_channel(long long chan_ptr) {
    EpChannel* chan = (EpChannel*)chan_ptr;
    if (!chan) return 0;
    /* Suppress GC during channel receive — same rationale as send_channel */
    int gc_was_enabled = ep_gc_enabled;
    ep_gc_enabled = 0;
    ep_mutex_lock(&chan->mutex);
    while (chan->size <= 0) {
        ep_cond_wait(&chan->cond_recv, &chan->mutex);
    }
    long long value = chan->data[chan->head];
    chan->head = (chan->head + 1) % chan->capacity;
    chan->size -= 1;
    ep_cond_signal(&chan->cond_send);
    ep_mutex_unlock(&chan->mutex);
    ep_gc_enabled = gc_was_enabled;
    return value;
}

// Non-blocking receive — returns 1 if data was available, 0 if channel empty
long long channel_try_recv(long long chan_ptr, long long out_ptr) {
    EpChannel* chan = (EpChannel*)chan_ptr;
    if (!chan) return 0;
    ep_mutex_lock(&chan->mutex);
    if (chan->size <= 0) {
        ep_mutex_unlock(&chan->mutex);
        return 0;
    }
    long long value = chan->data[chan->head];
    chan->head = (chan->head + 1) % chan->capacity;
    chan->size -= 1;
    ep_cond_signal(&chan->cond_send);
    ep_mutex_unlock(&chan->mutex);
    if (out_ptr) {
        *((long long*)out_ptr) = value;
    }
    return 1;
}

// Check if channel has data without consuming it
long long channel_has_data(long long chan_ptr) {
    EpChannel* chan = (EpChannel*)chan_ptr;
    if (!chan) return 0;
    ep_mutex_lock(&chan->mutex);
    int has = (chan->size > 0) ? 1 : 0;
    ep_mutex_unlock(&chan->mutex);
    return has;
}

// Select: wait for any of N channels to have data, with timeout in ms
// channels_list is a list of channel pointers
// Returns index (0-based) of first ready channel, or -1 on timeout
long long channel_select(long long channels_list, long long timeout_ms) {
    EpList* list = (EpList*)channels_list;
    if (!list || list->length == 0) return -1;
    
#ifdef _WIN32
    ULONGLONG start_tick = GetTickCount64();
#else
    struct timespec start, now;
    clock_gettime(CLOCK_MONOTONIC, &start);
#endif
    
    while (1) {
        // Poll all channels
        for (long long i = 0; i < list->length; i++) {
            EpChannel* chan = (EpChannel*)list->data[i];
            if (chan) {
                ep_mutex_lock(&chan->mutex);
                if (chan->size > 0) {
                    ep_mutex_unlock(&chan->mutex);
                    return i;
                }
                ep_mutex_unlock(&chan->mutex);
            }
        }
        
        // Check timeout
        if (timeout_ms >= 0) {
#ifdef _WIN32
            ULONGLONG now_tick = GetTickCount64();
            long long elapsed = (long long)(now_tick - start_tick);
#else
            clock_gettime(CLOCK_MONOTONIC, &now);
            long long elapsed = (now.tv_sec - start.tv_sec) * 1000 + (now.tv_nsec - start.tv_nsec) / 1000000;
#endif
            if (elapsed >= timeout_ms) return -1;
        }
        
        // Brief sleep to avoid busy-wait
#ifdef _WIN32
        Sleep(1);
#else
        usleep(1000); // 1ms
#endif
    }
}

#ifdef __wasm__
long long ep_net_connect(const char* host, long long port) {
    (void)host; (void)port;
    return -1;
}

long long ep_net_listen(long long port) {
    (void)port;
    return -1;
}

long long ep_net_accept(long long server_fd) {
    (void)server_fd;
    return -1;
}

long long ep_net_send(long long fd, const char* data) {
    (void)fd; (void)data;
    return 0;
}

char* ep_net_recv(long long fd, long long max_len) {
    (void)fd; (void)max_len;
    char* empty = malloc(1);
    if (empty) empty[0] = '\0';
    return empty;
}

long long ep_net_close(long long fd) {
    (void)fd;
    return -1;
}

long long ep_sleep_ms(long long ms) {
    struct timespec ts;
    ts.tv_sec = ms / 1000;
    ts.tv_nsec = (ms % 1000) * 1000000;
    nanosleep(&ts, NULL);
    return 0;
}

long long ep_system(long long cmd) {
    (void)cmd;
    return -1;
}

long long ep_play_sound(long long path) {
    (void)path;
    return -1;
}

long long ep_dlopen(long long path) {
    (void)path;
    return 0;
}

long long ep_dlsym(long long handle, long long name) {
    (void)handle; (void)name;
    return 0;
}

long long ep_dlclose(long long handle) {
    (void)handle;
    return 0;
}
#else
long long ep_net_connect(const char* host, long long port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return -1;
    struct hostent* server = gethostbyname(host);
    if (!server) {
#ifdef _WIN32
        closesocket(sockfd);
#else
        close(sockfd);
#endif
        return -1;
    }
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    memcpy(&serv_addr.sin_addr.s_addr, server->h_addr_list[0], server->h_length);
    serv_addr.sin_port = htons(port);
#ifdef _WIN32
    if (connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        closesocket(sockfd);
        return -1;
    }
#else
    // Bounded connect: an unreachable peer must not block ~75s on the OS SYN
    // timeout (this stalled node startup). Non-blocking connect + 5s select, then
    // restore blocking mode for the rest of the session.
    int _ep_flags = fcntl(sockfd, F_GETFL, 0);
    fcntl(sockfd, F_SETFL, _ep_flags | O_NONBLOCK);
    int _ep_cr = connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr));
    if (_ep_cr < 0) {
        if (errno != EINPROGRESS) { close(sockfd); return -1; }
        fd_set _ep_wset; FD_ZERO(&_ep_wset); FD_SET(sockfd, &_ep_wset);
        struct timeval _ep_tv; _ep_tv.tv_sec = 5; _ep_tv.tv_usec = 0;
        int _ep_sel = select(sockfd + 1, NULL, &_ep_wset, NULL, &_ep_tv);
        if (_ep_sel <= 0) { close(sockfd); return -1; } // timeout or error
        int _ep_so_err = 0; socklen_t _ep_slen = sizeof(_ep_so_err);
        if (getsockopt(sockfd, SOL_SOCKET, SO_ERROR, &_ep_so_err, &_ep_slen) < 0 || _ep_so_err != 0) {
            close(sockfd);
            return -1;
        }
    }
    fcntl(sockfd, F_SETFL, _ep_flags);
#endif
    return sockfd;
}

long long ep_net_listen(long long port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return -1;
    int opt = 1;
    setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, (const char*)&opt, sizeof(opt));
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = INADDR_ANY;
    serv_addr.sin_port = htons(port);
    if (bind(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
#ifdef _WIN32
        closesocket(sockfd);
#else
        close(sockfd);
#endif
        return -1;
    }
    if (listen(sockfd, 10) < 0) {
#ifdef _WIN32
        closesocket(sockfd);
#else
        close(sockfd);
#endif
        return -1;
    }
    return sockfd;
}

long long ep_net_accept(long long server_fd) {
    struct sockaddr_in cli_addr;
    socklen_t clilen = sizeof(cli_addr);
    int newsockfd = accept((int)server_fd, (struct sockaddr*)&cli_addr, &clilen);
    if (newsockfd >= 0) {
        /* Bound how long a single recv/send may block so a slow or silent
           client cannot pin a handler thread forever (slowloris). */
        struct timeval tv;
        tv.tv_sec = 30;
        tv.tv_usec = 0;
        setsockopt(newsockfd, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof(tv));
        setsockopt(newsockfd, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof(tv));
    }
    return newsockfd;
}

long long ep_net_send(long long fd, const char* data) {
    if (!data) return 0;
    /* send() may write fewer bytes than requested (partial write under load/
       backpressure). A single send() therefore silently truncated large IPC
       responses, cutting agent replies mid-stream. Loop until all bytes are sent. */
    size_t total = strlen(data);
    size_t off = 0;
    while (off < total) {
        ssize_t n = send((int)fd, data + off, total - off, 0);
        if (n <= 0) break;
        off += (size_t)n;
    }
    return (long long)off;
}

char* ep_net_recv(long long fd, long long max_len) {
    char* buf = malloc(max_len + 1);
    if (!buf) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
#ifdef _WIN32
    int n = recv((int)fd, buf, (int)max_len, 0);
#else
    ssize_t n = recv((int)fd, buf, max_len, 0);
#endif
    if (n < 0) n = 0;
    buf[n] = '\0';
    return buf;
}

long long ep_net_close(long long fd) {
#ifdef _WIN32
    return closesocket((int)fd);
#else
    return close((int)fd);
#endif
}

long long ep_sleep_ms(long long ms) {
#ifdef _WIN32
    Sleep((DWORD)ms);
#else
    usleep((useconds_t)(ms * 1000));
#endif
    return 0;
}

long long ep_system(long long cmd) {
    return (long long)system((const char*)cmd);
}

long long ep_play_sound(long long path) {
    char cmd[512];
    snprintf(cmd, sizeof(cmd), "afplay '%s' &", (const char*)path);
    return (long long)system(cmd);
}

/* ========== Dynamic Library Loading (FFI) ========== */
#ifndef _WIN32
#include <dlfcn.h>
#endif

long long ep_dlopen(long long path) {
#ifdef _WIN32
    HMODULE h = LoadLibraryA((const char*)path);
    return (long long)h;
#else
    const char* p = (const char*)path;
    void* handle = dlopen(p, RTLD_LAZY);
    return (long long)handle;
#endif
}

long long ep_dlsym(long long handle, long long name) {
#ifdef _WIN32
    FARPROC sym = GetProcAddress((HMODULE)handle, (const char*)name);
    return (long long)sym;
#else
    void* sym = dlsym((void*)handle, (const char*)name);
    return (long long)sym;
#endif
}

long long ep_dlclose(long long handle) {
#ifdef _WIN32
    return (long long)FreeLibrary((HMODULE)handle);
#else
    return (long long)dlclose((void*)handle);
#endif
}
#endif

/* Call a function pointer with 0..6 arguments.
   These are type-punned through long long — the C calling convention
   makes this work for integer and pointer arguments. */
typedef long long (*ep_fn0)(void);
typedef long long (*ep_fn1)(long long);
typedef long long (*ep_fn2)(long long, long long);
typedef long long (*ep_fn3)(long long, long long, long long);
typedef long long (*ep_fn4)(long long, long long, long long, long long);
typedef long long (*ep_fn5)(long long, long long, long long, long long, long long);
typedef long long (*ep_fn6)(long long, long long, long long, long long, long long, long long);
typedef long long (*ep_fn7)(long long, long long, long long, long long, long long, long long, long long);
typedef long long (*ep_fn8)(long long, long long, long long, long long, long long, long long, long long, long long);
typedef long long (*ep_fn9)(long long, long long, long long, long long, long long, long long, long long, long long, long long);
typedef long long (*ep_fn10)(long long, long long, long long, long long, long long, long long, long long, long long, long long, long long);

long long ep_dlcall0(long long fptr) {
    return ((ep_fn0)fptr)();
}
long long ep_dlcall1(long long fptr, long long a0) {
    return ((ep_fn1)fptr)(a0);
}
long long ep_dlcall2(long long fptr, long long a0, long long a1) {
    return ((ep_fn2)fptr)(a0, a1);
}
long long ep_dlcall3(long long fptr, long long a0, long long a1, long long a2) {
    return ((ep_fn3)fptr)(a0, a1, a2);
}
long long ep_dlcall4(long long fptr, long long a0, long long a1, long long a2, long long a3) {
    return ((ep_fn4)fptr)(a0, a1, a2, a3);
}
long long ep_dlcall5(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4) {
    return ((ep_fn5)fptr)(a0, a1, a2, a3, a4);
}
long long ep_dlcall6(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4, long long a5) {
    return ((ep_fn6)fptr)(a0, a1, a2, a3, a4, a5);
}
long long ep_dlcall7(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4, long long a5, long long a6) {
    return ((ep_fn7)fptr)(a0, a1, a2, a3, a4, a5, a6);
}
long long ep_dlcall8(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, long long a7) {
    return ((ep_fn8)fptr)(a0, a1, a2, a3, a4, a5, a6, a7);
}
long long ep_dlcall9(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, long long a7, long long a8) {
    return ((ep_fn9)fptr)(a0, a1, a2, a3, a4, a5, a6, a7, a8);
}
long long ep_dlcall10(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, long long a7, long long a8, long long a9) {
    return ((ep_fn10)fptr)(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
}

/* ========== Float FFI: ep_dlcall_f* ========== */
/* For calling C functions that accept/return double values.
   Arguments are passed as long long (bit-punned doubles).
   Return value is a double bit-punned back to long long.
   Use ep_double_to_bits() / ep_bits_to_double() to convert. */

typedef union { long long i; double f; } ep_float_bits;

static inline double ep_ll_to_double(long long v) {
    ep_float_bits u; u.i = v; return u.f;
}
static inline long long ep_double_to_ll(double v) {
    ep_float_bits u; u.f = v; return u.i;
}

/* Convert between ErnosPlain float representation and raw bits */
long long ep_double_to_bits(long long float_val) {
    /* float_val is already an EP Float stored as long long bits */
    return float_val;
}
long long ep_bits_to_double(long long bits) {
    return bits;
}

/* Float function pointer typedefs */
typedef double (*ep_ff0)(void);
typedef double (*ep_ff1)(double);
typedef double (*ep_ff2)(double, double);
typedef double (*ep_ff3)(double, double, double);
typedef double (*ep_ff4)(double, double, double, double);
typedef double (*ep_ff5)(double, double, double, double, double);
typedef double (*ep_ff6)(double, double, double, double, double, double);

/* Call functions that take doubles and return double */
long long ep_dlcall_f0(long long fptr) {
    return ep_double_to_ll(((ep_ff0)fptr)());
}
long long ep_dlcall_f1(long long fptr, long long a0) {
    return ep_double_to_ll(((ep_ff1)fptr)(ep_ll_to_double(a0)));
}
long long ep_dlcall_f2(long long fptr, long long a0, long long a1) {
    return ep_double_to_ll(((ep_ff2)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1)));
}
long long ep_dlcall_f3(long long fptr, long long a0, long long a1, long long a2) {
    return ep_double_to_ll(((ep_ff3)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1), ep_ll_to_double(a2)));
}
long long ep_dlcall_f4(long long fptr, long long a0, long long a1, long long a2, long long a3) {
    return ep_double_to_ll(((ep_ff4)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1), ep_ll_to_double(a2), ep_ll_to_double(a3)));
}
long long ep_dlcall_f5(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4) {
    return ep_double_to_ll(((ep_ff5)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1), ep_ll_to_double(a2), ep_ll_to_double(a3), ep_ll_to_double(a4)));
}
long long ep_dlcall_f6(long long fptr, long long a0, long long a1, long long a2, long long a3, long long a4, long long a5) {
    return ep_double_to_ll(((ep_ff6)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1), ep_ll_to_double(a2), ep_ll_to_double(a3), ep_ll_to_double(a4), ep_ll_to_double(a5)));
}

/* Variants that take doubles but return int (for comparison functions etc.) */
typedef long long (*ep_fdi1)(double);
typedef long long (*ep_fdi2)(double, double);
typedef long long (*ep_fdi3)(double, double, double);

long long ep_dlcall_fd1(long long fptr, long long a0) {
    return ((ep_fdi1)fptr)(ep_ll_to_double(a0));
}
long long ep_dlcall_fd2(long long fptr, long long a0, long long a1) {
    return ((ep_fdi2)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1));
}
long long ep_dlcall_fd3(long long fptr, long long a0, long long a1, long long a2) {
    return ((ep_fdi3)fptr)(ep_ll_to_double(a0), ep_ll_to_double(a1), ep_ll_to_double(a2));
}
/* ========== End Float FFI ========== */
/* ========== End Dynamic Library Loading ========== */

unsigned long hash_string(const char* str) {
    unsigned long hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

typedef struct {
    char* key;
    long long value;
    int used;
} EpMapEntry;

typedef struct {
    EpMapEntry* entries;
    long long capacity;
    long long size;
} EpMap;

/* Map value traversal for GC — walks all entries and marks values.
   Called by ep_gc_mark_object() via function pointer. */
static void ep_gc_mark_map_values_impl(void* ptr) {
    EpMap* map = (EpMap*)ptr;
    if (!map || !map->entries) return;
    for (long long i = 0; i < map->capacity; i++) {
        if (map->entries[i].used && map->entries[i].value != 0) {
            ep_gc_mark_object((void*)map->entries[i].value);
        }
        /* Also mark keys if they are heap strings */
        if (map->entries[i].used && map->entries[i].key != NULL) {
            ep_gc_mark_object((void*)map->entries[i].key);
        }
    }
}

static void ep_gc_mark_map_values_minor_impl(void* ptr) {
    EpMap* map = (EpMap*)ptr;
    if (!map || !map->entries) return;
    for (long long i = 0; i < map->capacity; i++) {
        if (map->entries[i].used && map->entries[i].value != 0) {
            ep_gc_mark_object_minor((void*)map->entries[i].value);
        }
        if (map->entries[i].used && map->entries[i].key != NULL) {
            ep_gc_mark_object_minor((void*)map->entries[i].key);
        }
    }
}

long long create_map(void) {
    EpMap* map = malloc(sizeof(EpMap));
    if (!map) return 0;
    map->capacity = 16;
    map->size = 0;
    map->entries = calloc(map->capacity, sizeof(EpMapEntry));
    if (!map->entries) {
        free(map);
        return 0;
    }
    ep_gc_register(map, EP_OBJ_MAP);
    return (long long)map;
}

static void map_resize(EpMap* map, long long new_capacity) {
    EpMapEntry* old_entries = map->entries;
    long long old_capacity = map->capacity;
    map->capacity = new_capacity;
    map->entries = calloc(new_capacity, sizeof(EpMapEntry));
    map->size = 0;
    for (long long i = 0; i < old_capacity; i++) {
        if (old_entries[i].used && old_entries[i].key != NULL) {
            char* key = old_entries[i].key;
            long long value = old_entries[i].value;
            unsigned long h = hash_string(key) % new_capacity;
            while (map->entries[h].used) {
                h = (h + 1) % new_capacity;
            }
            map->entries[h].key = key;
            map->entries[h].value = value;
            map->entries[h].used = 1;
            map->size++;
        }
    }
    free(old_entries);
}

/* Convert a key value to a string — handles both string pointers and integers */
static const char* ep_map_key_str(long long key_val, char* buf, int bufsize) {
    if (key_val == 0) { buf[0] = '0'; buf[1] = '\0'; return buf; }
    /* Check if value is in plausible pointer range for a string */
    if (key_val > 0x100000) {
        const char* p = (const char*)(void*)key_val;
        unsigned char first = (unsigned char)*p;
        if ((first >= 0x20 && first < 0x7F) || first >= 0xC0 || first == 0) {
            return p; /* valid string pointer */
        }
    }
    snprintf(buf, bufsize, "%lld", key_val);
    return buf;
}

long long map_insert(long long map_ptr, long long key_val, long long value) {
    if (EP_BADPTR(map_ptr)) return 0;
    EpMap* map = (EpMap*)map_ptr;
    char keybuf[32];
    const char* key = ep_map_key_str(key_val, keybuf, sizeof(keybuf));
    if (!map) return 0;
    if (map->size * 2 >= map->capacity) {
        map_resize(map, map->capacity * 2);
    }
    unsigned long h = hash_string(key) % map->capacity;
    while (map->entries[h].used) {
        if (strcmp(map->entries[h].key, key) == 0) {
            map->entries[h].value = value;
            ep_gc_write_barrier((void*)map_ptr, value);
            return value;
        }
        h = (h + 1) % map->capacity;
    }
    map->entries[h].key = strdup(key);
    map->entries[h].value = value;
    map->entries[h].used = 1;
    map->size++;
    ep_gc_write_barrier((void*)map_ptr, value);
    return value;
}

long long map_get_val(long long map_ptr, long long key_val) {
    if (EP_BADPTR(map_ptr)) return 0;
    EpMap* map = (EpMap*)map_ptr;
    char keybuf[32];
    const char* key = ep_map_key_str(key_val, keybuf, sizeof(keybuf));
    if (!map) return 0;
    unsigned long h = hash_string(key) % map->capacity;
    long long start_h = h;
    while (map->entries[h].used) {
        if (map->entries[h].key && strcmp(map->entries[h].key, key) == 0) {
            return map->entries[h].value;
        }
        h = (h + 1) % map->capacity;
        if (h == start_h) break;
    }
    return 0;
}

/* map_set_str: store a string value (strdup'd copy) under a string key */
long long map_set_str(long long map_ptr, long long key_val, long long str_val) {
    /* Store the string pointer as a long long value — same as map_insert */
    return map_insert(map_ptr, key_val, str_val);
}

/* map_get_str: retrieve a string value from a map (returns char* as long long) */
long long map_get_str(long long map_ptr, long long key_val) {
    /* Same as map_get_val — the stored long long IS a char* pointer */
    return map_get_val(map_ptr, key_val);
}

long long map_contains(long long map_ptr, long long key_val) {
    if (EP_BADPTR(map_ptr)) return 0;
    EpMap* map = (EpMap*)map_ptr;
    char keybuf[32];
    const char* key = ep_map_key_str(key_val, keybuf, sizeof(keybuf));
    if (!map) return 0;
    unsigned long h = hash_string(key) % map->capacity;
    long long start_h = h;
    while (map->entries[h].used) {
        if (map->entries[h].key && strcmp(map->entries[h].key, key) == 0) {
            return 1;
        }
        h = (h + 1) % map->capacity;
        if (h == start_h) break;
    }
    return 0;
}

long long map_delete(long long map_ptr, long long key_val) {
    if (EP_BADPTR(map_ptr)) return 0;
    EpMap* map = (EpMap*)map_ptr;
    char keybuf[32];
    const char* key = ep_map_key_str(key_val, keybuf, sizeof(keybuf));
    if (!map) return 0;
    unsigned long h = hash_string(key) % map->capacity;
    long long start_h = h;
    while (map->entries[h].used) {
        if (map->entries[h].key && strcmp(map->entries[h].key, key) == 0) {
            free(map->entries[h].key);
            map->entries[h].key = NULL;
            map->entries[h].value = 0;
            map->entries[h].used = 0;
            map->size--;
            long long next_h = (h + 1) % map->capacity;
            while (map->entries[next_h].used) {
                char* k = map->entries[next_h].key;
                long long v = map->entries[next_h].value;
                map->entries[next_h].key = NULL;
                map->entries[next_h].value = 0;
                map->entries[next_h].used = 0;
                map->size--;
                map_insert(map_ptr, (long long)k, v);
                free(k);
                next_h = (next_h + 1) % map->capacity;
            }
            return 1;
        }
        h = (h + 1) % map->capacity;
        if (h == start_h) break;
    }
    return 0;
}

long long map_keys(long long map_ptr) {
    EpMap* map = (EpMap*)map_ptr;
    if (!map) return (long long)create_list();
    long long list = create_list();
    for (long long i = 0; i < map->capacity; i++) {
        if (map->entries[i].used && map->entries[i].key) {
            append_list(list, (long long)strdup(map->entries[i].key));
        }
    }
    return list;
}

long long map_values(long long map_ptr) {
    EpMap* map = (EpMap*)map_ptr;
    if (!map) return (long long)create_list();
    long long list = create_list();
    for (long long i = 0; i < map->capacity; i++) {
        if (map->entries[i].used && map->entries[i].key) {
            append_list(list, map->entries[i].value);
        }
    }
    return list;
}

long long map_size(long long map_ptr) {
    EpMap* map = (EpMap*)map_ptr;
    if (!map) return 0;
    return map->size;
}

long long free_map(long long map_ptr) {
    EpMap* map = (EpMap*)map_ptr;
    if (!map) return 0;
    /* Skip if already freed (idempotent) */
    if (!ep_gc_find(map)) return 0;
    ep_gc_unregister(map);
    for (long long i = 0; i < map->capacity; i++) {
        if (map->entries[i].used && map->entries[i].key != NULL) {
            free(map->entries[i].key);
        }
    }
    free(map->entries);
    free(map);
    return 0;
}

typedef struct {
    long long* data;
    long long capacity;
    long long head;
    long long tail;
    long long size;
} EpDeque;

long long create_deque(void) {
    EpDeque* dq = malloc(sizeof(EpDeque));
    if (!dq) return 0;
    dq->capacity = 16;
    dq->size = 0;
    dq->head = 0;
    dq->tail = 0;
    dq->data = malloc(dq->capacity * sizeof(long long));
    if (!dq->data) {
        free(dq);
        return 0;
    }
    return (long long)dq;
}

static void deque_resize(EpDeque* dq, long long new_capacity) {
    long long* new_data = malloc(new_capacity * sizeof(long long));
    for (long long i = 0; i < dq->size; i++) {
        new_data[i] = dq->data[(dq->head + i) % dq->capacity];
    }
    free(dq->data);
    dq->data = new_data;
    dq->capacity = new_capacity;
    dq->head = 0;
    dq->tail = dq->size;
}

long long deque_push_back(long long dq_ptr, long long value) {
    EpDeque* dq = (EpDeque*)dq_ptr;
    if (!dq) return 0;
    if (dq->size >= dq->capacity) {
        deque_resize(dq, dq->capacity * 2);
    }
    dq->data[dq->tail] = value;
    dq->tail = (dq->tail + 1) % dq->capacity;
    dq->size++;
    return value;
}

long long deque_push_front(long long dq_ptr, long long value) {
    EpDeque* dq = (EpDeque*)dq_ptr;
    if (!dq) return 0;
    if (dq->size >= dq->capacity) {
        deque_resize(dq, dq->capacity * 2);
    }
    dq->head = (dq->head - 1 + dq->capacity) % dq->capacity;
    dq->data[dq->head] = value;
    dq->size++;
    return value;
}

long long deque_pop_back(long long dq_ptr) {
    EpDeque* dq = (EpDeque*)dq_ptr;
    if (!dq || dq->size == 0) return 0;
    dq->tail = (dq->tail - 1 + dq->capacity) % dq->capacity;
    long long value = dq->data[dq->tail];
    dq->size--;
    return value;
}

long long deque_pop_front(long long dq_ptr) {
    EpDeque* dq = (EpDeque*)dq_ptr;
    if (!dq || dq->size == 0) return 0;
    long long value = dq->data[dq->head];
    dq->head = (dq->head + 1) % dq->capacity;
    dq->size--;
    return value;
}

long long deque_length(long long dq_ptr) {
    EpDeque* dq = (EpDeque*)dq_ptr;
    if (!dq) return 0;
    return dq->size;
}

long long free_deque(long long dq_ptr) {
    EpDeque* dq = (EpDeque*)dq_ptr;
    if (!dq) return 0;
    free(dq->data);
    free(dq);
    return 0;
}

/* Filesystem Operations */
#include <dirent.h>
#include <sys/stat.h>

long long fs_scan_dir(long long path_val) {
    const char* path = (const char*)path_val;
    long long list_ptr = create_list();
    if (!path) return list_ptr;
    DIR* d = opendir(path);
    if (!d) return list_ptr;
    struct dirent* dir;
    while ((dir = readdir(d)) != NULL) {
        if (strcmp(dir->d_name, ".") == 0 || strcmp(dir->d_name, "..") == 0) {
            continue;
        }
        char* name = strdup(dir->d_name);
        append_list(list_ptr, (long long)name);
    }
    closedir(d);
    return list_ptr;
}

long long fs_copy_file(long long src_val, long long dest_val) {
    const char* src = (const char*)src_val;
    const char* dest = (const char*)dest_val;
    if (!src || !dest) return 0;
    FILE* f_src = fopen(src, "rb");
    if (!f_src) return 0;
    FILE* f_dest = fopen(dest, "wb");
    if (!f_dest) {
        fclose(f_src);
        return 0;
    }
    char buf[4096];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), f_src)) > 0) {
        fwrite(buf, 1, n, f_dest);
    }
    fclose(f_src);
    fclose(f_dest);
    return 1;
}

long long fs_delete_file(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return 0;
    return remove(path) == 0 ? 1 : 0;
}

long long fs_move_file(long long src_val, long long dest_val) {
    const char* src = (const char*)src_val;
    const char* dest = (const char*)dest_val;
    if (!src || !dest) return 0;
    return rename(src, dest) == 0 ? 1 : 0;
}

long long fs_exists(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return 0;
    struct stat st;
    return stat(path, &st) == 0 ? 1 : 0;
}

long long fs_is_dir(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return 0;
    struct stat st;
    if (stat(path, &st) != 0) return 0;
    return S_ISDIR(st.st_mode) ? 1 : 0;
}

long long fs_is_file(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return 0;
    struct stat st;
    if (stat(path, &st) != 0) return 0;
    return S_ISREG(st.st_mode) ? 1 : 0;
}

long long fs_get_size(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return 0;
    struct stat st;
    if (stat(path, &st) != 0) return 0;
    return (long long)st.st_size;
}

/* HTTP Client */
#ifdef __wasm__
long long ep_http_request(long long method_val, long long url_val, long long headers_val, long long body_val) {
    (void)method_val; (void)url_val; (void)headers_val; (void)body_val;
    return (long long)strdup("Error: HTTP request is not supported on WebAssembly");
}
#else
long long ep_http_request(long long method_val, long long url_val, long long headers_val, long long body_val) {
    const char* method = (const char*)method_val;
    const char* url = (const char*)url_val;
    const char* headers = (const char*)headers_val;
    const char* body = (const char*)body_val;
    if (!method || !url) return (long long)strdup("");
    if (strncmp(url, "http://", 7) != 0) {
        return (long long)strdup("Error: only http:// protocol supported");
    }
    const char* host_start = url + 7;
    const char* path_start = strchr(host_start, '/');
    char host[256];
    char path[1024];
    if (path_start) {
        size_t host_len = path_start - host_start;
        if (host_len >= sizeof(host)) host_len = sizeof(host) - 1;
        strncpy(host, host_start, host_len);
        host[host_len] = '\0';
        strncpy(path, path_start, sizeof(path) - 1);
        path[sizeof(path) - 1] = '\0';
    } else {
        strncpy(host, host_start, sizeof(host) - 1);
        host[sizeof(host) - 1] = '\0';
        strcpy(path, "/");
    }
    int port = 80;
    char* colon = strchr(host, ':');
    if (colon) {
        *colon = '\0';
        port = atoi(colon + 1);
    }
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return (long long)strdup("Error: socket creation failed");
    struct hostent* server = gethostbyname(host);
    if (!server) {
        close(sockfd);
        return (long long)strdup("Error: host resolution failed");
    }
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    memcpy(&serv_addr.sin_addr.s_addr, server->h_addr_list[0], server->h_length);
    serv_addr.sin_port = htons(port);
    if (connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        close(sockfd);
        return (long long)strdup("Error: connection failed");
    }
    char req[4096];
    size_t body_len = body ? strlen(body) : 0;
    int req_len = snprintf(req, sizeof(req),
        "%s %s HTTP/1.1\r\n"
        "Host: %s\r\n"
        "Content-Length: %zu\r\n"
        "Connection: close\r\n"
        "%s%s"
        "\r\n",
        method, path, host, body_len, headers ? headers : "", (headers && strlen(headers) > 0 && headers[strlen(headers)-1] != '\n') ? "\r\n" : "");
    if (send(sockfd, req, req_len, 0) < 0) {
        close(sockfd);
        return (long long)strdup("Error: send failed");
    }
    if (body_len > 0) {
        if (send(sockfd, body, body_len, 0) < 0) {
            close(sockfd);
            return (long long)strdup("Error: send body failed");
        }
    }
    size_t resp_cap = 4096;
    size_t resp_len = 0;
    char* resp = malloc(resp_cap);
    if (!resp) {
        close(sockfd);
        return (long long)strdup("");
    }
    char recv_buf[4096];
    ssize_t n;
    while ((n = recv(sockfd, recv_buf, sizeof(recv_buf), 0)) > 0) {
        if (resp_len + n >= resp_cap) {
            resp_cap *= 2;
            char* new_resp = realloc(resp, resp_cap);
            if (!new_resp) {
                free(resp);
                close(sockfd);
                return (long long)strdup("Error: memory allocation failed");
            }
            resp = new_resp;
        }
        memcpy(resp + resp_len, recv_buf, n);
        resp_len += n;
    }
    resp[resp_len] = '\0';
    close(sockfd);
    // Strip HTTP headers — return only the body after \r\n\r\n
    char* http_body = strstr(resp, "\r\n\r\n");
    if (http_body) {
        http_body += 4;
        char* result = strdup(http_body);
        free(resp);
        return (long long)result;
    }
    return (long long)resp;
}
#endif

#define ROTRIGHT(word,bits) (((word) >> (bits)) | ((word) << (32-(bits))))
#define CH(x,y,z) (((x) & (y)) ^ (~(x) & (z)))
#define MAJ(x,y,z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define EP0(x) (ROTRIGHT(x,2) ^ ROTRIGHT(x,13) ^ ROTRIGHT(x,22))
#define EP1(x) (ROTRIGHT(x,6) ^ ROTRIGHT(x,11) ^ ROTRIGHT(x,25))
#define SIG0(x) (ROTRIGHT(x,7) ^ ROTRIGHT(x,18) ^ ((x) >> 3))
#define SIG1(x) (ROTRIGHT(x,17) ^ ROTRIGHT(x,19) ^ ((x) >> 10))

typedef struct {
    unsigned char data[64];
    unsigned int datalen;
    unsigned long long bitlen;
    unsigned int state[8];
} EP_SHA256_CTX;

static const unsigned int sha256_k[64] = {
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

void ep_sha256_transform(EP_SHA256_CTX *ctx, const unsigned char *data) {
    unsigned int a, b, c, d, e, f, g, h, i, j, t1, t2, m[64];
    for (i = 0, j = 0; i < 16; ++i, j += 4)
        m[i] = (data[j] << 24) | (data[j + 1] << 16) | (data[j + 2] << 8) | (data[j + 3]);
    for ( ; i < 64; ++i)
        m[i] = SIG1(m[i - 2]) + m[i - 7] + SIG0(m[i - 15]) + m[i - 16];
    a = ctx->state[0]; b = ctx->state[1]; c = ctx->state[2]; d = ctx->state[3];
    e = ctx->state[4]; f = ctx->state[5]; g = ctx->state[6]; h = ctx->state[7];
    for (i = 0; i < 64; ++i) {
        t1 = h + EP1(e) + CH(e,f,g) + sha256_k[i] + m[i];
        t2 = EP0(a) + MAJ(a,b,c);
        h = g; g = f; f = e; e = d + t1; d = c; c = b; b = a; a = t1 + t2;
    }
    ctx->state[0] += a; ctx->state[1] += b; ctx->state[2] += c; ctx->state[3] += d;
    ctx->state[4] += e; ctx->state[5] += f; ctx->state[6] += g; ctx->state[7] += h;
}

void ep_sha256_init(EP_SHA256_CTX *ctx) {
    ctx->datalen = 0; ctx->bitlen = 0;
    ctx->state[0] = 0x6a09e667; ctx->state[1] = 0xbb67ae85; ctx->state[2] = 0x3c6ef372; ctx->state[3] = 0xa54ff53a;
    ctx->state[4] = 0x510e527f; ctx->state[5] = 0x9b05688c; ctx->state[6] = 0x1f83d9ab; ctx->state[7] = 0x5be0cd19;
}

void ep_sha256_update(EP_SHA256_CTX *ctx, const unsigned char *data, size_t len) {
    for (size_t i = 0; i < len; ++i) {
        ctx->data[ctx->datalen] = data[i];
        ctx->datalen++;
        if (ctx->datalen == 64) {
            ep_sha256_transform(ctx, ctx->data);
            ctx->bitlen += 512;
            ctx->datalen = 0;
        }
    }
}

void ep_sha256_final(EP_SHA256_CTX *ctx, unsigned char *hash) {
    unsigned int i = ctx->datalen;
    if (ctx->datalen < 56) {
        ctx->data[i++] = 0x80;
        while (i < 56) ctx->data[i++] = 0x00;
    } else {
        ctx->data[i++] = 0x80;
        while (i < 64) ctx->data[i++] = 0x00;
        ep_sha256_transform(ctx, ctx->data);
        memset(ctx->data, 0, 56);
    }
    ctx->bitlen += ctx->datalen * 8;
    ctx->data[63] = ctx->bitlen; ctx->data[62] = ctx->bitlen >> 8;
    ctx->data[61] = ctx->bitlen >> 16; ctx->data[60] = ctx->bitlen >> 24;
    ctx->data[59] = ctx->bitlen >> 32; ctx->data[58] = ctx->bitlen >> 40;
    ctx->data[57] = ctx->bitlen >> 48; ctx->data[56] = ctx->bitlen >> 56;
    ep_sha256_transform(ctx, ctx->data);
    for (i = 0; i < 4; ++i) {
        hash[i]      = (ctx->state[0] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 4]  = (ctx->state[1] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 8]  = (ctx->state[2] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 12] = (ctx->state[3] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 16] = (ctx->state[4] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 20] = (ctx->state[5] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 24] = (ctx->state[6] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 28] = (ctx->state[7] >> (24 - i * 8)) & 0x000000ff;
    }
}

char* ep_sha256(const char* s) {
    if (!s) s = "";
    EP_SHA256_CTX ctx;
    ep_sha256_init(&ctx);
    ep_sha256_update(&ctx, (const unsigned char*)s, strlen(s));
    unsigned char hash[32];
    ep_sha256_final(&ctx, hash);
    char* result = malloc(65);
    if (result) {
        for (int i = 0; i < 32; i++) {
            snprintf(result + (i * 2), 3, "%02x", hash[i]);
        }
        result[64] = '\0';
    }
    return result;
}

/* RFC 2104 HMAC-SHA256. Operates on raw bytes with explicit lengths (binary
   safe), so keys/messages containing NUL bytes hash correctly. Returns a
   malloc'd 64-char lowercase hex string. */
long long ep_hmac_sha256(long long key_ptr, long long key_len, long long msg_ptr, long long msg_len) {
    const unsigned char* key = (const unsigned char*)key_ptr;
    const unsigned char* msg = (const unsigned char*)msg_ptr;
    size_t klen = (size_t)key_len;
    size_t mlen = (size_t)msg_len;

    unsigned char k0[64];
    memset(k0, 0, sizeof(k0));
    if (klen > 64) {
        /* Keys longer than the block size are replaced by their hash. */
        EP_SHA256_CTX kc;
        ep_sha256_init(&kc);
        ep_sha256_update(&kc, key ? key : (const unsigned char*)"", klen);
        unsigned char kh[32];
        ep_sha256_final(&kc, kh);
        memcpy(k0, kh, 32);
    } else if (key) {
        memcpy(k0, key, klen);
    }

    unsigned char ipad[64], opad[64];
    for (int i = 0; i < 64; i++) {
        ipad[i] = k0[i] ^ 0x36;
        opad[i] = k0[i] ^ 0x5c;
    }

    /* inner = H((K0 ^ ipad) || message) */
    EP_SHA256_CTX ic;
    ep_sha256_init(&ic);
    ep_sha256_update(&ic, ipad, 64);
    if (msg && mlen) ep_sha256_update(&ic, msg, mlen);
    unsigned char inner[32];
    ep_sha256_final(&ic, inner);

    /* mac = H((K0 ^ opad) || inner) */
    EP_SHA256_CTX oc;
    ep_sha256_init(&oc);
    ep_sha256_update(&oc, opad, 64);
    ep_sha256_update(&oc, inner, 32);
    unsigned char mac[32];
    ep_sha256_final(&oc, mac);

    char* out = (char*)malloc(65);
    if (out) {
        for (int i = 0; i < 32; i++) {
            snprintf(out + (i * 2), 3, "%02x", mac[i]);
        }
        out[64] = '\0';
    }
    return (long long)out;
}

typedef struct {
    unsigned int count[2];
    unsigned int state[4];
    unsigned char buffer[64];
} EP_MD5_CTX;

#define F(x,y,z) (((x) & (y)) | (~(x) & (z)))
#define G(x,y,z) (((x) & (z)) | ((y) & ~(z)))
#define H(x,y,z) ((x) ^ (y) ^ (z))
#define I(x,y,z) ((y) ^ ((x) | ~(z)))
#define ROTATE_LEFT(x,n) (((x) << (n)) | ((x) >> (32-(n))))

#define FF(a,b,c,d,x,s,ac) { \
    (a) += F((b),(c),(d)) + (x) + (ac); \
    (a) = ROTATE_LEFT((a),(s)); \
    (a) += (b); \
}
#define GG(a,b,c,d,x,s,ac) { \
    (a) += G((b),(c),(d)) + (x) + (ac); \
    (a) = ROTATE_LEFT((a),(s)); \
    (a) += (b); \
}
#define HH(a,b,c,d,x,s,ac) { \
    (a) += H((b),(c),(d)) + (x) + (ac); \
    (a) = ROTATE_LEFT((a),(s)); \
    (a) += (b); \
}
#define II(a,b,c,d,x,s,ac) { \
    (a) += I((b),(c),(d)) + (x) + (ac); \
    (a) = ROTATE_LEFT((a),(s)); \
    (a) += (b); \
}

void ep_md5_init(EP_MD5_CTX *ctx) {
    ctx->count[0] = ctx->count[1] = 0;
    ctx->state[0] = 0x67452301;
    ctx->state[1] = 0xefcdab89;
    ctx->state[2] = 0x98badcfe;
    ctx->state[3] = 0x10325476;
}

void ep_md5_transform(unsigned int state[4], const unsigned char block[64]) {
    unsigned int a = state[0], b = state[1], c = state[2], d = state[3], x[16];
    for (int i = 0, j = 0; i < 16; i++, j += 4)
        x[i] = (block[j]) | (block[j+1] << 8) | (block[j+2] << 16) | (block[j+3] << 24);

    FF(a, b, c, d, x[0], 7, 0xd76aa478); FF(d, a, b, c, x[1], 12, 0xe8c7b756); FF(c, d, a, b, x[2], 17, 0x242070db); FF(b, c, d, a, x[3], 22, 0xc1bdceee);
    FF(a, b, c, d, x[4], 7, 0xf57c0faf); FF(d, a, b, c, x[5], 12, 0x4787c62a); FF(c, d, a, b, x[6], 17, 0xa8304613); FF(b, c, d, a, x[7], 22, 0xfd469501);
    FF(a, b, c, d, x[8], 7, 0x698098d8); FF(d, a, b, c, x[9], 12, 0x8b44f7af); FF(c, d, a, b, x[10], 17, 0xffff5bb1); FF(b, c, d, a, x[11], 22, 0x895cd7be);
    FF(a, b, c, d, x[12], 7, 0x6b901122); FF(d, a, b, c, x[13], 12, 0xfd987193); FF(c, d, a, b, x[14], 17, 0xa679438e); FF(b, c, d, a, x[15], 22, 0x49b40821);

    GG(a, b, c, d, x[1], 5, 0xf61e2562); GG(d, a, b, c, x[6], 9, 0xc040b340); GG(c, d, a, b, x[11], 14, 0x265e5a51); GG(b, c, d, a, x[0], 20, 0xe9b6c7aa);
    GG(a, b, c, d, x[5], 5, 0xd62f105d); GG(d, a, b, c, x[10], 9, 0x02441453); GG(c, d, a, b, x[15], 14, 0xd8a1e681); GG(b, c, d, a, x[4], 20, 0xe7d3fbc8);
    GG(a, b, c, d, x[9], 5, 0x21e1cde6); GG(d, a, b, c, x[14], 9, 0xc33707d6); GG(c, d, a, b, x[3], 14, 0xf4d50d87); GG(b, c, d, a, x[8], 20, 0x455a14ed);
    GG(a, b, c, d, x[13], 5, 0xa9e3e905); GG(d, a, b, c, x[2], 9, 0xfcefa3f8); GG(c, d, a, b, x[7], 14, 0x676f02d9); GG(b, c, d, a, x[12], 20, 0x8d2a4c8a);

    HH(a, b, c, d, x[5], 4, 0xfffa3942); HH(d, a, b, c, x[8], 11, 0x8771f681); HH(c, d, a, b, x[11], 16, 0x6d9d6122); HH(b, c, d, a, x[14], 23, 0xfde5380c);
    HH(a, b, c, d, x[1], 4, 0xa4beea44); HH(d, a, b, c, x[4], 11, 0x4bdecfa9); HH(c, d, a, b, x[7], 16, 0xf6bb4b60); HH(b, c, d, a, x[10], 23, 0xbebfbc70);
    HH(a, b, c, d, x[13], 4, 0x289b7ec6); HH(d, a, b, c, x[0], 11, 0xeaa127fa); HH(c, d, a, b, x[3], 16, 0xd4ef3085); HH(b, c, d, a, x[6], 23, 0x04881d05);
    HH(a, b, c, d, x[9], 4, 0xd9d4d039); HH(d, a, b, c, x[12], 11, 0xe6db99e5); HH(c, d, a, b, x[15], 16, 0x1fa27cf8); HH(b, c, d, a, x[2], 23, 0xc4ac5665);

    II(a, b, c, d, x[0], 6, 0xf4292244); II(d, a, b, c, x[7], 10, 0x432aff97); II(c, d, a, b, x[14], 15, 0xab9423a7); II(b, c, d, a, x[5], 21, 0xfc93a039);
    II(a, b, c, d, x[12], 6, 0x655b59c3); II(d, a, b, c, x[3], 10, 0x8f0ccc92); II(c, d, a, b, x[10], 15, 0xffeff47d); II(b, c, d, a, x[1], 21, 0x85845dd1);
    II(a, b, c, d, x[8], 6, 0x6fa87e4f); II(d, a, b, c, x[15], 10, 0xfe2ce6e0); II(c, d, a, b, x[6], 15, 0xa3014314); II(b, c, d, a, x[13], 21, 0x4e0811a1);
    II(a, b, c, d, x[4], 6, 0xf7537e82); II(d, a, b, c, x[11], 10, 0xbd3af235); II(c, d, a, b, x[2], 15, 0x2ad7d2bb); II(b, c, d, a, x[9], 21, 0xeb86d391);

    state[0] += a; state[1] += b; state[2] += c; state[3] += d;
}

void ep_md5_update(EP_MD5_CTX *ctx, const unsigned char *input, size_t input_len) {
    unsigned int i = 0, index = (ctx->count[0] >> 3) & 0x3F, part_len = 64 - index;
    ctx->count[0] += input_len << 3;
    if (ctx->count[0] < (input_len << 3)) ctx->count[1]++;
    ctx->count[1] += input_len >> 29;
    if (input_len >= part_len) {
        memcpy(&ctx->buffer[index], input, part_len);
        ep_md5_transform(ctx->state, ctx->buffer);
        for (i = part_len; i + 63 < input_len; i += 64)
            ep_md5_transform(ctx->state, &input[i]);
        index = 0;
    }
    memcpy(&ctx->buffer[index], &input[i], input_len - i);
}

void ep_md5_final(EP_MD5_CTX *ctx, unsigned char digest[16]) {
    unsigned char bits[8];
    bits[0] = ctx->count[0]; bits[1] = ctx->count[0] >> 8; bits[2] = ctx->count[0] >> 16; bits[3] = ctx->count[0] >> 24;
    bits[4] = ctx->count[1]; bits[5] = ctx->count[1] >> 8; bits[6] = ctx->count[1] >> 16; bits[7] = ctx->count[1] >> 24;
    unsigned int index = (ctx->count[0] >> 3) & 0x3F, pad_len = (index < 56) ? (56 - index) : (120 - index);
    unsigned char padding[64];
    memset(padding, 0, 64); padding[0] = 0x80;
    ep_md5_update(ctx, padding, pad_len);
    ep_md5_update(ctx, bits, 8);
    for (int i = 0; i < 4; i++) {
        digest[i*4]     = ctx->state[i];
        digest[i*4 + 1] = ctx->state[i] >> 8;
        digest[i*4 + 2] = ctx->state[i] >> 16;
        digest[i*4 + 3] = ctx->state[i] >> 24;
    }
}

char* ep_md5(const char* s) {
    if (!s) s = "";
    EP_MD5_CTX ctx;
    ep_md5_init(&ctx);
    ep_md5_update(&ctx, (const unsigned char*)s, strlen(s));
    unsigned char hash[16];
    ep_md5_final(&ctx, hash);
    char* result = malloc(33);
    if (result) {
        for (int i = 0; i < 16; i++) {
            snprintf(result + (i * 2), 3, "%02x", hash[i]);
        }
        result[32] = '\0';
    }
    return result;
}

char* read_file_content(const char* filepath) {
    char mode[3];
    mode[0] = 'r';
    mode[1] = 'b';
    mode[2] = '\0';
    FILE* f = fopen(filepath, mode);
    if (!f) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    char* buf = malloc(size + 1);
    if (!buf) {
        fclose(f);
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    size_t read_bytes = fread(buf, 1, size, f);
    buf[read_bytes] = '\0';
    fclose(f);
    ep_gc_register(buf, EP_OBJ_STRING);
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

long long create_list(void) {
    EpList* list = malloc(sizeof(EpList));
    if (!list) return 0;
    list->capacity = 4;
    list->length = 0;
    list->data = malloc(list->capacity * sizeof(long long));
    ep_gc_register(list, EP_OBJ_LIST);
    return (long long)list;
}

long long get_list_data_ptr(long long list_ptr) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    return (long long)list->data;
}

long long append_list(long long list_ptr, long long value) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    if (list->length >= list->capacity) {
        list->capacity *= 2;
        list->data = realloc(list->data, list->capacity * sizeof(long long));
    }
    list->data[list->length] = value;
    list->length += 1;
    ep_gc_write_barrier((void*)list_ptr, value);
    return value;
}

long long get_list(long long list_ptr, long long index) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    if (index < 0 || index >= list->length) return 0;
    return list->data[index];
}

long long set_list(long long list_ptr, long long index, long long value) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    if (index < 0 || index >= list->length) return 0;
    list->data[index] = value;
    ep_gc_write_barrier((void*)list_ptr, value);
    return value;
}

long long length_list(long long list_ptr) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    return list->length;
}

long long free_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    /* Skip if already freed (idempotent) */
    if (!ep_gc_find(list)) return 0;
    ep_gc_unregister(list);
    free(list->data);
    free(list);
    return 0;
}

static int sqlite_list_callback(void* arg, int argc, char** argv, char** col_names) {
    EpList* rows = (EpList*)arg;
    EpList* row = (EpList*)create_list();
    for (int i = 0; i < argc; i++) {
        char* val = argv[i] ? strdup(argv[i]) : strdup("");
        append_list((long long)row, (long long)val);
    }
    append_list((long long)rows, (long long)row);
    return 0;
}

long long sqlite_get_callback_ptr(long long dummy) {
    return (long long)sqlite_list_callback;
}

/* SQLite type-safe wrappers — marshal between int and long long */
#ifdef EP_HAS_SQLITE
typedef struct sqlite3 sqlite3;
int sqlite3_open(const char*, sqlite3**);
int sqlite3_close(sqlite3*);
int sqlite3_exec(sqlite3*, const char*, int(*)(void*,int,char**,char**), void*, char**);

long long ep_sqlite3_open(long long filename, long long db_ptr) {
    sqlite3* db = NULL;
    int rc = sqlite3_open((const char*)filename, &db);
    if (rc == 0 && db_ptr != 0) {
        *((long long*)db_ptr) = (long long)db;
    }
    return (long long)rc;
}

long long ep_sqlite3_close(long long db) {
    return (long long)sqlite3_close((sqlite3*)db);
}

long long ep_sqlite3_exec(long long db, long long sql, long long callback, long long cb_arg, long long errmsg_ptr) {
    return (long long)sqlite3_exec((sqlite3*)db, (const char*)sql,
        (int(*)(void*,int,char**,char**))(callback),
        (void*)cb_arg, (char**)errmsg_ptr);
}

/* Prepared-statement API for parameterized queries (defeats SQL injection). */
typedef struct sqlite3_stmt sqlite3_stmt;
int sqlite3_prepare_v2(sqlite3*, const char*, int, sqlite3_stmt**, const char**);
int sqlite3_bind_text(sqlite3_stmt*, int, const char*, int, void(*)(void*));
int sqlite3_bind_int64(sqlite3_stmt*, int, long long);
int sqlite3_step(sqlite3_stmt*);
int sqlite3_column_count(sqlite3_stmt*);
const unsigned char* sqlite3_column_text(sqlite3_stmt*, int);
long long sqlite3_column_int64(sqlite3_stmt*, int);
int sqlite3_finalize(sqlite3_stmt*);

long long ep_sqlite3_prepare_v2(long long db, long long sql) {
    sqlite3_stmt* stmt = NULL;
    int rc = sqlite3_prepare_v2((sqlite3*)db, (const char*)sql, -1, &stmt, NULL);
    if (rc != 0) return 0;
    return (long long)stmt;
}

long long ep_sqlite3_bind_text(long long stmt, long long idx, long long value) {
    /* SQLITE_TRANSIENT ((void*)-1): sqlite copies the bound string. The value is
       a bound parameter, never concatenated into SQL — this is the safe path. */
    return (long long)sqlite3_bind_text((sqlite3_stmt*)stmt, (int)idx,
        (const char*)value, -1, (void(*)(void*))(intptr_t)-1);
}

long long ep_sqlite3_bind_int(long long stmt, long long idx, long long value) {
    return (long long)sqlite3_bind_int64((sqlite3_stmt*)stmt, (int)idx, value);
}

long long ep_sqlite3_step(long long stmt) {
    return (long long)sqlite3_step((sqlite3_stmt*)stmt);
}

long long ep_sqlite3_column_count(long long stmt) {
    return (long long)sqlite3_column_count((sqlite3_stmt*)stmt);
}

long long ep_sqlite3_column_text(long long stmt, long long col) {
    const unsigned char* t = sqlite3_column_text((sqlite3_stmt*)stmt, (int)col);
    if (!t) return (long long)strdup("");
    return (long long)strdup((const char*)t);
}

long long ep_sqlite3_column_int(long long stmt, long long col) {
    return sqlite3_column_int64((sqlite3_stmt*)stmt, (int)col);
}

long long ep_sqlite3_finalize(long long stmt) {
    return (long long)sqlite3_finalize((sqlite3_stmt*)stmt);
}
#endif /* EP_HAS_SQLITE */

int ep_argc = 0;
char** ep_argv = NULL;

void init_ep_args(int argc, char** argv) {
    ep_argc = argc;
    ep_argv = argv;
    ep_gc_register_thread((void*)&argc);
    /* Wire up channel scanning for GC (defined after EpChannel struct) */
    ep_gc_scan_channels_major = ep_gc_scan_channels_major_impl;
    ep_gc_scan_channels_minor = ep_gc_scan_channels_minor_impl;
    /* Wire up map value traversal for GC (defined after EpMap struct) */
    ep_gc_mark_map_values = ep_gc_mark_map_values_impl;
    ep_gc_mark_map_values_minor = ep_gc_mark_map_values_minor_impl;
}

long long get_argument_count(void) {
    return ep_argc;
}

const char* get_argument(long long index) {
    if (index < 0 || index >= ep_argc) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    return ep_argv[index];
}

long long write_file_content(const char* filepath, const char* content) {
    char mode[3];
    mode[0] = 'w';
    mode[1] = 'b';
    mode[2] = '\0';
    FILE* f = fopen(filepath, mode);
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
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    long long total_len = strlen(s);
    if (start < 0 || start >= total_len || len <= 0) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    if (start + len > total_len) {
        len = total_len - start;
    }
    char* sub = malloc(len + 1);
    if (!sub) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    strncpy(sub, s + start, len);
    sub[len] = '\0';
    ep_gc_register(sub, EP_OBJ_STRING);
    return sub;
}

char* string_from_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    char* s = malloc(list->length + 1);
    if (!s) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        ep_gc_register(empty, EP_OBJ_STRING);
        return empty;
    }
    for (long long i = 0; i < list->length; i++) {
        s[i] = (char)list->data[i];
    }
    s[list->length] = '\0';
    ep_gc_register(s, EP_OBJ_STRING);
    return s;
}

// Inverse of string_from_list: convert a string to a list of its byte values in
// a single O(n) pass (one strlen + one copy). This lets callers iterate a string
// in O(n) total via O(1) get_list, instead of O(n) get_character per index
// (which is O(n^2) over the whole string).
long long string_to_list(const char* s) {
    EpList* list = malloc(sizeof(EpList));
    if (!list) return 0;
    long long len = s ? (long long)strlen(s) : 0;
    list->capacity = len > 0 ? len : 4;
    list->length = len;
    list->data = malloc(list->capacity * sizeof(long long));
    if (!list->data) {
        list->capacity = 0;
        list->length = 0;
        ep_gc_register(list, EP_OBJ_LIST);
        return (long long)list;
    }
    for (long long i = 0; i < len; i++) {
        list->data[i] = (unsigned char)s[i];
    }
    ep_gc_register(list, EP_OBJ_LIST);
    return (long long)list;
}

long long pop_list(long long list_ptr) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    if (!list || list->length <= 0) return 0;
    list->length -= 1;
    return list->data[list->length];
}

long long remove_list(long long list_ptr, long long index) {
    if (EP_BADPTR(list_ptr)) return 0;
    EpList* list = (EpList*)list_ptr;
    if (!list || index < 0 || index >= list->length) return 0;
    long long removed = list->data[index];
    for (long long i = index; i < list->length - 1; i++) {
        list->data[i] = list->data[i + 1];
    }
    list->length -= 1;
    return removed;
}

long long display_string(const char* s) {
    if (s) puts(s);
    return 0;
}

/* ========== File System Runtime ========== */
#include <sys/stat.h>
#ifdef _WIN32
  #include <io.h>
  #include <direct.h>
  #define mkdir(p, m) _mkdir(p)
  #define rmdir _rmdir
  #define getcwd _getcwd
  #define popen _popen
  #define pclose _pclose
  #define getpid _getpid
  #define setenv(k, v, o) _putenv_s(k, v)
  /* Minimal dirent polyfill for Windows */
  #include <windows.h>
  typedef struct { char d_name[260]; } ep_dirent;
  typedef struct { HANDLE hFind; WIN32_FIND_DATAA data; int first; } EP_DIR;
  static EP_DIR* ep_opendir(const char* p) {
      EP_DIR* d = (EP_DIR*)malloc(sizeof(EP_DIR));
      char buf[270]; snprintf(buf, sizeof(buf), "%s\\*", p);
      d->hFind = FindFirstFileA(buf, &d->data);
      d->first = 1;
      return (d->hFind == INVALID_HANDLE_VALUE) ? (free(d), (EP_DIR*)NULL) : d;
  }
  static ep_dirent* ep_readdir(EP_DIR* d) {
      static ep_dirent ent;
      if (d->first) { d->first = 0; strcpy(ent.d_name, d->data.cFileName); return &ent; }
      if (!FindNextFileA(d->hFind, &d->data)) return NULL;
      strcpy(ent.d_name, d->data.cFileName); return &ent;
  }
  static void ep_closedir(EP_DIR* d) { FindClose(d->hFind); free(d); }
  #define DIR EP_DIR
  #define dirent ep_dirent
  #define opendir ep_opendir
  #define readdir ep_readdir
  #define closedir ep_closedir
#else
  #include <dirent.h>
  #include <unistd.h>
#endif

long long ep_read_file(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    FILE* f = fopen(path, "rb");
    if (!f) return (long long)"";
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    char* buf = (char*)malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';
    fclose(f);
    return (long long)buf;
}

long long ep_write_file(long long path_ptr, long long content_ptr) {
    const char* path = (const char*)path_ptr;
    const char* content = (const char*)content_ptr;
    FILE* f = fopen(path, "wb");
    if (!f) return 0;
    fputs(content, f);
    fclose(f);
    return 1;
}

long long ep_append_file(long long path_ptr, long long content_ptr) {
    const char* path = (const char*)path_ptr;
    const char* content = (const char*)content_ptr;
    FILE* f = fopen(path, "ab");
    if (!f) return 0;
    fputs(content, f);
    fclose(f);
    return 1;
}

long long ep_file_exists(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    struct stat st;
    return stat(path, &st) == 0 ? 1 : 0;
}

long long ep_is_directory(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    struct stat st;
    if (stat(path, &st) != 0) return 0;
    return S_ISDIR(st.st_mode) ? 1 : 0;
}

long long ep_file_size(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    struct stat st;
    if (stat(path, &st) != 0) return -1;
    return (long long)st.st_size;
}

long long ep_list_directory(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    DIR* dir = opendir(path);
    if (!dir) return (long long)create_list();
    long long list = create_list();
    struct dirent* entry;
    while ((entry = readdir(dir)) != NULL) {
        if (entry->d_name[0] == '.' && (entry->d_name[1] == '\0' || 
            (entry->d_name[1] == '.' && entry->d_name[2] == '\0'))) continue;
        char* name = strdup(entry->d_name);
        append_list(list, (long long)name);
    }
    closedir(dir);
    return list;
}

long long ep_create_directory(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    return mkdir(path, 0755) == 0 ? 1 : 0;
}

long long ep_remove_file(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    return remove(path) == 0 ? 1 : 0;
}

long long ep_remove_directory(long long path_ptr) {
    const char* path = (const char*)path_ptr;
    return rmdir(path) == 0 ? 1 : 0;
}

long long ep_rename_file(long long old_ptr, long long new_ptr) {
    return rename((const char*)old_ptr, (const char*)new_ptr) == 0 ? 1 : 0;
}

long long ep_copy_file(long long src_ptr, long long dst_ptr) {
    const char* src = (const char*)src_ptr;
    const char* dst = (const char*)dst_ptr;
    FILE* fin = fopen(src, "rb");
    if (!fin) return 0;
    FILE* fout = fopen(dst, "wb");
    if (!fout) { fclose(fin); return 0; }
    char buf[8192];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), fin)) > 0) {
        fwrite(buf, 1, n, fout);
    }
    fclose(fin);
    fclose(fout);
    return 1;
}

/* ========== Date/Time Runtime ========== */
#include <time.h>
#include <sys/time.h>

long long ep_time_now_ms(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return (long long)tv.tv_sec * 1000LL + (long long)tv.tv_usec / 1000LL;
}

long long ep_time_now_sec(void) {
    return (long long)time(NULL);
}


long long ep_time_year(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_year + 1900 : 0;
}

long long ep_time_month(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_mon + 1 : 0;
}

long long ep_time_day(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_mday : 0;
}

long long ep_time_hour(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_hour : 0;
}

long long ep_time_minute(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_min : 0;
}

long long ep_time_second(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_sec : 0;
}

long long ep_time_weekday(long long ts) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    return tm ? tm->tm_wday : 0;
}

long long ep_format_time(long long ts, long long fmt_ptr) {
    time_t t = (time_t)ts;
    struct tm* tm = localtime(&t);
    if (!tm) return (long long)"";
    char* buf = (char*)malloc(256);
    strftime(buf, 256, (const char*)fmt_ptr, tm);
    return (long long)buf;
}

/* ========== OS Runtime ========== */

long long ep_getenv(long long name_ptr) {
    const char* val = getenv((const char*)name_ptr);
    return val ? (long long)val : (long long)"";
}

long long ep_setenv(long long name_ptr, long long val_ptr) {
    return setenv((const char*)name_ptr, (const char*)val_ptr, 1) == 0 ? 1 : 0;
}

long long ep_get_cwd(void) {
    char* buf = (char*)malloc(4096);
    if (getcwd(buf, 4096)) return (long long)buf;
    free(buf);
    return (long long)"";
}

long long ep_os_name(void) {
    #if defined(__APPLE__)
    return (long long)"macos";
    #elif defined(__linux__)
    return (long long)"linux";
    #elif defined(_WIN32)
    return (long long)"windows";
    #else
    return (long long)"unknown";
    #endif
}

long long ep_arch_name(void) {
    #if defined(__aarch64__) || defined(__arm64__)
    return (long long)"arm64";
    #elif defined(__x86_64__)
    return (long long)"x86_64";
    #elif defined(__i386__)
    return (long long)"x86";
    #else
    return (long long)"unknown";
    #endif
}

long long ep_exit(long long code) {
    exit((int)code);
    return 0;
}

long long ep_get_pid(void) {
    return (long long)getpid();
}

long long ep_get_home_dir(void) {
    const char* home = getenv("HOME");
    return home ? (long long)home : (long long)"";
}

#ifdef __wasm__
long long ep_run_command(long long cmd_ptr) {
    (void)cmd_ptr;
    return (long long)"Error: running external commands is not supported on WebAssembly";
}
#else
long long ep_run_command(long long cmd_ptr) {
    const char* cmd = (const char*)cmd_ptr;
    FILE* fp = popen(cmd, "r");
    if (!fp) return (long long)"";
    char* result = (char*)malloc(65536);
    size_t total = 0;
    char buf[4096];
    while (fgets(buf, sizeof(buf), fp)) {
        size_t len = strlen(buf);
        memcpy(result + total, buf, len);
        total += len;
    }
    result[total] = '\0';
    pclose(fp);
    return (long long)result;
}
#endif

/* ========== HashMap helpers ========== */

long long ep_hash_string(long long s_ptr) {
    const char* s = (const char*)s_ptr;
    if (!s) return 0;
    unsigned long long hash = 5381;
    int c;
    while ((c = *s++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return (long long)hash;
}

long long ep_str_equals(long long a_ptr, long long b_ptr) {
    if (a_ptr == b_ptr) return 1;
    if (!a_ptr || !b_ptr) return 0;
    /* If either value looks like a small integer (not a valid heap pointer),
       fall back to integer comparison — strcmp would segfault. */
    if ((unsigned long long)a_ptr < 4096ULL || (unsigned long long)b_ptr < 4096ULL) return 0;
    return strcmp((const char*)a_ptr, (const char*)b_ptr) == 0 ? 1 : 0;
}

/* ========== Sync Primitives ========== */

#ifdef _WIN32
long long ep_mutex_create(void) {
    CRITICAL_SECTION* m = (CRITICAL_SECTION*)malloc(sizeof(CRITICAL_SECTION));
    InitializeCriticalSection(m);
    return (long long)m;
}
long long ep_mutex_lock_fn(long long m) {
    EnterCriticalSection((CRITICAL_SECTION*)m);
    return 1;
}
long long ep_mutex_unlock_fn(long long m) {
    LeaveCriticalSection((CRITICAL_SECTION*)m);
    return 1;
}
long long ep_mutex_trylock(long long m) {
    return TryEnterCriticalSection((CRITICAL_SECTION*)m) ? 1 : 0;
}
long long ep_mutex_destroy(long long m) {
    DeleteCriticalSection((CRITICAL_SECTION*)m);
    free((void*)m);
    return 0;
}
#else
long long ep_mutex_create(void) {
    pthread_mutex_t* m = (pthread_mutex_t*)malloc(sizeof(pthread_mutex_t));
    pthread_mutex_init(m, NULL);
    return (long long)m;
}

long long ep_mutex_lock_fn(long long m) {
    return pthread_mutex_lock((pthread_mutex_t*)m) == 0 ? 1 : 0;
}

long long ep_mutex_unlock_fn(long long m) {
    return pthread_mutex_unlock((pthread_mutex_t*)m) == 0 ? 1 : 0;
}

long long ep_mutex_trylock(long long m) {
    return pthread_mutex_trylock((pthread_mutex_t*)m) == 0 ? 1 : 0;
}

long long ep_mutex_destroy(long long m) {
    pthread_mutex_destroy((pthread_mutex_t*)m);
    free((void*)m);
    return 0;
}
#endif

#ifdef _WIN32
long long ep_rwlock_create(void) {
    SRWLOCK* rwl = (SRWLOCK*)malloc(sizeof(SRWLOCK));
    InitializeSRWLock(rwl);
    return (long long)rwl;
}
long long ep_rwlock_read_lock(long long rwl) {
    AcquireSRWLockShared((SRWLOCK*)rwl);
    return 1;
}
long long ep_rwlock_write_lock(long long rwl) {
    AcquireSRWLockExclusive((SRWLOCK*)rwl);
    return 1;
}
long long ep_rwlock_unlock(long long rwl) {
    /* SRWLOCK does not have a single "unlock" — we try exclusive first.
       In practice the caller should know which lock was taken.
       ReleaseSRWLockExclusive on a shared lock is undefined, but
       the runtime guarantees matched lock/unlock pairs. We default
       to releasing the exclusive lock; shared unlock is handled
       by pairing read_lock -> read_unlock if needed later. */
    ReleaseSRWLockExclusive((SRWLOCK*)rwl);
    return 1;
}
long long ep_rwlock_destroy(long long rwl) {
    /* SRWLOCK has no destroy */
    free((void*)rwl);
    return 0;
}
#else
long long ep_rwlock_create(void) {
    pthread_rwlock_t* rwl = (pthread_rwlock_t*)malloc(sizeof(pthread_rwlock_t));
    pthread_rwlock_init(rwl, NULL);
    return (long long)rwl;
}

long long ep_rwlock_read_lock(long long rwl) {
    return pthread_rwlock_rdlock((pthread_rwlock_t*)rwl) == 0 ? 1 : 0;
}

long long ep_rwlock_write_lock(long long rwl) {
    return pthread_rwlock_wrlock((pthread_rwlock_t*)rwl) == 0 ? 1 : 0;
}

long long ep_rwlock_unlock(long long rwl) {
    return pthread_rwlock_unlock((pthread_rwlock_t*)rwl) == 0 ? 1 : 0;
}

long long ep_rwlock_destroy(long long rwl) {
    pthread_rwlock_destroy((pthread_rwlock_t*)rwl);
    free((void*)rwl);
    return 0;
}
#endif

#ifdef _MSC_VER
long long ep_atomic_create(long long initial) {
    volatile long long* a = (volatile long long*)malloc(sizeof(long long));
    InterlockedExchange64(a, initial);
    return (long long)a;
}
long long ep_atomic_load(long long a) {
    return InterlockedCompareExchange64((volatile long long*)a, 0, 0);
}
long long ep_atomic_store(long long a, long long value) {
    InterlockedExchange64((volatile long long*)a, value);
    return value;
}
long long ep_atomic_add(long long a, long long delta) {
    return InterlockedExchangeAdd64((volatile long long*)a, delta);
}
long long ep_atomic_sub(long long a, long long delta) {
    return InterlockedExchangeAdd64((volatile long long*)a, -delta);
}
long long ep_atomic_cas(long long a, long long expected, long long desired) {
    long long old = InterlockedCompareExchange64((volatile long long*)a, desired, expected);
    return (old == expected) ? 1 : 0;
}
#else
long long ep_atomic_create(long long initial) {
    long long* a = (long long*)malloc(sizeof(long long));
    __atomic_store_n(a, initial, __ATOMIC_SEQ_CST);
    return (long long)a;
}

long long ep_atomic_load(long long a) {
    return __atomic_load_n((long long*)a, __ATOMIC_SEQ_CST);
}

long long ep_atomic_store(long long a, long long value) {
    __atomic_store_n((long long*)a, value, __ATOMIC_SEQ_CST);
    return value;
}

long long ep_atomic_add(long long a, long long delta) {
    return __atomic_fetch_add((long long*)a, delta, __ATOMIC_SEQ_CST);
}

long long ep_atomic_sub(long long a, long long delta) {
    return __atomic_fetch_sub((long long*)a, delta, __ATOMIC_SEQ_CST);
}

long long ep_atomic_cas(long long a, long long expected, long long desired) {
    long long exp = expected;
    return __atomic_compare_exchange_n((long long*)a, &exp, desired, 0, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST) ? 1 : 0;
}
#endif

/* Barrier — portable polyfill (macOS lacks pthread_barrier_t) */
typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    unsigned count;
    unsigned target;
    unsigned generation;
} EpBarrier;

long long ep_barrier_create(long long count) {
    EpBarrier* b = (EpBarrier*)malloc(sizeof(EpBarrier));
    pthread_mutex_init(&b->mutex, NULL);
    pthread_cond_init(&b->cond, NULL);
    b->count = 0;
    b->target = (unsigned)count;
    b->generation = 0;
    return (long long)b;
}

long long ep_barrier_wait(long long bp) {
    EpBarrier* b = (EpBarrier*)bp;
    pthread_mutex_lock(&b->mutex);
    unsigned gen = b->generation;
    b->count++;
    if (b->count >= b->target) {
        b->count = 0;
        b->generation++;
        pthread_cond_broadcast(&b->cond);
        pthread_mutex_unlock(&b->mutex);
        return 1; /* serial thread */
    }
    while (gen == b->generation) {
        pthread_cond_wait(&b->cond, &b->mutex);
    }
    pthread_mutex_unlock(&b->mutex);
    return 0;
}

long long ep_barrier_destroy(long long bp) {
    EpBarrier* b = (EpBarrier*)bp;
    pthread_mutex_destroy(&b->mutex);
    pthread_cond_destroy(&b->cond);
    free(b);
    return 0;
}

/* Semaphore via mutex+condvar (portable) */
typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    long long value;
} EpSemaphore;

long long ep_semaphore_create(long long initial) {
    EpSemaphore* s = (EpSemaphore*)malloc(sizeof(EpSemaphore));
    pthread_mutex_init(&s->mutex, NULL);
    pthread_cond_init(&s->cond, NULL);
    s->value = initial;
    return (long long)s;
}

long long ep_semaphore_wait(long long sp) {
    EpSemaphore* s = (EpSemaphore*)sp;
    pthread_mutex_lock(&s->mutex);
    while (s->value <= 0) {
        pthread_cond_wait(&s->cond, &s->mutex);
    }
    s->value--;
    pthread_mutex_unlock(&s->mutex);
    return 1;
}

long long ep_semaphore_post(long long sp) {
    EpSemaphore* s = (EpSemaphore*)sp;
    pthread_mutex_lock(&s->mutex);
    s->value++;
    pthread_cond_signal(&s->cond);
    pthread_mutex_unlock(&s->mutex);
    return 1;
}

long long ep_semaphore_trywait(long long sp) {
    EpSemaphore* s = (EpSemaphore*)sp;
    pthread_mutex_lock(&s->mutex);
    if (s->value > 0) {
        s->value--;
        pthread_mutex_unlock(&s->mutex);
        return 1;
    }
    pthread_mutex_unlock(&s->mutex);
    return 0;
}

long long ep_semaphore_destroy(long long sp) {
    EpSemaphore* s = (EpSemaphore*)sp;
    pthread_mutex_destroy(&s->mutex);
    pthread_cond_destroy(&s->cond);
    free(s);
    return 0;
}

long long ep_condvar_create(void) {
    pthread_cond_t* cv = (pthread_cond_t*)malloc(sizeof(pthread_cond_t));
    pthread_cond_init(cv, NULL);
    return (long long)cv;
}

long long ep_condvar_wait(long long cv, long long m) {
    return pthread_cond_wait((pthread_cond_t*)cv, (pthread_mutex_t*)m) == 0 ? 1 : 0;
}

long long ep_condvar_signal(long long cv) {
    return pthread_cond_signal((pthread_cond_t*)cv) == 0 ? 1 : 0;
}

long long ep_condvar_broadcast(long long cv) {
    return pthread_cond_broadcast((pthread_cond_t*)cv) == 0 ? 1 : 0;
}

long long ep_condvar_destroy(long long cv) {
    pthread_cond_destroy((pthread_cond_t*)cv);
    free((void*)cv);
    return 0;
}

/* ========== Regex (simple stub — delegates to POSIX regex) ========== */
#include <regex.h>

long long ep_regex_match(long long pattern_ptr, long long text_ptr) {
    regex_t regex;
    const char* pattern = (const char*)pattern_ptr;
    const char* text = (const char*)text_ptr;
    int ret = regcomp(&regex, pattern, REG_EXTENDED | REG_NOSUB);
    if (ret) return 0;
    ret = regexec(&regex, text, 0, NULL, 0);
    regfree(&regex);
    return ret == 0 ? 1 : 0;
}

long long ep_regex_find(long long pattern_ptr, long long text_ptr) {
    regex_t regex;
    regmatch_t match;
    const char* pattern = (const char*)pattern_ptr;
    const char* text = (const char*)text_ptr;
    int ret = regcomp(&regex, pattern, REG_EXTENDED);
    if (ret) return (long long)"";
    ret = regexec(&regex, text, 1, &match, 0);
    if (ret != 0) { regfree(&regex); return (long long)""; }
    int len = match.rm_eo - match.rm_so;
    char* result = (char*)malloc(len + 1);
    memcpy(result, text + match.rm_so, len);
    result[len] = '\0';
    regfree(&regex);
    return (long long)result;
}

long long ep_regex_find_all(long long pattern_ptr, long long text_ptr) {
    regex_t regex;
    regmatch_t match;
    const char* pattern = (const char*)pattern_ptr;
    const char* text = (const char*)text_ptr;
    long long list = create_list();
    int ret = regcomp(&regex, pattern, REG_EXTENDED);
    if (ret) return list;
    const char* cursor = text;
    while (regexec(&regex, cursor, 1, &match, 0) == 0) {
        int len = match.rm_eo - match.rm_so;
        char* result = (char*)malloc(len + 1);
        memcpy(result, cursor + match.rm_so, len);
        result[len] = '\0';
        append_list(list, (long long)result);
        cursor += match.rm_eo;
        if (match.rm_eo == 0) break;
    }
    regfree(&regex);
    return list;
}

long long ep_regex_replace(long long pattern_ptr, long long text_ptr, long long repl_ptr) {
    /* Simple single-replacement via regex */
    regex_t regex;
    regmatch_t match;
    const char* pattern = (const char*)pattern_ptr;
    const char* text = (const char*)text_ptr;
    const char* repl = (const char*)repl_ptr;
    int ret = regcomp(&regex, pattern, REG_EXTENDED);
    if (ret) return text_ptr;
    ret = regexec(&regex, text, 1, &match, 0);
    if (ret != 0) { regfree(&regex); return text_ptr; }
    size_t tlen = strlen(text);
    size_t rlen = strlen(repl);
    size_t new_len = tlen - (match.rm_eo - match.rm_so) + rlen;
    char* result = (char*)malloc(new_len + 1);
    memcpy(result, text, match.rm_so);
    memcpy(result + match.rm_so, repl, rlen);
    memcpy(result + match.rm_so + rlen, text + match.rm_eo, tlen - match.rm_eo);
    result[new_len] = '\0';
    regfree(&regex);
    return (long long)result;
}

long long ep_regex_split(long long pattern_ptr, long long text_ptr) {
    long long list = create_list();
    /* Simple split: find matches and split around them */
    regex_t regex;
    regmatch_t match;
    const char* pattern = (const char*)pattern_ptr;
    const char* text = (const char*)text_ptr;
    int ret = regcomp(&regex, pattern, REG_EXTENDED);
    if (ret) {
        append_list(list, text_ptr);
        return list;
    }
    const char* cursor = text;
    while (regexec(&regex, cursor, 1, &match, 0) == 0) {
        int len = match.rm_so;
        char* part = (char*)malloc(len + 1);
        memcpy(part, cursor, len);
        part[len] = '\0';
        append_list(list, (long long)part);
        cursor += match.rm_eo;
        if (match.rm_eo == 0) break;
    }
    char* rest = strdup(cursor);
    append_list(list, (long long)rest);
    regfree(&regex);
    return list;
}

/* ========== Base64 ========== */
static const char b64_table[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

long long ep_base64_encode(long long data_ptr) {
    const unsigned char* data = (const unsigned char*)data_ptr;
    size_t len = strlen((const char*)data);
    size_t out_len = 4 * ((len + 2) / 3);
    char* out = (char*)malloc(out_len + 1);
    size_t i, j = 0;
    for (i = 0; i < len; i += 3) {
        unsigned int n = data[i] << 16;
        if (i + 1 < len) n |= data[i+1] << 8;
        if (i + 2 < len) n |= data[i+2];
        out[j++] = b64_table[(n >> 18) & 63];
        out[j++] = b64_table[(n >> 12) & 63];
        out[j++] = (i + 1 < len) ? b64_table[(n >> 6) & 63] : '=';
        out[j++] = (i + 2 < len) ? b64_table[n & 63] : '=';
    }
    out[j] = '\0';
    return (long long)out;
}

long long ep_uuid_v4(void) {
    char* uuid = (char*)malloc(37);
    unsigned char bytes[16];
    ep_secure_random_bytes(bytes, 16);
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
    snprintf(uuid, 37, "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11],
        bytes[12], bytes[13], bytes[14], bytes[15]);
    return (long long)uuid;
}

long long file_read(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return (long long)strdup("");
    FILE* f = fopen(path, "rb");
    if (!f) return (long long)strdup("");
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    char* buf = malloc(size + 1);
    if (!buf) { fclose(f); return (long long)strdup(""); }
    fread(buf, 1, size, f);
    buf[size] = '\0';
    fclose(f);
    ep_gc_register(buf, EP_OBJ_STRING);
    return (long long)buf;
}

long long file_write(long long path_val, long long content_val) {
    const char* path = (const char*)path_val;
    const char* content = (const char*)content_val;
    if (!path || !content) return 0;
    FILE* f = fopen(path, "wb");
    if (!f) return 0;
    size_t len = strlen(content);
    fwrite(content, 1, len, f);
    fclose(f);
    return 1;
}

long long file_append(long long path_val, long long content_val) {
    const char* path = (const char*)path_val;
    const char* content = (const char*)content_val;
    if (!path || !content) return 0;
    FILE* f = fopen(path, "ab");
    if (!f) return 0;
    size_t len = strlen(content);
    fwrite(content, 1, len, f);
    fclose(f);
    return 1;
}

long long file_exists(long long path_val) {
    const char* path = (const char*)path_val;
    if (!path) return 0;
    FILE* f = fopen(path, "r");
    if (f) { fclose(f); return 1; }
    return 0;
}

long long string_contains(long long s_val, long long sub_val) {
    const char* s = (const char*)s_val;
    const char* sub = (const char*)sub_val;
    if (!s || !sub) return 0;
    return strstr(s, sub) != NULL ? 1 : 0;
}

long long string_index_of(long long s_val, long long sub_val) {
    const char* s = (const char*)s_val;
    const char* sub = (const char*)sub_val;
    if (!s || !sub) return -1;
    const char* found = strstr(s, sub);
    if (!found) return -1;
    return (long long)(found - s);
}

long long string_replace(long long s_val, long long old_val, long long new_val) {
    const char* s = (const char*)s_val;
    const char* old_str = (const char*)old_val;
    const char* new_str = (const char*)new_val;
    if (!s || !old_str || !new_str) return (long long)strdup(s ? s : "");
    size_t old_len = strlen(old_str);
    size_t new_len = strlen(new_str);
    if (old_len == 0) return (long long)strdup(s);
    int count = 0;
    const char* p = s;
    while ((p = strstr(p, old_str)) != NULL) { count++; p += old_len; }
    size_t result_len = strlen(s) + count * (new_len - old_len);
    char* result = malloc(result_len + 1);
    if (!result) return (long long)strdup(s);
    char* dst = result;
    p = s;
    while (*p) {
        if (strncmp(p, old_str, old_len) == 0) {
            memcpy(dst, new_str, new_len);
            dst += new_len;
            p += old_len;
        } else {
            *dst++ = *p++;
        }
    }
    *dst = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

/* ========== Additional String Functions ========== */
#include <ctype.h>

long long string_upper(long long s_val) {
    const char* s = (const char*)s_val;
    if (!s) return (long long)strdup("");
    long long len = strlen(s);
    char* result = malloc(len + 1);
    for (long long i = 0; i < len; i++) result[i] = toupper((unsigned char)s[i]);
    result[len] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long string_lower(long long s_val) {
    const char* s = (const char*)s_val;
    if (!s) return (long long)strdup("");
    long long len = strlen(s);
    char* result = malloc(len + 1);
    for (long long i = 0; i < len; i++) result[i] = tolower((unsigned char)s[i]);
    result[len] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long string_trim(long long s_val) {
    const char* s = (const char*)s_val;
    if (!s) return (long long)strdup("");
    while (*s && isspace((unsigned char)*s)) s++;
    long long len = strlen(s);
    while (len > 0 && isspace((unsigned char)s[len - 1])) len--;
    char* result = malloc(len + 1);
    memcpy(result, s, len);
    result[len] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long string_split(long long s_val, long long delim_val) {
    const char* s = (const char*)s_val;
    const char* delim = (const char*)delim_val;
    if (!s || !delim) return create_list();
    long long list = create_list();
    long long dlen = strlen(delim);
    if (dlen == 0) { append_list(list, s_val); return list; }
    const char* p = s;
    while (1) {
        const char* found = strstr(p, delim);
        long long partlen = found ? (found - p) : (long long)strlen(p);
        char* part = malloc(partlen + 1);
        memcpy(part, p, partlen);
        part[partlen] = '\0';
        ep_gc_register(part, EP_OBJ_STRING);
        append_list(list, (long long)part);
        if (!found) break;
        p = found + dlen;
    }
    return list;
}

long long char_at(long long s_val, long long index) {
    const char* s = (const char*)s_val;
    if (!s || index < 0 || index >= (long long)strlen(s)) return 0;
    return (unsigned char)s[index];
}

long long char_from_code(long long code) {
    char* result = malloc(2);
    result[0] = (char)code;
    result[1] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long ep_abs(long long n) {
    return n < 0 ? -n : n;
}

// Auto-convert any value to string for string interpolation
long long ep_auto_to_string(long long val) {
    // If the value is 0, return "0"
    if (val == 0) return (long long)strdup("0");
    // Check if val is a GC-tracked string (heap-allocated)
    EpGCObject* obj = ep_gc_find((void*)val);
    if (obj && obj->kind == EP_OBJ_STRING) {
        return val; // It's a known string pointer
    }
    // Check if val is a static string literal (in .rodata/.data segment)
    // These aren't GC-tracked but ARE valid pointers. Use a safe probe:
    // only dereference if the address is in a readable memory page.
    if (val > 0x100000) {
#if defined(_WIN32)
        // Windows: use VirtualQuery to safely probe pointer validity
        MEMORY_BASIC_INFORMATION mbi;
        if (VirtualQuery((void*)val, &mbi, sizeof(mbi)) && mbi.State == MEM_COMMIT && !(mbi.Protect & (PAGE_NOACCESS | PAGE_GUARD))) {
            const char* p = (const char*)(void*)val;
            unsigned char first = (unsigned char)*p;
            if ((first >= 0x20 && first <= 0x7E) || (first >= 0xC0 && first <= 0xFD) || first == '\n' || first == '\t' || first == '\r' || first == 0) {
                return val; // Readable memory, looks like a string
            }
        }
#elif defined(__APPLE__)
        // macOS: use vm_read_overwrite to safely probe
        char probe;
        vm_size_t sz = 1;
        kern_return_t kr = vm_read_overwrite(mach_task_self(), (mach_vm_address_t)val, 1, (mach_vm_address_t)&probe, &sz);
        if (kr == KERN_SUCCESS) {
            unsigned char first = (unsigned char)probe;
            if ((first >= 0x20 && first <= 0x7E) || (first >= 0xC0 && first <= 0xFD) || first == '\n' || first == '\t' || first == '\r' || first == 0) {
                return val; // Readable memory, looks like a string
            }
        }
#else
        // Linux: use write() to /dev/null as a safe pointer probe
        // write() returns -1 with EFAULT for invalid pointers, no signal
        int devnull = open("/dev/null", 1); // O_WRONLY
        if (devnull >= 0) {
            ssize_t r = write(devnull, (const void*)val, 1);
            close(devnull);
            if (r == 1) {
                const char* p = (const char*)(void*)val;
                unsigned char first = (unsigned char)*p;
                if ((first >= 0x20 && first <= 0x7E) || (first >= 0xC0 && first <= 0xFD) || first == '\n' || first == '\t' || first == '\r' || first == 0) {
                    return val;
                }
            }
        }
#endif
    }
    // Otherwise, convert integer to string
    char* buf = (char*)malloc(32);
    snprintf(buf, 32, "%lld", val);
    ep_gc_register(buf, EP_OBJ_STRING);
    return (long long)buf;
}

long long ep_random_int(long long min, long long max) {
    if (max <= min) return min;
    /* Draw from the OS CSPRNG with rejection sampling to avoid modulo bias. */
    unsigned long long range = (unsigned long long)(max - min) + 1ULL;
    unsigned long long limit = UINT64_MAX - (UINT64_MAX % range);
    unsigned long long r;
    do {
        ep_secure_random_bytes((unsigned char*)&r, sizeof(r));
    } while (r >= limit);
    return min + (long long)(r % range);
}

// JSON built-in functions
static const char* json_skip_ws(const char* p) {
    while (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r') p++;
    return p;
}

static const char* json_skip_value(const char* p) {
    p = json_skip_ws(p);
    if (*p == '"') {
        p++;
        while (*p && *p != '"') { if (*p == '\\') p++; p++; }
        if (*p == '"') p++;
    } else if (*p == '{') {
        int depth = 1; p++;
        while (*p && depth > 0) {
            if (*p == '"') { p++; while (*p && *p != '"') { if (*p == '\\') p++; p++; } if (*p) p++; }
            else if (*p == '{') { depth++; p++; }
            else if (*p == '}') { depth--; p++; }
            else p++;
        }
    } else if (*p == '[') {
        int depth = 1; p++;
        while (*p && depth > 0) {
            if (*p == '"') { p++; while (*p && *p != '"') { if (*p == '\\') p++; p++; } if (*p) p++; }
            else if (*p == '[') { depth++; p++; }
            else if (*p == ']') { depth--; p++; }
            else p++;
        }
    } else {
        while (*p && *p != ',' && *p != '}' && *p != ']' && *p != ' ' && *p != '\n') p++;
    }
    return p;
}

static const char* json_find_key(const char* json, const char* key) {
    const char* p = json_skip_ws(json);
    if (*p != '{') return NULL;
    p++;
    while (*p) {
        p = json_skip_ws(p);
        if (*p == '}') return NULL;
        if (*p != '"') return NULL;
        p++;
        const char* ks = p;
        while (*p && *p != '"') { if (*p == '\\') p++; p++; }
        size_t klen = p - ks;
        if (*p == '"') p++;
        p = json_skip_ws(p);
        if (*p == ':') p++;
        p = json_skip_ws(p);
        if (klen == strlen(key) && strncmp(ks, key, klen) == 0) {
            return p;
        }
        p = json_skip_value(p);
        p = json_skip_ws(p);
        if (*p == ',') p++;
    }
    return NULL;
}

long long json_get_string(long long json_val, long long key_val) {
    const char* json = (const char*)json_val;
    const char* key = (const char*)key_val;
    if (!json || !key) return (long long)strdup("");
    const char* val = json_find_key(json, key);
    if (!val || *val != '"') return (long long)strdup("");
    val++;
    const char* end = val;
    while (*end && *end != '"') { if (*end == '\\') end++; end++; }
    size_t len = end - val;
    char* result = (char*)malloc(len + 1);
    // Handle escape sequences
    size_t di = 0;
    const char* si = val;
    while (si < end) {
        if (*si == '\\' && si + 1 < end) {
            si++;
            switch (*si) {
                case 'n': result[di++] = '\n'; break;
                case 't': result[di++] = '\t'; break;
                case 'r': result[di++] = '\r'; break;
                case '"': result[di++] = '"'; break;
                case '\\': result[di++] = '\\'; break;
                default: result[di++] = *si; break;
            }
        } else {
            result[di++] = *si;
        }
        si++;
    }
    result[di] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long json_get_int(long long json_val, long long key_val) {
    const char* json = (const char*)json_val;
    const char* key = (const char*)key_val;
    if (!json || !key) return 0;
    const char* val = json_find_key(json, key);
    if (!val) return 0;
    return atoll(val);
}

long long json_get_bool(long long json_val, long long key_val) {
    const char* json = (const char*)json_val;
    const char* key = (const char*)key_val;
    if (!json || !key) return 0;
    const char* val = json_find_key(json, key);
    if (!val) return 0;
    if (strncmp(val, "true", 4) == 0) return 1;
    return 0;
}

// SHA-1 implementation (RFC 3174) for WebSocket handshake
static unsigned int sha1_left_rotate(unsigned int x, int n) {
    return (x << n) | (x >> (32 - n));
}

long long ep_sha1(long long data_val) {
    const unsigned char* data = (const unsigned char*)data_val;
    if (!data) return (long long)strdup("");
    size_t len = strlen((const char*)data);

    unsigned int h0 = 0x67452301, h1 = 0xEFCDAB89, h2 = 0x98BADCFE, h3 = 0x10325476, h4 = 0xC3D2E1F0;
    size_t new_len = len + 1;
    while (new_len % 64 != 56) new_len++;
    unsigned char* msg = (unsigned char*)calloc(new_len + 8, 1);
    memcpy(msg, data, len);
    msg[len] = 0x80;
    unsigned long long bits_len = (unsigned long long)len * 8;
    for (int i = 0; i < 8; i++) msg[new_len + 7 - i] = (unsigned char)(bits_len >> (i * 8));

    for (size_t offset = 0; offset < new_len + 8; offset += 64) {
        unsigned int w[80];
        for (int i = 0; i < 16; i++) {
            w[i] = ((unsigned int)msg[offset + i*4] << 24) | ((unsigned int)msg[offset + i*4+1] << 16) |
                    ((unsigned int)msg[offset + i*4+2] << 8) | (unsigned int)msg[offset + i*4+3];
        }
        for (int i = 16; i < 80; i++) w[i] = sha1_left_rotate(w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16], 1);
        unsigned int a = h0, b = h1, c = h2, d = h3, e = h4;
        for (int i = 0; i < 80; i++) {
            unsigned int f, k;
            if (i < 20) { f = (b & c) | ((~b) & d); k = 0x5A827999; }
            else if (i < 40) { f = b ^ c ^ d; k = 0x6ED9EBA1; }
            else if (i < 60) { f = (b & c) | (b & d) | (c & d); k = 0x8F1BBCDC; }
            else { f = b ^ c ^ d; k = 0xCA62C1D6; }
            unsigned int temp = sha1_left_rotate(a, 5) + f + e + k + w[i];
            e = d; d = c; c = sha1_left_rotate(b, 30); b = a; a = temp;
        }
        h0 += a; h1 += b; h2 += c; h3 += d; h4 += e;
    }
    free(msg);

    // Return Base64-encoded hash directly (for WebSocket handshake)
    unsigned char hash[20];
    hash[0] = (h0>>24)&0xFF; hash[1] = (h0>>16)&0xFF; hash[2] = (h0>>8)&0xFF; hash[3] = h0&0xFF;
    hash[4] = (h1>>24)&0xFF; hash[5] = (h1>>16)&0xFF; hash[6] = (h1>>8)&0xFF; hash[7] = h1&0xFF;
    hash[8] = (h2>>24)&0xFF; hash[9] = (h2>>16)&0xFF; hash[10] = (h2>>8)&0xFF; hash[11] = h2&0xFF;
    hash[12] = (h3>>24)&0xFF; hash[13] = (h3>>16)&0xFF; hash[14] = (h3>>8)&0xFF; hash[15] = h3&0xFF;
    hash[16] = (h4>>24)&0xFF; hash[17] = (h4>>16)&0xFF; hash[18] = (h4>>8)&0xFF; hash[19] = h4&0xFF;

    // Base64 encode the 20-byte hash
    size_t b64_len = 4 * ((20 + 2) / 3);
    char* result = (char*)malloc(b64_len + 1);
    size_t j = 0;
    for (size_t bi = 0; bi < 20; bi += 3) {
        unsigned int n2 = ((unsigned int)hash[bi]) << 16;
        if (bi + 1 < 20) n2 |= ((unsigned int)hash[bi+1]) << 8;
        if (bi + 2 < 20) n2 |= (unsigned int)hash[bi+2];
        result[j++] = b64_table[(n2 >> 18) & 0x3F];
        result[j++] = b64_table[(n2 >> 12) & 0x3F];
        result[j++] = (bi + 1 < 20) ? b64_table[(n2 >> 6) & 0x3F] : '=';
        result[j++] = (bi + 2 < 20) ? b64_table[n2 & 0x3F] : '=';
    }
    result[j] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

// Read exact N bytes from a socket
#ifdef __wasm__
long long ep_net_recv_bytes(long long fd, long long count) {
    (void)fd; (void)count;
    return (long long)strdup("");
}
#else
long long ep_net_recv_bytes(long long fd, long long count) {
    if (count <= 0) return (long long)strdup("");
    char* buf = (char*)malloc(count + 1);
#ifdef _WIN32
    int total = 0;
    while (total < (int)count) {
        int n = recv((int)fd, buf + total, (int)(count - total), 0);
        if (n <= 0) break;
        total += n;
    }
#else
    ssize_t total = 0;
    while (total < count) {
        ssize_t n = recv((int)fd, buf + total, count - total, 0);
        if (n <= 0) break;
        total += n;
    }
#endif
    buf[total] = '\0';
    ep_gc_register(buf, EP_OBJ_STRING);
    return (long long)buf;
}
#endif

long long ep_get_args(void) {
    long long list_ptr = create_list();
    for (int i = 0; i < ep_argc; i++) {
        char* arg_copy = strdup(ep_argv[i]);
        ep_gc_register(arg_copy, EP_OBJ_STRING);
        append_list(list_ptr, (long long)arg_copy);
    }
    return list_ptr;
}

"#;

#[allow(dead_code)]
const C_MAIN_BOOTSTRAPPER: &str = r#"
/* Bootstrapper C main */
int main(int argc, char** argv) {
    {
        unsigned int seed;
        FILE* urand = fopen("/dev/urandom", "rb");
        if (urand && fread(&seed, sizeof(seed), 1, urand) == 1) {
            fclose(urand);
        } else {
            if (urand) fclose(urand);
            seed = (unsigned int)time(NULL) ^ (unsigned int)getpid();
        }
        srand(seed);
    }
    init_ep_args(argc, argv);
    int result = (int)_main();
    ep_gc_shutdown();
    return result;
}
"#;
