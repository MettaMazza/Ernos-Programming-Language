/// ErnosPlain Type Checker — Hindley-Milner type inference with unification
///
/// This module implements a real type system for ErnosPlain:
/// - Type variables for unknown types
/// - Constraint generation by walking the AST
/// - Unification algorithm to solve constraints
/// - Error reporting with source locations

use std::collections::HashMap;
use crate::ast::*;

/// Simple Levenshtein distance for fuzzy function name matching
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i-1] == b[j-1] { 0 } else { 1 };
            dp[i][j] = (dp[i-1][j] + 1).min(dp[i][j-1] + 1).min(dp[i-1][j-1] + cost);
        }
    }
    dp[m][n]
}

// ──────────────────────────────────────────────
// Core type representations
// ──────────────────────────────────────────────

/// A unique identifier for type variables
pub type TypeVarId = usize;

/// Monomorphic type — a concrete type or a type variable
#[derive(Debug, Clone, PartialEq)]
pub enum MonoType {
    Int,
    Float,
    Bool,
    Str,       // static string literal (&str in C terms)
    DynStr,    // heap-allocated string
    Unit,      // void / no value
    Never,     // unreachable (e.g., after return)

    /// An unresolved type variable, to be unified
    Var(TypeVarId),

    /// Typed list: List(T) where T is the element type
    List(Box<MonoType>),

    /// Function type: Fun(params, return_type)
    Fun(Vec<MonoType>, Box<MonoType>),

    /// Named struct with optional type arguments
    Struct(String, Vec<MonoType>),

    /// Named enum with optional type arguments  
    Enum(String, Vec<MonoType>),

    /// Borrowed reference to T
    Ref(Box<MonoType>),

    /// Future<T> — result type of an async function
    Future(Box<MonoType>),
}

impl MonoType {
    /// Returns true if this type is or contains heap-allocated data
    pub fn is_heap_allocated(&self) -> bool {
        matches!(self, MonoType::DynStr | MonoType::List(_) | MonoType::Struct(_, _) | MonoType::Enum(_, _))
    }
    
    /// Human-readable name for error messages
    pub fn display_name(&self) -> String {
        match self {
            MonoType::Int => "Int".to_string(),
            MonoType::Float => "Float".to_string(),
            MonoType::Bool => "Bool".to_string(),
            MonoType::Str => "Str".to_string(),
            MonoType::DynStr => "DynStr".to_string(),
            MonoType::Unit => "Unit".to_string(),
            MonoType::Never => "Never".to_string(),
            MonoType::Var(id) => format!("?T{}", id),
            MonoType::List(elem) => format!("List of {}", elem.display_name()),
            MonoType::Fun(params, ret) => {
                let params_str: Vec<String> = params.iter().map(|p| p.display_name()).collect();
                format!("({}) -> {}", params_str.join(", "), ret.display_name())
            }
            MonoType::Struct(name, args) => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let args_str: Vec<String> = args.iter().map(|a| a.display_name()).collect();
                    format!("{} of {}", name, args_str.join(" and "))
                }
            }
            MonoType::Enum(name, args) => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let args_str: Vec<String> = args.iter().map(|a| a.display_name()).collect();
                    format!("{} of {}", name, args_str.join(" and "))
                }
            }
            MonoType::Ref(inner) => format!("borrow of {}", inner.display_name()),
            MonoType::Future(inner) => format!("Future of {}", inner.display_name()),
        }
    }
}

// ──────────────────────────────────────────────
// Type error
// ──────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}

impl TypeError {
    fn new(message: String, span: Span) -> Self {
        Self { message, span, hint: None }
    }
    
    fn with_hint(message: String, span: Span, hint: String) -> Self {
        Self { message, span, hint: Some(hint) }
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type error at line {}:{}: {}", self.span.line, self.span.col, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\n  hint: {}", hint)?;
        }
        Ok(())
    }
}

// ──────────────────────────────────────────────
// Substitution table (type variable → resolved type)
// ──────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Substitution {
    bindings: HashMap<TypeVarId, MonoType>,
}

impl Substitution {
    fn new() -> Self {
        Self { bindings: HashMap::new() }
    }

    /// Apply substitution to a type, resolving all bound type variables
    fn apply(&self, ty: &MonoType) -> MonoType {
        match ty {
            MonoType::Var(id) => {
                if let Some(bound) = self.bindings.get(id) {
                    // Follow the chain — the bound type might also contain variables
                    self.apply(bound)
                } else {
                    ty.clone()
                }
            }
            MonoType::List(elem) => MonoType::List(Box::new(self.apply(elem))),
            MonoType::Fun(params, ret) => MonoType::Fun(
                params.iter().map(|p| self.apply(p)).collect(),
                Box::new(self.apply(ret)),
            ),
            MonoType::Struct(name, args) => MonoType::Struct(
                name.clone(),
                args.iter().map(|a| self.apply(a)).collect(),
            ),
            MonoType::Enum(name, args) => MonoType::Enum(
                name.clone(),
                args.iter().map(|a| self.apply(a)).collect(),
            ),
            MonoType::Ref(inner) => MonoType::Ref(Box::new(self.apply(inner))),
            MonoType::Future(inner) => MonoType::Future(Box::new(self.apply(inner))),
            // Primitive types are not affected by substitution
            _ => ty.clone(),
        }
    }

    /// Bind a type variable to a type, with occurs check
    fn bind(&mut self, var: TypeVarId, ty: &MonoType) -> Result<(), TypeError> {
        if let MonoType::Var(id) = ty {
            if *id == var {
                // X = X, trivially true
                return Ok(());
            }
        }
        
        // Occurs check: prevent infinite types like T = List(T)
        if self.occurs_in(var, ty) {
            return Err(TypeError::new(
                format!("Infinite type: ?T{} occurs in {}", var, self.apply(ty).display_name()),
                Span::default(),
            ));
        }
        
        self.bindings.insert(var, ty.clone());
        Ok(())
    }

    /// Check if a type variable occurs anywhere in a type
    fn occurs_in(&self, var: TypeVarId, ty: &MonoType) -> bool {
        let resolved = self.apply(ty);
        match &resolved {
            MonoType::Var(id) => *id == var,
            MonoType::List(elem) => self.occurs_in(var, elem),
            MonoType::Fun(params, ret) => {
                params.iter().any(|p| self.occurs_in(var, p)) || self.occurs_in(var, ret)
            }
            MonoType::Struct(_, args) | MonoType::Enum(_, args) => {
                args.iter().any(|a| self.occurs_in(var, a))
            }
            MonoType::Ref(inner) | MonoType::Future(inner) => self.occurs_in(var, inner),
            _ => false,
        }
    }
}

// ──────────────────────────────────────────────
// Unification
// ──────────────────────────────────────────────

/// Unify two types, updating the substitution table.
/// Returns an error if the types are incompatible.
fn unify(subst: &mut Substitution, t1: &MonoType, t2: &MonoType, span: Span) -> Result<(), TypeError> {
    let t1 = subst.apply(t1);
    let t2 = subst.apply(t2);

    match (&t1, &t2) {
        // Same primitive types
        (MonoType::Int, MonoType::Int) => Ok(()),
        (MonoType::Float, MonoType::Float) => Ok(()),
        (MonoType::Bool, MonoType::Bool) => Ok(()),
        (MonoType::Str, MonoType::Str) => Ok(()),
        (MonoType::DynStr, MonoType::DynStr) => Ok(()),
        (MonoType::Unit, MonoType::Unit) => Ok(()),
        (MonoType::Never, _) => Ok(()), // Never unifies with anything (bottom type)
        (_, MonoType::Never) => Ok(()),
        
        // Str and DynStr are compatible (string coercion)
        (MonoType::Str, MonoType::DynStr) | (MonoType::DynStr, MonoType::Str) => Ok(()),
        
        // Int and Float mixed arithmetic promotion
        (MonoType::Int, MonoType::Float) | (MonoType::Float, MonoType::Int) => Ok(()),
        
        // Int and Bool are compatible (ErnosPlain uses int for booleans)
        (MonoType::Int, MonoType::Bool) | (MonoType::Bool, MonoType::Int) => Ok(()),

        // Type variable binding
        (MonoType::Var(id), _) => subst.bind(*id, &t2),
        (_, MonoType::Var(id)) => subst.bind(*id, &t1),

        // Structural types
        (MonoType::List(e1), MonoType::List(e2)) => unify(subst, e1, e2, span),
        
        (MonoType::Fun(p1, r1), MonoType::Fun(p2, r2)) => {
            if p1.len() != p2.len() {
                return Err(TypeError::new(
                    format!("Function argument count mismatch: expected {}, found {}", p1.len(), p2.len()),
                    span,
                ));
            }
            for (a, b) in p1.iter().zip(p2.iter()) {
                unify(subst, a, b, span)?;
            }
            unify(subst, r1, r2, span)
        }
        
        (MonoType::Struct(n1, a1), MonoType::Struct(n2, a2)) => {
            if n1 != n2 {
                return Err(TypeError::new(
                    format!("Type mismatch: expected {}, found {}", n1, n2),
                    span,
                ));
            }
            for (a, b) in a1.iter().zip(a2.iter()) {
                unify(subst, a, b, span)?;
            }
            Ok(())
        }
        
        (MonoType::Enum(n1, a1), MonoType::Enum(n2, a2)) => {
            if n1 != n2 {
                return Err(TypeError::new(
                    format!("Type mismatch: expected {}, found {}", n1, n2),
                    span,
                ));
            }
            for (a, b) in a1.iter().zip(a2.iter()) {
                unify(subst, a, b, span)?;
            }
            Ok(())
        }
        
        (MonoType::Ref(i1), MonoType::Ref(i2)) => unify(subst, i1, i2, span),
        (MonoType::Future(i1), MonoType::Future(i2)) => unify(subst, i1, i2, span),

        // Incompatible types
        _ => Err(TypeError::new(
            format!("Type mismatch: expected {}, found {}", t1.display_name(), t2.display_name()),
            span,
        )),
    }
}

// ──────────────────────────────────────────────
// Type checker
// ──────────────────────────────────────────────

pub struct TypeChecker {
    subst: Substitution,
    next_var: TypeVarId,
    /// Variable → type for the current scope
    env: Vec<HashMap<String, MonoType>>,
    /// Function name → (param types, return type)
    func_types: HashMap<String, (Vec<MonoType>, MonoType)>,
    /// Struct name → field definitions
    struct_defs: HashMap<String, Vec<(String, MonoType)>>,
    /// Enum name → variant definitions
    enum_defs: HashMap<String, Vec<(String, Vec<(String, MonoType)>)>>,
    /// Variant name → enum name (quick lookup)
    variant_to_enum: HashMap<String, String>,
    /// Method (struct_name, method_name) → (param types, return type)
    method_types: HashMap<(String, String), (Vec<MonoType>, MonoType)>,
    /// Collected errors (we continue checking even after errors)
    pub errors: Vec<TypeError>,
    /// Collected warnings
    pub warnings: Vec<TypeError>,
    /// Variables bound to closures (for call resolution)
    closure_names: std::collections::HashSet<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            subst: Substitution::new(),
            next_var: 0,
            env: vec![HashMap::new()],
            func_types: HashMap::new(),
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            variant_to_enum: HashMap::new(),
            method_types: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            closure_names: std::collections::HashSet::new(),
        }
    }

    /// Generate a fresh type variable
    fn fresh_var(&mut self) -> MonoType {
        let id = self.next_var;
        self.next_var += 1;
        MonoType::Var(id)
    }

    /// Push a new scope
    fn push_scope(&mut self) {
        self.env.push(HashMap::new());
    }

    /// Pop the current scope
    fn pop_scope(&mut self) {
        self.env.pop();
    }

    /// Look up a variable in all scopes (innermost first)
    fn lookup(&self, name: &str) -> Option<MonoType> {
        for scope in self.env.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    /// Define a variable in the current scope
    fn define(&mut self, name: String, ty: MonoType) {
        if let Some(scope) = self.env.last_mut() {
            scope.insert(name, ty);
        }
    }

    /// Convert a TypeAnnotation (from the AST) to a MonoType
    fn annotation_to_mono(&self, ann: &TypeAnnotation) -> MonoType {
        match ann {
            TypeAnnotation::Int => MonoType::Int,
            TypeAnnotation::Float => MonoType::Float,
            TypeAnnotation::Bool => MonoType::Bool,
            TypeAnnotation::Str => MonoType::Str,
            TypeAnnotation::DynStr => MonoType::DynStr,
            TypeAnnotation::List => MonoType::List(Box::new(self.fresh_var_immut())),
            TypeAnnotation::UserDefined(name) => {
                if self.enum_defs.contains_key(name) {
                    MonoType::Enum(name.clone(), vec![])
                } else {
                    MonoType::Struct(name.clone(), vec![])
                }
            }
            TypeAnnotation::Generic(name, args) => {
                let mono_args: Vec<MonoType> = args.iter().map(|a| self.annotation_to_mono(a)).collect();
                if self.enum_defs.contains_key(name) {
                    MonoType::Enum(name.clone(), mono_args)
                } else {
                    MonoType::Struct(name.clone(), mono_args)
                }
            }
        }
    }

    // Immutable version for use inside &self methods
    fn fresh_var_immut(&self) -> MonoType {
        MonoType::Var(self.next_var) // Not ideal but the var won't collide if we're careful
    }

    /// Record an error without stopping
    fn error(&mut self, message: String, span: Span) {
        self.errors.push(TypeError::new(message, span));
    }
    
    fn error_with_hint(&mut self, message: String, span: Span, hint: String) {
        self.errors.push(TypeError::with_hint(message, span, hint));
    }

    // ──────────────────────────────────────────
    // Phase 1: Register all declarations
    // ──────────────────────────────────────────

    fn register_declarations(&mut self, program: &Program) {
        // Register struct definitions
        for sd in &program.struct_defs {
            let fields: Vec<(String, MonoType)> = sd.fields.iter()
                .map(|(name, ann, _)| (name.clone(), self.annotation_to_mono(ann)))
                .collect();
            self.struct_defs.insert(sd.name.clone(), fields);
        }

        // Register enum definitions — two passes to support recursive types
        // Pass 1: register enum names so annotation_to_mono can see them
        for ed in &program.enum_defs {
            self.enum_defs.insert(ed.name.clone(), vec![]);
            for (vname, _) in &ed.variants {
                self.variant_to_enum.insert(vname.clone(), ed.name.clone());
            }
        }
        // Pass 2: populate variant fields (now self-referential fields resolve correctly)
        for ed in &program.enum_defs {
            let variants: Vec<(String, Vec<(String, MonoType)>)> = ed.variants.iter()
                .map(|(vname, fields)| {
                    let mono_fields: Vec<(String, MonoType)> = fields.iter()
                        .map(|(fname, ann)| (fname.clone(), self.annotation_to_mono(ann)))
                        .collect();
                    (vname.clone(), mono_fields)
                })
                .collect();
            
            self.enum_defs.insert(ed.name.clone(), variants);
        }

        // Register function signatures
        for func in &program.functions {
            let param_types: Vec<MonoType> = func.params.iter()
                .map(|(_, is_borrowed, ann)| {
                    let base = if let Some(a) = ann {
                        self.annotation_to_mono(a)
                    } else {
                        self.fresh_var()
                    };
                    if *is_borrowed {
                        MonoType::Ref(Box::new(base))
                    } else {
                        base
                    }
                })
                .collect();
            
            let ret_type = if let Some(ann) = &func.return_type {
                self.annotation_to_mono(ann)
            } else {
                self.fresh_var()
            };
            
            self.func_types.insert(func.name.clone(), (param_types, ret_type));
        }

        // Register external function signatures
        for ext in &program.externals {
            let param_types: Vec<MonoType> = ext.params.iter()
                .map(|(_, is_borrowed, ann)| {
                    let base = if let Some(a) = ann {
                        self.annotation_to_mono(a)
                    } else {
                        self.fresh_var()
                    };
                    if *is_borrowed {
                        MonoType::Ref(Box::new(base))
                    } else {
                        base
                    }
                })
                .collect();
            
            let ret_type = if let Some(ann) = &ext.return_type {
                self.annotation_to_mono(ann)
            } else {
                self.fresh_var()
            };
            
            self.func_types.insert(ext.name.clone(), (param_types, ret_type));
        }

        // Register built-in functions
        self.register_builtins();

        // Register method signatures
        for md in &program.method_defs {
            let param_types: Vec<MonoType> = md.params.iter()
                .map(|(_, is_borrowed, ann)| {
                    let base = if let Some(a) = ann {
                        self.annotation_to_mono(a)
                    } else {
                        self.fresh_var()
                    };
                    if *is_borrowed { MonoType::Ref(Box::new(base)) } else { base }
                })
                .collect();
            
            let ret_type = if let Some(ann) = &md.return_type {
                self.annotation_to_mono(ann)
            } else {
                self.fresh_var()
            };
            
            self.method_types.insert(
                (md.struct_name.clone(), md.name.clone()),
                (param_types, ret_type),
            );
        }
    }

    fn register_builtins(&mut self) {
        // List operations — pre-compute fresh vars to avoid borrow checker issues
        let v0 = self.fresh_var();
        self.func_types.insert("create_list".into(), (vec![], MonoType::List(Box::new(v0))));
        
        let v1 = self.fresh_var();
        let v2 = self.fresh_var();
        self.func_types.insert("append_list".into(), (vec![MonoType::List(Box::new(v1)), v2], MonoType::Int));
        
        let v3 = self.fresh_var();
        let v4 = self.fresh_var();
        self.func_types.insert("get_list".into(), (vec![MonoType::List(Box::new(v3)), MonoType::Int], v4));
        
        let v5 = self.fresh_var();
        let v6 = self.fresh_var();
        self.func_types.insert("set_list".into(), (vec![MonoType::List(Box::new(v5)), MonoType::Int, v6], MonoType::Int));
        
        let v7 = self.fresh_var();
        self.func_types.insert("length_list".into(), (vec![MonoType::List(Box::new(v7))], MonoType::Int));
        
        let v8 = self.fresh_var();
        self.func_types.insert("remove_list".into(), (vec![MonoType::List(Box::new(v8)), MonoType::Int], MonoType::Int));

        // String operations
        self.func_types.insert("string_length".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("concat".into(), (vec![MonoType::Str, MonoType::Str], MonoType::DynStr));
        self.func_types.insert("substring".into(), (vec![MonoType::Str, MonoType::Int, MonoType::Int], MonoType::DynStr));
        self.func_types.insert("int_to_string".into(), (vec![MonoType::Int], MonoType::DynStr));
        self.func_types.insert("float_to_string".into(), (vec![MonoType::Float], MonoType::DynStr));
        self.func_types.insert("string_to_int".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("ep_int_to_str".into(), (vec![MonoType::Int], MonoType::DynStr));

        // Math
        self.func_types.insert("int_to_float".into(), (vec![MonoType::Int], MonoType::Float));
        self.func_types.insert("float_to_int".into(), (vec![MonoType::Float], MonoType::Int));

        // I/O
        self.func_types.insert("read_line".into(), (vec![], MonoType::DynStr));
        self.func_types.insert("read_int".into(), (vec![], MonoType::Int));
        self.func_types.insert("read_float".into(), (vec![], MonoType::Float));

        // Concurrency
        self.func_types.insert("create_channel".into(), (vec![], MonoType::Int));
        self.func_types.insert("send_channel".into(), (vec![MonoType::Int, MonoType::Int], MonoType::Unit));
        self.func_types.insert("recv_channel".into(), (vec![MonoType::Int], MonoType::Int));

        // List operations (continued)
        let v9 = self.fresh_var();
        self.func_types.insert("pop_list".into(), (vec![MonoType::List(Box::new(v9))], MonoType::Int));

        // Map operations
        let v10 = self.fresh_var();
        self.func_types.insert("create_map".into(), (vec![], v10));
        self.func_types.insert("map_insert".into(), (vec![MonoType::Int, MonoType::Int, MonoType::Int], MonoType::Int));
        self.func_types.insert("map_set_str".into(), (vec![MonoType::Int, MonoType::Str, MonoType::Int], MonoType::Int));
        self.func_types.insert("map_get_val".into(), (vec![MonoType::Int, MonoType::Int], MonoType::Int));
        self.func_types.insert("map_get_str".into(), (vec![MonoType::Int, MonoType::Str], MonoType::Int));
        let v11 = self.fresh_var();
        self.func_types.insert("map_keys".into(), (vec![MonoType::Int], MonoType::List(Box::new(v11))));
        self.func_types.insert("map_has_key".into(), (vec![MonoType::Int, MonoType::Int], MonoType::Int));

        // String operations (continued)
        self.func_types.insert("string_upper".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("string_lower".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("string_trim".into(), (vec![MonoType::Str], MonoType::DynStr));
        let v_split = self.fresh_var();
        self.func_types.insert("string_split".into(), (vec![MonoType::Str, MonoType::Str], MonoType::List(Box::new(v_split))));
        self.func_types.insert("char_at".into(), (vec![MonoType::Str, MonoType::Int], MonoType::Int));
        self.func_types.insert("char_from_code".into(), (vec![MonoType::Int], MonoType::DynStr));
        self.func_types.insert("string_contains".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("string_index_of".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("string_replace".into(), (vec![MonoType::Str, MonoType::Str, MonoType::Str], MonoType::DynStr));

        // File I/O
        self.func_types.insert("file_read".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("file_write".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("file_append".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("file_exists".into(), (vec![MonoType::Str], MonoType::Int));

        // Math / random
        self.func_types.insert("ep_abs".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_random_int".into(), (vec![MonoType::Int, MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_time_ms".into(), (vec![], MonoType::Int));
        self.func_types.insert("ep_sleep_ms".into(), (vec![MonoType::Int], MonoType::Unit));
        self.func_types.insert("ep_system".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("ep_play_sound".into(), (vec![MonoType::Str], MonoType::Int));

        // Display helpers
        self.func_types.insert("display".into(), (vec![MonoType::Int], MonoType::Unit));
        self.func_types.insert("display_string".into(), (vec![MonoType::Str], MonoType::Unit));
        self.func_types.insert("ep_auto_to_string".into(), (vec![MonoType::Int], MonoType::DynStr));

        // Memory management
        let v12 = self.fresh_var();
        self.func_types.insert("free_list".into(), (vec![MonoType::List(Box::new(v12))], MonoType::Unit));
        self.func_types.insert("free_map".into(), (vec![MonoType::Int], MonoType::Unit));
        self.func_types.insert("free_deque".into(), (vec![MonoType::Int], MonoType::Unit));

        // Map operations (continued)
        self.func_types.insert("map_size".into(), (vec![MonoType::Int], MonoType::Int));
        let vmc = self.fresh_var();
        self.func_types.insert("map_contains".into(), (vec![MonoType::Int, vmc], MonoType::Int));
        let vmd = self.fresh_var();
        self.func_types.insert("map_delete".into(), (vec![MonoType::Int, vmd], MonoType::Unit));
        let v13 = self.fresh_var();
        self.func_types.insert("map_values".into(), (vec![MonoType::Int], MonoType::List(Box::new(v13))));

        // Deque operations
        self.func_types.insert("create_deque".into(), (vec![], MonoType::Int));
        self.func_types.insert("deque_push_front".into(), (vec![MonoType::Int, MonoType::Int], MonoType::Unit));
        self.func_types.insert("deque_push_back".into(), (vec![MonoType::Int, MonoType::Int], MonoType::Unit));
        self.func_types.insert("deque_pop_front".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("deque_pop_back".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("deque_length".into(), (vec![MonoType::Int], MonoType::Int));

        // Concurrency (continued)
        self.func_types.insert("channel_has_data".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("channel_try_recv".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("channel_select".into(), (vec![MonoType::Int], MonoType::Int));

        // File system
        self.func_types.insert("read_file_content".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("write_file_content".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("run_command".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("fs_exists".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("fs_is_file".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("fs_is_dir".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("fs_get_size".into(), (vec![MonoType::Str], MonoType::Int));
        self.func_types.insert("fs_copy_file".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("fs_move_file".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("fs_delete_file".into(), (vec![MonoType::Str], MonoType::Int));
        let v14 = self.fresh_var();
        self.func_types.insert("fs_scan_dir".into(), (vec![MonoType::Str], MonoType::List(Box::new(v14))));

        // String utilities (continued)
        self.func_types.insert("get_character".into(), (vec![MonoType::Str, MonoType::Int], MonoType::Int));
        self.func_types.insert("string_from_list".into(), (vec![MonoType::List(Box::new(MonoType::Int))], MonoType::DynStr));
        self.func_types.insert("get_list_data_ptr".into(), (vec![MonoType::Int], MonoType::Int));

        // CLI arguments
        self.func_types.insert("get_argument".into(), (vec![MonoType::Int], MonoType::DynStr));
        self.func_types.insert("get_argument_count".into(), (vec![], MonoType::Int));

        // Networking
        self.func_types.insert("ep_net_connect".into(), (vec![MonoType::Str, MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_net_listen".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_net_accept".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_net_send".into(), (vec![MonoType::Int, MonoType::Str], MonoType::Int));
        self.func_types.insert("ep_net_recv".into(), (vec![MonoType::Int, MonoType::Int], MonoType::DynStr));
        self.func_types.insert("ep_net_recv_bytes".into(), (vec![MonoType::Int, MonoType::Int], MonoType::DynStr));
        self.func_types.insert("ep_net_close".into(), (vec![MonoType::Int], MonoType::Unit));

        // HTTP
        self.func_types.insert("ep_http_request".into(), (vec![MonoType::Str, MonoType::Str, MonoType::Str, MonoType::Str], MonoType::DynStr));

        // Crypto
        self.func_types.insert("ep_md5".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("ep_sha256".into(), (vec![MonoType::Str], MonoType::DynStr));
        self.func_types.insert("ep_sha1".into(), (vec![MonoType::Str], MonoType::DynStr));

        // JSON
        self.func_types.insert("json_get_string".into(), (vec![MonoType::Str, MonoType::Str], MonoType::DynStr));
        self.func_types.insert("json_get_int".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));
        self.func_types.insert("json_get_bool".into(), (vec![MonoType::Str, MonoType::Str], MonoType::Int));

        // SQLite
        self.func_types.insert("sqlite_get_callback_ptr".into(), (vec![], MonoType::Int));

        // Time (additional)
        self.func_types.insert("ep_time_now_ms".into(), (vec![], MonoType::Int));
        self.func_types.insert("ep_time_now_sec".into(), (vec![], MonoType::Int));
        self.func_types.insert("ep_time_year".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_time_month".into(), (vec![MonoType::Int], MonoType::Int));
        self.func_types.insert("ep_time_day".into(), (vec![MonoType::Int], MonoType::Int));
    }

    // ──────────────────────────────────────────
    // Phase 2: Type check each function body
    // ──────────────────────────────────────────

    pub fn check_program(&mut self, program: &Program) {
        self.register_declarations(program);

        for func in &program.functions {
            self.check_function(func);
        }

        for md in &program.method_defs {
            self.check_method(md);
        }
    }

    fn check_function(&mut self, func: &Function) {
        self.push_scope();

        // Bind parameters
        if let Some((param_types, _)) = self.func_types.get(&func.name).cloned() {
            for (i, (name, _, _)) in func.params.iter().enumerate() {
                if i < param_types.len() {
                    self.define(name.clone(), param_types[i].clone());
                }
            }
        }

        // Check body
        for stmt in &func.body {
            self.check_stmt(stmt);
        }

        self.pop_scope();
    }

    fn check_method(&mut self, md: &MethodDef) {
        self.push_scope();

        // Bind `self` parameter
        self.define("self".into(), MonoType::Struct(md.struct_name.clone(), vec![]));

        // Bind parameters
        if let Some((param_types, _)) = self.method_types.get(&(md.struct_name.clone(), md.name.clone())).cloned() {
            for (i, (name, _, _)) in md.params.iter().enumerate() {
                if i < param_types.len() {
                    self.define(name.clone(), param_types[i].clone());
                }
            }
        }

        for stmt in &md.body {
            self.check_stmt(stmt);
        }

        self.pop_scope();
    }

    // ──────────────────────────────────────────
    // Statement type checking
    // ──────────────────────────────────────────

    fn check_stmt(&mut self, stmt: &Stmt) {
        match &stmt.node {
            StmtNode::Set(name, expr, type_ann) => {
                let expr_type = self.check_expr(expr);
                
                // Track if this variable is bound to a closure
                if matches!(expr.node, ExprNode::Closure(_, _)) {
                    self.closure_names.insert(name.clone());
                }
                
                if let Some(ann) = type_ann {
                    let declared_type = self.annotation_to_mono(ann);
                    if let Err(_e) = unify(&mut self.subst, &declared_type, &expr_type, expr.span) {
                        self.error_with_hint(
                            format!("Cannot assign {} to variable '{}' declared as {}",
                                expr_type.display_name(), name, declared_type.display_name()),
                            stmt.span,
                            format!("The expression has type {} but the variable was declared as {}",
                                self.subst.apply(&expr_type).display_name(),
                                self.subst.apply(&declared_type).display_name()),
                        );
                    }
                    self.define(name.clone(), declared_type);
                } else {
                    self.define(name.clone(), expr_type);
                }
            }

            StmtNode::If(cond, then_body, else_body) => {
                let cond_type = self.check_expr(cond);
                // Condition should be bool-like (Int or Bool)
                let is_bool_like = matches!(
                    self.subst.apply(&cond_type),
                    MonoType::Int | MonoType::Bool | MonoType::Var(_)
                );
                if !is_bool_like {
                    self.error(
                        format!("Condition must be Bool or Int, found {}", 
                            self.subst.apply(&cond_type).display_name()),
                        cond.span,
                    );
                }

                self.push_scope();
                for s in then_body { self.check_stmt(s); }
                self.pop_scope();

                if let Some(else_b) = else_body {
                    self.push_scope();
                    for s in else_b { self.check_stmt(s); }
                    self.pop_scope();
                }
            }

            StmtNode::RepeatWhile(cond, body) => {
                let cond_type = self.check_expr(cond);
                let is_bool_like = matches!(
                    self.subst.apply(&cond_type),
                    MonoType::Int | MonoType::Bool | MonoType::Var(_)
                );
                if !is_bool_like {
                    self.error(
                        format!("Loop condition must be Bool or Int, found {}",
                            self.subst.apply(&cond_type).display_name()),
                        cond.span,
                    );
                }

                self.push_scope();
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }

            StmtNode::Return(expr) => {
                let _ret_type = self.check_expr(expr);
                // TODO: unify with declared return type
            }

            StmtNode::Display(expr) => {
                let _display_type = self.check_expr(expr);
                // display accepts any type
            }

            StmtNode::Spawn(func_name, args) => {
                let arg_types: Vec<MonoType> = args.iter().map(|a| self.check_expr(a)).collect();
                if let Some((param_types, _)) = self.func_types.get(func_name).cloned() {
                    if arg_types.len() != param_types.len() {
                        self.error(
                            format!("Function '{}' expects {} arguments, got {}", 
                                func_name, param_types.len(), arg_types.len()),
                            stmt.span,
                        );
                    } else {
                        for (i, (arg_t, param_t)) in arg_types.iter().zip(param_types.iter()).enumerate() {
                            if let Err(_) = unify(&mut self.subst, arg_t, param_t, stmt.span) {
                                self.error(
                                    format!("Argument {} of '{}': expected {}, found {}",
                                        i + 1, func_name, param_t.display_name(), arg_t.display_name()),
                                    stmt.span,
                                );
                            }
                        }
                    }
                }
            }

            StmtNode::Send(chan, val) => {
                let _chan_type = self.check_expr(chan);
                let _val_type = self.check_expr(val);
            }

            StmtNode::FieldSet(obj, field_name, val) => {
                let obj_type = self.check_expr(obj);
                let val_type = self.check_expr(val);
                
                let resolved = self.subst.apply(&obj_type);
                if let MonoType::Struct(struct_name, _) = &resolved {
                    if let Some(fields) = self.struct_defs.get(struct_name).cloned() {
                        if let Some((_, field_type)) = fields.iter().find(|(n, _)| n == field_name) {
                            if let Err(_) = unify(&mut self.subst, &val_type, field_type, val.span) {
                                self.error(
                                    format!("Cannot assign {} to field '{}' of {}: expected {}",
                                        self.subst.apply(&val_type).display_name(),
                                        field_name, struct_name,
                                        self.subst.apply(field_type).display_name()),
                                    val.span,
                                );
                            }
                        } else {
                            self.error(
                                format!("Struct '{}' has no field '{}'", struct_name, field_name),
                                stmt.span,
                            );
                        }
                    }
                }
            }

            StmtNode::Match(expr, arms) => {
                let match_type = self.check_expr(expr);
                let resolved = self.subst.apply(&match_type);
                
                if let MonoType::Enum(enum_name, _) = &resolved {
                    if let Some(variants) = self.enum_defs.get(enum_name).cloned() {
                        for (variant_name, bindings, body) in arms {
                            self.push_scope();
                            if let Some((_, fields)) = variants.iter().find(|(vn, _)| vn == variant_name) {
                                for (i, binding) in bindings.iter().enumerate() {
                                    if i < fields.len() {
                                        self.define(binding.clone(), fields[i].1.clone());
                                    }
                                }
                            }
                            for s in body { self.check_stmt(s); }
                            self.pop_scope();
                        }
                    }
                } else {
                    // String or integer match — just type-check the body of each arm
                    for (_pattern, _bindings, body) in arms {
                        self.push_scope();
                        for s in body { self.check_stmt(s); }
                        self.pop_scope();
                    }
                }
            }

            StmtNode::ForEach(loop_var, iterable, body) => {
                let iter_type = self.check_expr(iterable);
                let resolved = self.subst.apply(&iter_type);
                
                let elem_type = match &resolved {
                    MonoType::List(elem) => (**elem).clone(),
                    _ => MonoType::Int, // range() returns Int elements
                };
                
                self.push_scope();
                self.define(loop_var.clone(), elem_type);
                for s in body { self.check_stmt(s); }
                self.pop_scope();
            }

            StmtNode::Break | StmtNode::Continue => {}

            StmtNode::ExprStmt(expr) => {
                let _t = self.check_expr(expr);
            }
        }
    }

    // ──────────────────────────────────────────
    // Expression type inference
    // ──────────────────────────────────────────

    fn check_expr(&mut self, expr: &Expr) -> MonoType {
        match &expr.node {
            ExprNode::Integer(_) => MonoType::Int,
            ExprNode::FloatLiteral(_) => MonoType::Float,
            ExprNode::BoolLiteral(_) => MonoType::Bool,
            ExprNode::StringLiteral(_) => MonoType::Str,

            ExprNode::Identifier(name) => {
                if let Some(ty) = self.lookup(name) {
                    ty
                } else if let Some(enum_name) = self.variant_to_enum.get(name).cloned() {
                    // Bare enum variant (no data)
                    MonoType::Enum(enum_name, vec![])
                } else {
                    // Unknown variable — this might be a runtime-defined global
                    // Don't error here; the existing compiler allows unresolved identifiers 
                    // for some built-in variables
                    self.fresh_var()
                }
            }

            ExprNode::Binary(left, _op, right) => {
                let lt = self.check_expr(left);
                let rt = self.check_expr(right);
                let lt_r = self.subst.apply(&lt);
                let rt_r = self.subst.apply(&rt);
                
                // Numeric promotion
                if lt_r == MonoType::Float || rt_r == MonoType::Float {
                    // Unify both with Float
                    if let Err(_) = unify(&mut self.subst, &lt, &MonoType::Float, expr.span) {
                        self.error(
                            format!("Left operand of arithmetic must be numeric, found {}", self.subst.apply(&lt).display_name()),
                            left.span,
                        );
                    }
                    if let Err(_) = unify(&mut self.subst, &rt, &MonoType::Float, expr.span) {
                        self.error(
                            format!("Right operand of arithmetic must be numeric, found {}", self.subst.apply(&rt).display_name()),
                            right.span,
                        );
                    }
                    MonoType::Float
                } else {
                    // Unify both with Int
                    if let Err(_) = unify(&mut self.subst, &lt, &MonoType::Int, expr.span) {
                        self.error(
                            format!("Left operand of arithmetic must be numeric, found {}", self.subst.apply(&lt).display_name()),
                            left.span,
                        );
                    }
                    if let Err(_) = unify(&mut self.subst, &rt, &MonoType::Int, expr.span) {
                        self.error(
                            format!("Right operand of arithmetic must be numeric, found {}", self.subst.apply(&rt).display_name()),
                            right.span,
                        );
                    }
                    MonoType::Int
                }
            }

            ExprNode::Comparison(left, _op, right) => {
                let _lt = self.check_expr(left);
                let _rt = self.check_expr(right);
                MonoType::Bool
            }

            ExprNode::Logical(left, _op, right) => {
                let _lt = self.check_expr(left);
                let _rt = self.check_expr(right);
                MonoType::Bool
            }

            ExprNode::Call(name, args) => {
                let arg_types: Vec<MonoType> = args.iter().map(|a| self.check_expr(a)).collect();
                
                if let Some((param_types, ret_type)) = self.func_types.get(name).cloned() {
                    // Polymorphic builtins that accept any value type — skip arg type checking
                    // because the C runtime stores all values as long long (ints or pointers)
                    let skip_type_check = matches!(name.as_str(),
                        "concat" | "append_list" | "get_list" | "set_list" |
                        "map_insert" | "map_set_str" | "map_get_val" | "map_get_str" |
                        "map_contains" | "map_delete" | "map_keys" | "map_size" |
                        "map_values" | "map_has_key" | "free_list" | "free_map" |
                        "ep_auto_to_string" | "display"
                    );
                    // Check argument count (some builtins like concat are variadic)
                    if !skip_type_check && arg_types.len() != param_types.len() {
                        self.error(
                            format!("Function '{}' expects {} arguments, got {}",
                                name, param_types.len(), arg_types.len()),
                            expr.span,
                        );
                    } else if !skip_type_check {
                        // Check argument types
                        for (i, (arg_t, param_t)) in arg_types.iter().zip(param_types.iter()).enumerate() {
                            if let Err(_) = unify(&mut self.subst, arg_t, param_t, expr.span) {
                                let resolved_arg = self.subst.apply(arg_t);
                                let resolved_param = self.subst.apply(param_t);
                                self.error_with_hint(
                                    format!("Argument {} of '{}': expected {}, found {}",
                                        i + 1, name, resolved_param.display_name(), resolved_arg.display_name()),
                                    expr.span,
                                    format!("Consider converting the value to {}", resolved_param.display_name()),
                                );
                            }
                        }
                    }
                    ret_type
                } else if self.closure_names.contains(name) || self.lookup(name).is_some() {
                    // It's a closure variable or in-scope variable — treat as valid call
                    self.fresh_var()
                } else {
                    // Unknown function — emit error with suggestion
                    let mut best_match: Option<(&str, usize)> = None;
                    for known in self.func_types.keys() {
                        // Check Levenshtein distance
                        let dist = levenshtein_distance(name, known);
                        if dist <= 3 {
                            if best_match.is_none() || dist < best_match.unwrap().1 {
                                best_match = Some((known, dist));
                            }
                        }
                        // Also check if user's name is a suffix/substring of a known function
                        // (handles to_upper → string_upper, index_of → string_index_of)
                        if known.ends_with(name) || known.ends_with(&format!("_{}", name)) {
                            best_match = Some((known, 0));
                        }
                        // Check if stripping common prefixes from user's name matches a suffix
                        // e.g. to_upper → strip "to_" → upper → string_upper ends with upper
                        for prefix in &["to_", "get_", "is_", "do_"] {
                            if let Some(stripped) = name.strip_prefix(prefix) {
                                if known.ends_with(stripped) && stripped.len() >= 3 {
                                    best_match = Some((known, 0));
                                }
                            }
                        }
                    }
                    if let Some((suggestion, _)) = best_match {
                        self.error_with_hint(
                            format!("Unknown function '{}'", name),
                            expr.span,
                            format!("Did you mean '{}'?", suggestion),
                        );
                    } else {
                        self.error(
                            format!("Unknown function '{}'. Use --list-builtins to see available functions.", name),
                            expr.span,
                        );
                    }
                    self.fresh_var()
                }
            }

            ExprNode::Channel => MonoType::Int,
            ExprNode::Receive(inner) => {
                let _chan_type = self.check_expr(inner);
                MonoType::Int
            }

            ExprNode::Borrow(inner) => {
                let inner_type = self.check_expr(inner);
                MonoType::Ref(Box::new(inner_type))
            }

            ExprNode::FieldAccess(obj, field_name) => {
                let obj_type = self.check_expr(obj);
                let resolved = self.subst.apply(&obj_type);
                
                if let MonoType::Struct(struct_name, _) = &resolved {
                    if let Some(fields) = self.struct_defs.get(struct_name).cloned() {
                        if let Some((_, field_type)) = fields.iter().find(|(n, _)| n == field_name) {
                            return field_type.clone();
                        } else {
                            self.error(
                                format!("Struct '{}' has no field '{}'", struct_name, field_name),
                                expr.span,
                            );
                        }
                    }
                }
                self.fresh_var()
            }

            ExprNode::StructCreate(struct_name, field_exprs) => {
                if let Some(fields) = self.struct_defs.get(struct_name).cloned() {
                    for (fname, fexpr) in field_exprs {
                        let expr_type = self.check_expr(fexpr);
                        if let Some((_, expected_type)) = fields.iter().find(|(n, _)| n == fname) {
                            if let Err(_) = unify(&mut self.subst, &expr_type, expected_type, fexpr.span) {
                                self.error(
                                    format!("Field '{}' of '{}': expected {}, found {}",
                                        fname, struct_name,
                                        self.subst.apply(expected_type).display_name(),
                                        self.subst.apply(&expr_type).display_name()),
                                    fexpr.span,
                                );
                            }
                        }
                    }
                }
                MonoType::Struct(struct_name.clone(), vec![])
            }

            ExprNode::EnumCreate(enum_name, variant_name, args) => {
                let actual_enum = if enum_name.is_empty() {
                    self.variant_to_enum.get(variant_name).cloned().unwrap_or_default()
                } else {
                    enum_name.clone()
                };
                
                // Type check variant args against declared field types
                let variant_fields = self.enum_defs.get(&actual_enum)
                    .and_then(|variants| variants.iter().find(|(vn, _)| vn == variant_name))
                    .map(|(_, fields)| fields.clone());

                if let Some(fields) = variant_fields {
                    if args.len() != fields.len() {
                        self.error(
                            format!("Enum variant '{}::{}' expects {} argument(s), found {}",
                                actual_enum, variant_name, fields.len(), args.len()),
                            expr.span,
                        );
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.check_expr(arg);
                        if i < fields.len() {
                            let (ref field_name, ref expected_type) = fields[i];
                            if let Err(_) = unify(&mut self.subst, &arg_type, expected_type, arg.span) {
                                self.error(
                                    format!("Enum variant '{}::{}', field '{}': expected {}, found {}",
                                        actual_enum, variant_name, field_name,
                                        self.subst.apply(expected_type).display_name(),
                                        self.subst.apply(&arg_type).display_name()),
                                    arg.span,
                                );
                            }
                        }
                    }
                } else {
                    // Variant not found in enum def — just check args without field matching
                    for arg in args {
                        let _t = self.check_expr(arg);
                    }
                }
                
                MonoType::Enum(actual_enum, vec![])
            }

            ExprNode::MethodCall(obj, method_name, args) => {
                let obj_type = self.check_expr(obj);
                let resolved = self.subst.apply(&obj_type);
                
                for arg in args {
                    let _t = self.check_expr(arg);
                }
                
                if let MonoType::Struct(struct_name, _) = &resolved {
                    if let Some((_, ret_type)) = self.method_types.get(&(struct_name.clone(), method_name.clone())).cloned() {
                        return ret_type;
                    }
                }
                self.fresh_var()
            }

            ExprNode::UnaryNot(inner) => {
                let _t = self.check_expr(inner);
                MonoType::Bool
            }

            ExprNode::TryExpr(inner) => {
                let _t = self.check_expr(inner);
                MonoType::Int // try returns an error code
            }

            ExprNode::Closure(params, body) => {
                self.push_scope();
                let param_types: Vec<MonoType> = params.iter()
                    .map(|name| {
                        let tv = self.fresh_var();
                        self.define(name.clone(), tv.clone());
                        tv
                    })
                    .collect();
                
                let mut last_type = MonoType::Unit;
                for s in body {
                    self.check_stmt(s);
                    // Track return type from last statement
                    if let StmtNode::Return(expr) = &s.node {
                        last_type = self.check_expr(expr);
                    }
                }
                self.pop_scope();
                
                MonoType::Fun(param_types, Box::new(last_type))
            }

            ExprNode::Await(inner) => {
                let inner_type = self.check_expr(inner);
                let resolved = self.subst.apply(&inner_type);
                if let MonoType::Future(result_type) = resolved {
                    *result_type
                } else {
                    // Await on a non-future — it's still valid in current ErnosPlain
                    inner_type
                }
            }

            ExprNode::ListLiteral(elements) => {
                let elem_var = self.fresh_var();
                for elem in elements {
                    let _elem_type = self.check_expr(elem);
                }
                MonoType::List(Box::new(elem_var))
            }
        }
    }

    // ──────────────────────────────────────────
    // Public interface
    // ──────────────────────────────────────────

    /// Run type checking on a program. Returns errors found.
    pub fn check(program: &Program) -> Vec<TypeError> {
        let mut checker = TypeChecker::new();
        checker.check_program(program);
        checker.errors
    }

    /// Run type checking, returning both errors and warnings
    pub fn check_full(program: &Program) -> (Vec<TypeError>, Vec<TypeError>) {
        let mut checker = TypeChecker::new();
        checker.check_program(program);
        (checker.errors, checker.warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn check_source(source: &str) -> Vec<TypeError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Lexer error");
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().expect("Parser error");
        TypeChecker::check(&program)
    }

    #[test]
    fn test_valid_program() {
        let errors = check_source(
            "define main:\n    set x to 42\n    display x\n    return 0"
        );
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_struct_field_access() {
        let errors = check_source(
            "define structure User:\n    field name as Str\n    field age as Int\n\ndefine main:\n    set user to create User:\n        name is \"Alice\"\n        age is 30\n    display user.name\n    return 0"
        );
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_arithmetic_types() {
        let errors = check_source(
            "define add with a as Int and b as Int returning Int:\n    return a plus b\n\ndefine main:\n    set result to add(10 and 20)\n    display result\n    return 0"
        );
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }
}
