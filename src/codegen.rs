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
    matches!(t, Type::List | Type::Str | Type::DynStr | Type::RefList | Type::RefStr | Type::Struct(_) | Type::Enum(_))
}

pub struct Codegen {
    out: String,
    func_return_types: HashMap<String, Type>,
    current_return_type: Type,
    spawn_index: usize,
    pub is_test_mode: bool,
    struct_defs: HashMap<String, StructDef>,
    enum_defs: HashMap<String, EnumDef>,
    /// Maps variant name -> enum name for quick lookup
    variant_to_enum: HashMap<String, String>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            func_return_types: HashMap::new(),
            current_return_type: Type::Int,
            spawn_index: 0,
            is_test_mode: false,
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            variant_to_enum: HashMap::new(),
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

    fn type_annotation_to_type(&self, ann: &TypeAnnotation) -> Type {
        match ann {
            TypeAnnotation::Int => Type::Int,
            TypeAnnotation::Float => Type::Float,
            TypeAnnotation::Bool => Type::Bool,
            TypeAnnotation::Str => Type::Str,
            TypeAnnotation::DynStr => Type::DynStr,
            TypeAnnotation::List => Type::List,
            TypeAnnotation::UserDefined(name) => {
                if self.enum_defs.contains_key(name) {
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
        self.func_return_types.insert("ep_net_connect".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_listen".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_accept".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_send".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_recv".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_net_close".to_string(), Type::Int);
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
        self.func_return_types.insert("get_list_data_ptr".to_string(), Type::Int);
        self.func_return_types.insert("sqlite_get_callback_ptr".to_string(), Type::Int);
        self.func_return_types.insert("free_list".to_string(), Type::Int);
        self.func_return_types.insert("create_map".to_string(), Type::Int);
        self.func_return_types.insert("map_insert".to_string(), Type::Int);
        self.func_return_types.insert("map_get_val".to_string(), Type::Int);
        self.func_return_types.insert("map_contains".to_string(), Type::Int);
        self.func_return_types.insert("map_delete".to_string(), Type::Int);
        self.func_return_types.insert("free_map".to_string(), Type::Int);
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
        self.func_return_types.insert("concat".to_string(), Type::DynStr);
        self.func_return_types.insert("int_to_string".to_string(), Type::DynStr);
        self.func_return_types.insert("read_line".to_string(), Type::DynStr);
        self.func_return_types.insert("read_int".to_string(), Type::Int);
        self.func_return_types.insert("read_float".to_string(), Type::Float);
        self.func_return_types.insert("int_to_float".to_string(), Type::Float);
        self.func_return_types.insert("float_to_int".to_string(), Type::Int);

        for ext in &program.externals {
            if let Some(ref rt) = ext.return_type {
                self.func_return_types.insert(ext.name.clone(), self.type_annotation_to_type(rt));
            } else if !self.func_return_types.contains_key(&ext.name) {
                self.func_return_types.insert(ext.name.clone(), Type::Int);
            }
        }

        // 3 passes for resolution of dependencies/mutual calls
        for _ in 0..3 {
            for func in &program.functions {
                // If the function has an explicit return type annotation, use it
                if let Some(ref rt) = func.return_type {
                    self.func_return_types.insert(func.name.clone(), self.type_annotation_to_type(rt));
                    continue;
                }

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
                
                let ret = self.determine_ret_type(&func.body, &var_types).unwrap_or(Type::Int);
                self.func_return_types.insert(func.name.clone(), ret);
            }
        }
    }

    fn collect_var_types(&self, stmts: &[Stmt], var_types: &mut HashMap<String, Type>) {
        for stmt in stmts {
            match &stmt.node {
                StmtNode::Set(name, expr, type_ann) => {
                    let t = if let Some(ann) = type_ann {
                        self.type_annotation_to_type(ann)
                    } else {
                        self.infer_type(expr, var_types)
                    };
                    var_types.insert(name.clone(), t);
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
                StmtNode::ForEach(loop_var, _iterable, body) => {
                    // The loop variable is always an Int (element from list or range index)
                    var_types.insert(loop_var.clone(), Type::Int);
                    self.collect_var_types(body, var_types);
                }
                StmtNode::Match(expr, arms) => {
                    let enum_type = self.infer_type(expr, var_types);
                    if let Type::Enum(enum_name) = &enum_type {
                        if let Some(ed) = self.enum_defs.get(enum_name) {
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
                        for (fname, ftype) in &sd.fields {
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
                if let Type::Struct(struct_name) = &obj_type {
                    let key = format!("{}_{}", struct_name, method_name);
                    self.func_return_types.get(&key).cloned().unwrap_or(Type::Int)
                } else {
                    Type::Int
                }
            }
            ExprNode::UnaryNot(_) => Type::Int,
            ExprNode::TryExpr(inner) => {
                let inner_type = self.infer_type(inner, var_types);
                match inner_type {
                    _ => Type::Int,
                }
            }
            ExprNode::Closure(_, _) => Type::Int, // closure is a function pointer (long long)
            ExprNode::Await(inner) => self.infer_type(inner, var_types),
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
                                        if src_t != Type::RefList && src_t != Type::RefStr {
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
                            if src_t != Type::RefList && src_t != Type::RefStr {
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
                                if src_t != Type::RefList && src_t != Type::RefStr {
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
                let t = var_types.get(name);
                let expr_str = self.gen_expr(expr, var_types)?;

                if t == Some(&Type::List) {
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        long long tmp_val = {};\n", expr_str));
                    self.out.push_str(&format!("        free_list({});\n", name));
                    self.out.push_str(&format!("        {} = tmp_val;\n", name));
                    self.out.push_str("    }\n");
                } else if let Some(Type::Enum(ename)) = t {
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        long long tmp_val = {};\n", expr_str));
                    self.out.push_str(&format!("        free_enum_{}({});\n", ename, name));
                    self.out.push_str(&format!("        {} = tmp_val;\n", name));
                    self.out.push_str("    }\n");
                } else {
                    self.out.push_str(&format!("    {} = {};\n", name, expr_str));
                }
            }
            StmtNode::Return(expr) => {
                let expr_str = self.gen_expr(expr, var_types)?;
                self.out.push_str(&format!("    ret_val = {};\n", expr_str));

                if let ExprNode::Identifier(name) = &expr.node {
                    let t = var_types.get(name);
                    if t == Some(&Type::List) {
                        self.out.push_str(&format!("    {} = 0;\n", name));
                    } else if matches!(t, Some(Type::Enum(_))) {
                        self.out.push_str(&format!("    {} = 0;\n", name));
                    }
                }
                self.out.push_str("    goto L_cleanup;\n");
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
                        self.out.push_str(&format!("    printf(\"%lld\\n\", (long long){});\n", expr_str));
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
                let idx = self.spawn_index;
                self.spawn_index += 1;
                self.out.push_str("    {\n");
                self.out.push_str(&format!("        spawn_args_{}* s_args = malloc(sizeof(spawn_args_{}));\n", idx, idx));
                for (j, arg) in args.iter().enumerate() {
                    let arg_str = self.gen_expr(arg, var_types)?;
                    self.out.push_str(&format!("        s_args->arg{} = {};\n", j, arg_str));
                }
                self.out.push_str("        pthread_t t;\n");
                self.out.push_str(&format!("        int rc = pthread_create(&t, NULL, spawn_wrapper_{}, s_args);\n", idx));
                self.out.push_str("        if (rc != 0) { printf(\"DEBUG: pthread_create failed: %d\\n\", rc); }\n");
                self.out.push_str("        pthread_detach(t);\n");
                self.out.push_str("    }\n");
            }
            StmtNode::Send(chan, val) => {
                let chan_str = self.gen_expr(chan, var_types)?;
                let val_str = self.gen_expr(val, var_types)?;
                self.out.push_str(&format!("    send_channel({}, {});\n", chan_str, val_str));
                if let ExprNode::Identifier(name) = &val.node {
                    if var_types.get(name) == Some(&Type::List) {
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
                } else {
                    return Err("Match/check statement on non-enum type".to_string());
                }
            }
            StmtNode::ForEach(loop_var, iterable, body) => {
                if let ExprNode::Call(func_name, args) = &iterable.node {
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
                
                let is_string = self.infer_type(left, var_types) == Type::Str || self.infer_type(left, var_types) == Type::DynStr;
                if is_string {
                    let cmp_op = match op {
                        CompOp::LessThan => "< 0",
                        CompOp::GreaterThan => "> 0",
                        CompOp::LessEqual => "<= 0",
                        CompOp::GreaterEqual => ">= 0",
                        CompOp::Equals => "== 0",
                        CompOp::NotEquals => "!= 0",
                    };
                    Ok(format!("(strcmp((char*){}, (char*){}) {})", left_str, right_str, cmp_op))
                } else {
                    let op_str = match op {
                        CompOp::LessThan => "<",
                        CompOp::GreaterThan => ">",
                        CompOp::LessEqual => "<=",
                        CompOp::GreaterEqual => ">=",
                        CompOp::Equals => "==",
                        CompOp::NotEquals => "!=",
                    };
                    Ok(format!("{} {} {}", left_str, op_str, right_str))
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
                        "get_character" | "substring" if i == 0 => {
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
                if var_types.contains_key(name) && !self.func_return_types.contains_key(name) {
                    // It's a variable, not a known function — treat as closure call
                    let arg_types: Vec<&str> = args.iter().map(|_| "long long").collect();
                    let fn_type = format!("long long(*)({})", if arg_types.is_empty() { "void".to_string() } else { arg_types.join(", ") });
                    Ok(format!("(({}){})({})", fn_type, name, formatted_args.join(", ")))
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
                    Err(format!("Field access on non-struct type"))
                }
            }
            ExprNode::StructCreate(struct_name, fields) => {
                let c_name = format!("EpStruct_{}", struct_name);
                let num_fields = fields.len();
                let mut lines = Vec::new();
                lines.push(format!("({{"));
                lines.push(format!("    {}* _s = ({}*)malloc(sizeof({}));", c_name, c_name, c_name));
                for (fname, fexpr) in fields {
                    let fval = self.gen_expr(fexpr, var_types)?;
                    lines.push(format!("    _s->{} = {};", fname, fval));
                }
                lines.push(format!("    {{ EpGCObject* _go = ep_gc_register(_s, EP_OBJ_STRUCT); if(_go) _go->num_fields = {}; }}", num_fields));
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
                    _ => return Err(format!("Method call on non-struct type")),
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
                // Simple try: evaluate the inner expression directly. 
                // If it causes a signal (SIGFPE, SIGSEGV), ep_try catches it via setjmp.
                Ok(format!(
                    "({{ volatile long long _try_r_{id} = 0; \
                    if (setjmp(ep_try_buf) == 0) {{ \
                        ep_try_active = 1; \
                        _try_r_{id} = {inner}; \
                        ep_try_active = 0; \
                    }} else {{ \
                        _try_r_{id} = 0; \
                    }} \
                    _try_r_{id}; }})",
                    id = try_id,
                    inner = inner_str
                ))
            }
            ExprNode::Await(inner) => {
                let inner_str = self.gen_expr(inner, var_types)?;
                Ok(format!("({{ EpFuture* _fut = (EpFuture*){}; long long _res = 0; if (_fut) {{ if (!_fut->completed) {{ _res = receive_channel(_fut->chan); }} else {{ _res = _fut->value; }} }} _res; }})", inner_str))
            }
            ExprNode::Closure(params, body) => {
                // Generate a static closure function
                let closure_name = format!("_ep_closure_{}", self.spawn_index);
                self.spawn_index += 1;

                // Build parameter list for the C function
                let c_params: Vec<String> = params.iter()
                    .map(|p| format!("long long {}", p))
                    .collect();
                let c_param_str = if c_params.is_empty() {
                    "void".to_string()
                } else {
                    c_params.join(", ")
                };

                // Generate the closure body using buffer swapping
                let mut closure_var_types = HashMap::new();
                for p in params {
                    closure_var_types.insert(p.clone(), Type::Int);
                }

                // Save current output, generate closure into fresh buffer
                let saved_out = std::mem::take(&mut self.out);

                self.out.push_str(&format!("long long {}({}) {{\n", closure_name, c_param_str));
                self.out.push_str("    long long ret_val = 0;\n");
                for stmt in body {
                    self.gen_statement(stmt, &closure_var_types)?;
                }
                self.out.push_str("L_cleanup:\n");
                self.out.push_str("    return ret_val;\n");
                self.out.push_str("}\n\n");

                // Prepend closure function, then restore the main output
                let closure_code = std::mem::take(&mut self.out);
                self.out = closure_code + &saved_out;

                // Return the function pointer as a long long
                Ok(format!("(long long){}", closure_name))
            }
        }
    }
}

// ========== analyze_safety, generate, gen_function ==========

impl Codegen {
    fn analyze_safety(&self, program: &Program) -> Result<(), String> {
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
        lines.push("#include <sys/types.h>\n".to_string());
        lines.push("#include <sys/wait.h>\n".to_string());
        lines.push("#include <unistd.h>\n".to_string());
        lines.push("#include <stdio.h>\n".to_string());
        lines.push("#include <stdlib.h>\n\n".to_string());
        
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
        lines.push("}\n\n".to_string());
        
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

        self.analyze_return_types(program);
        self.analyze_safety(program)?;

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
                for (fname, ftype) in &sd.fields {
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
                self.out.push_str(&format!("    EpStruct_{}* s = (EpStruct_{}*)ptr;\n", sd.name, sd.name));
                for (fname, ftype) in &sd.fields {
                    match ftype {
                        TypeAnnotation::List => {
                            self.out.push_str(&format!("    free_list(s->{});\n", fname));
                        }
                        TypeAnnotation::DynStr => {
                            self.out.push_str(&format!("    if (s->{}) free((void*)s->{});\n", fname, fname));
                        }
                        TypeAnnotation::UserDefined(inner_name) => {
                            self.out.push_str(&format!("    free_struct_{}(s->{});\n", inner_name, fname));
                        }
                        TypeAnnotation::Generic(inner_name, _) => {
                            self.out.push_str(&format!("    free_struct_{}(s->{});\n", inner_name, fname));
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
                                    if self.enum_defs.contains_key(inner_name) {
                                        self.out.push_str(&format!("        free_enum_{}(e->data{});\n", inner_name, j));
                                    } else {
                                        self.out.push_str(&format!("        free_struct_{}(e->data{});\n", inner_name, j));
                                    }
                                }
                                TypeAnnotation::Generic(inner_name, _) => {
                                    if self.enum_defs.contains_key(inner_name) {
                                        self.out.push_str(&format!("        free_enum_{}(e->data{});\n", inner_name, j));
                                    } else {
                                        self.out.push_str(&format!("        free_struct_{}(e->data{});\n", inner_name, j));
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
            let mut params_str = Vec::new();
            for _ in &ext.params {
                params_str.push("long long");
            }
            self.out.push_str(&format!("long long {}({});\n", ext.name, params_str.join(", ")));
        }
        self.out.push_str("\n");

        self.out.push_str("\n/* User Function Prototypes */\n");
        for func in &program.functions {
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

        for func in &program.functions {
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
                };
                self.gen_method(&md)?;
            }
        }

        if self.is_test_mode {
            self.out.push_str(&self.get_c_test_main_source(program));
        } else {
            self.out.push_str(C_MAIN_BOOTSTRAPPER);
        }

        Ok(self.out.clone())
    }

    fn gen_function(&mut self, func: &Function) -> Result<(), String> {
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
            // 1. Generate the thread argument struct
            self.out.push_str(&format!("typedef struct {{\n"));
            self.out.push_str("    EpFuture* fut;\n");
            for (j, _param) in func.params.iter().enumerate() {
                self.out.push_str(&format!("    long long arg{};\n", j));
            }
            if func.params.is_empty() {
                self.out.push_str("    int dummy;\n");
            }
            self.out.push_str(&format!("}} {}_async_args;\n\n", name));

            // 2. Generate wrapper function
            self.out.push_str(&format!("void* {}_async_wrapper(void* r) {{\n", name));
            self.out.push_str("    int stack_dummy;\n");
            self.out.push_str("    ep_gc_register_thread(&stack_dummy);\n");
            self.out.push_str(&format!("    {}_async_args* args = ({}_async_args*)r;\n", name, name));
            
            let mut args_call = Vec::new();
            for j in 0..func.params.len() {
                args_call.push(format!("args->arg{}", j));
            }
            self.out.push_str(&format!("    long long res = {}_impl({});\n", name, args_call.join(", ")));
            self.out.push_str("    args->fut->value = res;\n");
            self.out.push_str("    args->fut->completed = 1;\n");
            self.out.push_str("    send_channel(args->fut->chan, res);\n");
            self.out.push_str("    free(args);\n");
            self.out.push_str("    ep_gc_unregister_thread();\n");
            self.out.push_str("    return NULL;\n");
            self.out.push_str("}\n\n");

            // 3. Generate public main function
            self.out.push_str(&format!("long long {}({}) {{\n", name, params_decl.join(", ")));
            self.out.push_str("    EpFuture* fut = (EpFuture*)malloc(sizeof(EpFuture));\n");
            self.out.push_str("    fut->chan = create_channel();\n");
            self.out.push_str("    fut->completed = 0;\n");
            self.out.push_str("    fut->value = 0;\n");
            self.out.push_str("    ep_gc_register(fut, EP_OBJ_STRUCT);\n");
            self.out.push_str(&format!("    {}_async_args* args = ({}_async_args*)malloc(sizeof({}_async_args));\n", name, name, name));
            self.out.push_str("    args->fut = fut;\n");
            for (j, param) in func.params.iter().enumerate() {
                self.out.push_str(&format!("    args->arg{} = {};\n", j, param.0));
            }
            self.out.push_str("    pthread_t thread;\n");
            self.out.push_str(&format!("    pthread_create(&thread, NULL, {}_async_wrapper, args);\n", name));
            self.out.push_str("    pthread_detach(thread);\n");
            self.out.push_str("    return (long long)fut;\n");
            self.out.push_str("}\n\n");
        }

        let impl_name = if func.is_async {
            format!("{}_impl", name)
        } else {
            name.clone()
        };
        self.out.push_str(&format!("long long {}({}) {{\n", impl_name, params_decl.join(", ")));
        
        for (var_name, _) in &var_types {
            let is_param = func.params.iter().any(|p| &p.0 == var_name);
            if !is_param {
                self.out.push_str(&format!("    long long {} = 0;\n", var_name));
            }
        }
        self.out.push_str("    long long ret_val = 0;\n\n");

        // Push GC roots for all tracked locals
        let mut gc_root_count = 0;
        for (var_name, _) in &var_types {
            let is_param = func.params.iter().any(|p| &p.0 == var_name);
            if !is_param {
                let t = var_types.get(var_name);
                let is_tracked = matches!(t, Some(Type::List) | Some(Type::DynStr) | Some(Type::Struct(_)) | Some(Type::Enum(_)));
                if is_tracked {
                    self.out.push_str(&format!("    ep_gc_push_root(&{});\n", var_name));
                    gc_root_count += 1;
                }
            }
        }
        // Also push params that are tracked
        for param in &func.params {
            let t = var_types.get(&param.0);
            let is_tracked = matches!(t, Some(Type::List) | Some(Type::DynStr) | Some(Type::Struct(_)) | Some(Type::Enum(_)));
            if is_tracked {
                self.out.push_str(&format!("    ep_gc_push_root(&{});\n", param.0));
                gc_root_count += 1;
            }
        }
        if gc_root_count > 0 {
            self.out.push_str("\n");
        }

        // GC safe point: collect only if this function uses heap-allocated data
        let needs_gc = gc_root_count > 0 || var_types.values().any(|t| 
            matches!(t, Type::List | Type::DynStr | Type::Struct(_) | Type::Enum(_) | Type::RefList)
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
        for (var_name, _) in &var_types {
            let is_param = func.params.iter().any(|p| &p.0 == var_name);
            if !is_param {
                let t = var_types.get(var_name);
                if t == Some(&Type::List) {
                    self.out.push_str(&format!("    free_list({});\n", var_name));
                } else if let Some(Type::Struct(sname)) = t {
                    self.out.push_str(&format!("    free_struct_{}({});\n", sname, var_name));
                } else if let Some(Type::Enum(ename)) = t {
                    self.out.push_str(&format!("    free_enum_{}({});\n", ename, var_name));
                }
            }
        }
        self.out.push_str("    return ret_val;\n}\n\n");

        Ok(())
    }

    fn gen_method(&mut self, md: &MethodDef) -> Result<(), String> {
        let mut var_types = HashMap::new();
        var_types.insert("self".to_string(), Type::Struct(md.struct_name.clone()));

        for param in &md.params {
            let param_type = if let Some(ref ann) = param.2 {
                self.type_annotation_to_type(ann)
            } else if param.1 {
                Type::RefList
            } else {
                Type::Int
            };
            var_types.insert(param.0.clone(), param_type);
        }
        self.collect_var_types(&md.body, &mut var_types);

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
                self.out.push_str(&format!("    long long {} = 0;\n", var_name));
            }
        }
        self.out.push_str("    long long ret_val = 0;\n\n");

        for stmt in &md.body {
            self.gen_statement(stmt, &var_types)?;
        }

        self.out.push_str("L_cleanup:\n");
        self.out.push_str("    return ret_val;\n}\n\n");

        Ok(())
    }
}

const RUNTIME_HEADER_AND_SRC: &str = r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#include <signal.h>
#ifndef _WIN32
#include <unistd.h>
#endif

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
    _exit(128 + sig);
}

__attribute__((constructor))
static void ep_install_signal_handlers(void) {
    signal(SIGFPE, ep_signal_handler);
    signal(SIGSEGV, ep_signal_handler);
    signal(SIGABRT, ep_signal_handler);
}

#ifdef _WIN32
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
#include <pthread.h>

typedef enum {
    EP_OBJ_LIST,
    EP_OBJ_STRING,
    EP_OBJ_STRUCT,
    EP_OBJ_CLOSURE
} EpObjKind;

typedef struct EpGCObject {
    EpObjKind kind;
    int marked;
    void* ptr;                /* actual allocation pointer */
    long long size;           /* payload size for structs */
    long long num_fields;     /* number of fields for structs (each is long long) */
    struct EpGCObject* next;  /* intrusive linked list */
} EpGCObject;

typedef struct {
    long long chan;
    int completed;
    long long value;
} EpFuture;

/* GC globals */
static EpGCObject* ep_gc_head = NULL;
static long long ep_gc_count = 0;
static long long ep_gc_threshold = 256;
static int ep_gc_enabled = 1;
static pthread_mutex_t ep_gc_mutex = PTHREAD_MUTEX_INITIALIZER;

/* Thread registry for conservative stack scanning in multi-threaded environment */
#define EP_MAX_THREADS 256
static __thread void* volatile ep_thread_local_top = NULL;
static __thread void* ep_thread_local_bottom = NULL;

static void* volatile* ep_thread_tops[EP_MAX_THREADS];
static void* ep_thread_bottoms[EP_MAX_THREADS];
static int ep_thread_active[EP_MAX_THREADS];
static int ep_num_threads = 0;
static pthread_mutex_t ep_thread_registry_mutex = PTHREAD_MUTEX_INITIALIZER;

static void ep_gc_register_thread(void* stack_bottom) {
    ep_thread_local_bottom = stack_bottom;
    ep_thread_local_top = stack_bottom;
    
    pthread_mutex_lock(&ep_thread_registry_mutex);
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
        ep_thread_active[slot] = 1;
    }
    pthread_mutex_unlock(&ep_thread_registry_mutex);
}

static void ep_gc_unregister_thread(void) {
    pthread_mutex_lock(&ep_thread_registry_mutex);
    pthread_t self = pthread_self();
    for (int i = 0; i < ep_num_threads; i++) {
        if (ep_thread_active[i] && ep_thread_tops[i] == &ep_thread_local_top) {
            ep_thread_active[i] = 0;
            break;
        }
    }
    pthread_mutex_unlock(&ep_thread_registry_mutex);
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

static void ep_gc_table_insert(void* key, EpGCObject* value) {
    if (ep_gc_table_size * 2 >= ep_gc_table_cap) {
        long long old_cap = ep_gc_table_cap;
        long long new_cap = old_cap == 0 ? 512 : old_cap * 2;
        EpGCEntry* new_table = (EpGCEntry*)calloc(new_cap, sizeof(EpGCEntry));
        for (long long i = 0; i < old_cap; i++) {
            if (ep_gc_table[i].key != NULL) {
                long long idx = ((uintptr_t)ep_gc_table[i].key) % new_cap;
                while (new_table[idx].key != NULL) {
                    idx = (idx + 1) % new_cap;
                }
                new_table[idx] = ep_gc_table[i];
            }
        }
        free(ep_gc_table);
        ep_gc_table = new_table;
        ep_gc_table_cap = new_cap;
    }
    
    long long idx = ((uintptr_t)key) % ep_gc_table_cap;
    while (ep_gc_table[idx].key != NULL) {
        if (ep_gc_table[idx].key == key) {
            ep_gc_table[idx].value = value;
            return;
        }
        idx = (idx + 1) % ep_gc_table_cap;
    }
    ep_gc_table[idx].key = key;
    ep_gc_table[idx].value = value;
    ep_gc_table_size++;
}

static EpGCObject* ep_gc_table_get(void* key) {
    if (ep_gc_table_cap == 0) return NULL;
    long long idx = ((uintptr_t)key) % ep_gc_table_cap;
    while (ep_gc_table[idx].key != NULL) {
        if (ep_gc_table[idx].key == key) return ep_gc_table[idx].value;
        idx = (idx + 1) % ep_gc_table_cap;
    }
    return NULL;
}

static void ep_gc_table_remove(void* key) {
    if (ep_gc_table_cap == 0) return;
    long long idx = ((uintptr_t)key) % ep_gc_table_cap;
    while (ep_gc_table[idx].key != NULL) {
        if (ep_gc_table[idx].key == key) {
            ep_gc_table[idx].key = NULL;
            ep_gc_table[idx].value = NULL;
            ep_gc_table_size--;
            long long next_idx = (idx + 1) % ep_gc_table_cap;
            while (ep_gc_table[next_idx].key != NULL) {
                void* rehash_key = ep_gc_table[next_idx].key;
                EpGCObject* rehash_val = ep_gc_table[next_idx].value;
                ep_gc_table[next_idx].key = NULL;
                ep_gc_table[next_idx].value = NULL;
                ep_gc_table_size--;
                ep_gc_table_insert(rehash_key, rehash_val);
                next_idx = (next_idx + 1) % ep_gc_table_cap;
            }
            return;
        }
        idx = (idx + 1) % ep_gc_table_cap;
    }
}

/* Dummy shadow stack API for compatibility with compiler-generated code */
static void ep_gc_push_root(long long* root) { (void)root; }
static void ep_gc_pop_roots(long long count) { (void)count; }

/* Register a new GC object */
static EpGCObject* ep_gc_register(void* ptr, EpObjKind kind) {
    if (!ptr) return NULL;
    pthread_mutex_lock(&ep_gc_mutex);
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
    obj->next = ep_gc_head;
    ep_gc_head = obj;
    ep_gc_count++;
    ep_gc_table_insert(ptr, obj);
    pthread_mutex_unlock(&ep_gc_mutex);
    return obj;
}

/* Find GC object by pointer */
static EpGCObject* ep_gc_find(void* ptr) {
    return ep_gc_table_get(ptr);
}

/* Forward declarations for list type (needed by GC mark) */
typedef struct {
    long long* data;
    long long length;
    long long capacity;
} EpList;

/* Mark a single object and recursively mark its children */
static void ep_gc_mark_object(void* ptr) {
    if (!ptr) return;
    EpGCObject* obj = ep_gc_find(ptr);
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
    }
}

/* Mark phase: traverse from stack roots of all registered threads */
static void ep_gc_mark(void) {
    jmp_buf regs;
    memset(&regs, 0, sizeof(regs));
    setjmp(regs); /* Spill registers of the current thread */
    
    // Update stack top of current thread
    volatile void* stack_top;
    stack_top = (void*)&stack_top;
    ep_thread_local_top = (void*)stack_top;
    
    pthread_mutex_lock(&ep_thread_registry_mutex);
    for (int i = 0; i < ep_num_threads; i++) {
        if (ep_thread_active[i]) {
            void** start = (void**)*ep_thread_tops[i];
            void** end = (void**)ep_thread_bottoms[i];
            if (start && end) {
                if (start > end) {
                    void** tmp = start;
                    start = end;
                    end = tmp;
                }
                for (void** cur = start; cur < end; cur++) {
                    void* ptr = *cur;
                    if (ptr) {
                        ep_gc_mark_object(ptr);
                    }
                }
            }
        }
    }
    pthread_mutex_unlock(&ep_thread_registry_mutex);
}

/* Sweep phase: free unmarked objects */
static void ep_gc_sweep(void) {
    EpGCObject** cur = &ep_gc_head;
    while (*cur) {
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
            }

            free(garbage);
            ep_gc_count--;
        } else {
            (*cur)->marked = 0;  /* reset for next cycle */
            cur = &(*cur)->next;
        }
    }
}

/* Run a full GC collection */
static void ep_gc_collect(void) {
    if (!ep_gc_enabled) return;
    ep_gc_mark();
    ep_gc_sweep();
    ep_gc_threshold = ep_gc_count * 2;
    if (ep_gc_threshold < 256) ep_gc_threshold = 256;
}

/* Maybe trigger GC if we've exceeded threshold */
static void ep_gc_maybe_collect(void) {
    EP_GC_UPDATE_TOP();
    pthread_mutex_lock(&ep_gc_mutex);
    if (ep_gc_count >= ep_gc_threshold) {
        ep_gc_collect();
    }
    pthread_mutex_unlock(&ep_gc_mutex);
}

/* Unregister an object (for explicit free — removes from GC tracking) */
static void ep_gc_unregister(void* ptr) {
    if (!ptr) return;
    pthread_mutex_lock(&ep_gc_mutex);
    ep_gc_table_remove(ptr);
    EpGCObject** cur = &ep_gc_head;
    while (*cur) {
        if ((*cur)->ptr == ptr) {
            EpGCObject* found = *cur;
            *cur = found->next;
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
    EpGCObject* cur = ep_gc_head;
    while (cur) {
        EpGCObject* next = cur->next;
        if (cur->kind == EP_OBJ_LIST) {
            EpList* list = (EpList*)cur->ptr;
            if (list) { free(list->data); free(list); }
        } else {
            free(cur->ptr);
        }
        free(cur);
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
char* string_from_list(long long list_ptr);
long long string_length(const char* s);
long long display_string(const char* s);

typedef struct {
    long long* data;
    long long capacity;
    long long head;
    long long tail;
    long long size;
    ep_mutex_t mutex;
    ep_cond_t cond_recv;
    ep_cond_t cond_send;
} EpChannel;

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
    return (long long)chan;
}

long long send_channel(long long chan_ptr, long long value) {
    EpChannel* chan = (EpChannel*)chan_ptr;
    if (!chan) return 0;
    ep_mutex_lock(&chan->mutex);
    while (chan->size >= chan->capacity) {
        ep_cond_wait(&chan->cond_send, &chan->mutex);
    }
    chan->data[chan->tail] = value;
    chan->tail = (chan->tail + 1) % chan->capacity;
    chan->size += 1;
    ep_cond_signal(&chan->cond_recv);
    ep_mutex_unlock(&chan->mutex);
    return value;
}

long long receive_channel(long long chan_ptr) {
    EpChannel* chan = (EpChannel*)chan_ptr;
    if (!chan) return 0;
    ep_mutex_lock(&chan->mutex);
    while (chan->size <= 0) {
        ep_cond_wait(&chan->cond_recv, &chan->mutex);
    }
    long long value = chan->data[chan->head];
    chan->head = (chan->head + 1) % chan->capacity;
    chan->size -= 1;
    ep_cond_signal(&chan->cond_send);
    ep_mutex_unlock(&chan->mutex);
    return value;
}

long long ep_net_connect(const char* host, long long port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return -1;
    struct hostent* server = gethostbyname(host);
    if (!server) {
        close(sockfd);
        return -1;
    }
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    memcpy(&serv_addr.sin_addr.s_addr, server->h_addr_list[0], server->h_length);
    serv_addr.sin_port = htons(port);
    if (connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        close(sockfd);
        return -1;
    }
    return sockfd;
}

long long ep_net_listen(long long port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return -1;
    int opt = 1;
    setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = INADDR_ANY;
    serv_addr.sin_port = htons(port);
    if (bind(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        close(sockfd);
        return -1;
    }
    if (listen(sockfd, 10) < 0) {
        close(sockfd);
        return -1;
    }
    return sockfd;
}

long long ep_net_accept(long long server_fd) {
    struct sockaddr_in cli_addr;
    socklen_t clilen = sizeof(cli_addr);
    int newsockfd = accept((int)server_fd, (struct sockaddr*)&cli_addr, &clilen);
    return newsockfd;
}

long long ep_net_send(long long fd, const char* data) {
    if (!data) return 0;
    return send((int)fd, data, strlen(data), 0);
}

char* ep_net_recv(long long fd, long long max_len) {
    char* buf = malloc(max_len + 1);
    if (!buf) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    ssize_t n = recv((int)fd, buf, max_len, 0);
    if (n < 0) n = 0;
    buf[n] = '\0';
    return buf;
}

long long ep_net_close(long long fd) {
    return close((int)fd);
}

long long ep_sleep_ms(long long ms) {
    usleep((useconds_t)(ms * 1000));
    return 0;
}

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

long long map_insert(long long map_ptr, long long key_val, long long value) {
    EpMap* map = (EpMap*)map_ptr;
    const char* key = (const char*)key_val;
    if (!map || !key) return 0;
    if (map->size * 2 >= map->capacity) {
        map_resize(map, map->capacity * 2);
    }
    unsigned long h = hash_string(key) % map->capacity;
    while (map->entries[h].used) {
        if (strcmp(map->entries[h].key, key) == 0) {
            map->entries[h].value = value;
            return value;
        }
        h = (h + 1) % map->capacity;
    }
    map->entries[h].key = strdup(key);
    map->entries[h].value = value;
    map->entries[h].used = 1;
    map->size++;
    return value;
}

long long map_get_val(long long map_ptr, long long key_val) {
    EpMap* map = (EpMap*)map_ptr;
    const char* key = (const char*)key_val;
    if (!map || !key) return 0;
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

long long map_contains(long long map_ptr, long long key_val) {
    EpMap* map = (EpMap*)map_ptr;
    const char* key = (const char*)key_val;
    if (!map || !key) return 0;
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
    EpMap* map = (EpMap*)map_ptr;
    const char* key = (const char*)key_val;
    if (!map || !key) return 0;
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

long long free_map(long long map_ptr) {
    EpMap* map = (EpMap*)map_ptr;
    if (!map) return 0;
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
    return (long long)resp;
}

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
            sprintf(result + (i * 2), "%02x", hash[i]);
        }
        result[64] = '\0';
    }
    return result;
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
            sprintf(result + (i * 2), "%02x", hash[i]);
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
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    return (long long)list->data;
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

long long free_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
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

int ep_argc = 0;
char** ep_argv = NULL;

void init_ep_args(int argc, char** argv) {
    ep_argc = argc;
    ep_argv = argv;
    ep_gc_register_thread((void*)&argc);
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

long long pop_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list || list->length <= 0) return 0;
    list->length -= 1;
    return list->data[list->length];
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
    const char* a = (const char*)a_ptr;
    const char* b = (const char*)b_ptr;
    if (a == b) return 1;
    if (!a || !b) return 0;
    return strcmp(a, b) == 0 ? 1 : 0;
}

/* ========== Sync Primitives ========== */

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
    for (int i = 0; i < 16; i++) bytes[i] = rand() & 0xFF;
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
    snprintf(uuid, 37, "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11],
        bytes[12], bytes[13], bytes[14], bytes[15]);
    return (long long)uuid;
}

long long ep_random_int(long long min, long long max) {
    if (max <= min) return min;
    return min + (rand() % (max - min + 1));
}

"#;

const C_MAIN_BOOTSTRAPPER: &str = r#"
/* Bootstrapper C main */
int main(int argc, char** argv) {
    srand((unsigned int)time(NULL));
    init_ep_args(argc, argv);
    int result = (int)_main();
    ep_gc_shutdown();
    return result;
}
"#;
