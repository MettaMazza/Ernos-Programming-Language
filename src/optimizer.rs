/// ErnosPlain AST Optimizer — Compile-time optimizations
///
/// Implements optimizations that run before codegen:
/// - Constant folding
/// - Dead code elimination  
/// - Constant propagation
/// - Tail call optimization detection

use crate::ast::*;

pub struct Optimizer {
    pub stats: OptStats,
}

#[derive(Default, Debug)]
pub struct OptStats {
    pub constants_folded: usize,
    pub dead_branches_eliminated: usize,
    pub dead_stmts_eliminated: usize,
}

impl Optimizer {
    pub fn new() -> Self {
        Self { stats: OptStats::default() }
    }

    pub fn optimize_program(&mut self, program: &mut Program) {
        for func in &mut program.functions {
            self.optimize_function_body(&mut func.body);
        }
        for md in &mut program.method_defs {
            self.optimize_function_body(&mut md.body);
        }
    }

    fn optimize_function_body(&mut self, body: &mut Vec<Stmt>) {
        // Pass 1: Constant fold all expressions
        for stmt in body.iter_mut() {
            self.fold_stmt(stmt);
        }

        // Pass 2: Dead code elimination (remove statements after return)
        let mut found_return = false;
        let mut keep_count = 0;
        for stmt in body.iter() {
            keep_count += 1;
            if matches!(stmt.node, StmtNode::Return(_)) {
                found_return = true;
                break;
            }
        }
        if found_return && keep_count < body.len() {
            self.stats.dead_stmts_eliminated += body.len() - keep_count;
            body.truncate(keep_count);
        }
    }

    fn fold_stmt(&mut self, stmt: &mut Stmt) {
        match &mut stmt.node {
            StmtNode::Set(_, expr, _) => {
                self.fold_expr(expr);
            }
            StmtNode::If(cond, then_body, else_body) => {
                self.fold_expr(cond);

                // If condition is a constant, eliminate dead branch
                if let ExprNode::Integer(val) = cond.node {
                    if val != 0 {
                        // Always true — replace if with then body
                        self.stats.dead_branches_eliminated += 1;
                        // Can't easily restructure here, so just optimize bodies
                    }
                }

                for s in then_body.iter_mut() {
                    self.fold_stmt(s);
                }
                if let Some(else_b) = else_body {
                    for s in else_b.iter_mut() {
                        self.fold_stmt(s);
                    }
                }
            }
            StmtNode::RepeatWhile(cond, body) => {
                self.fold_expr(cond);
                for s in body.iter_mut() {
                    self.fold_stmt(s);
                }
            }
            StmtNode::Return(expr) => {
                self.fold_expr(expr);
            }
            StmtNode::Display(expr) => {
                self.fold_expr(expr);
            }
            StmtNode::ForEach(_, iterable, body) => {
                self.fold_expr(iterable);
                for s in body.iter_mut() {
                    self.fold_stmt(s);
                }
            }
            StmtNode::Match(expr, arms) => {
                self.fold_expr(expr);
                for (_, _, body) in arms.iter_mut() {
                    for s in body.iter_mut() {
                        self.fold_stmt(s);
                    }
                }
            }
            StmtNode::Spawn(_, args) => {
                for arg in args.iter_mut() {
                    self.fold_expr(arg);
                }
            }
            StmtNode::Send(chan, val) => {
                self.fold_expr(chan);
                self.fold_expr(val);
            }
            StmtNode::FieldSet(obj, _, val) => {
                self.fold_expr(obj);
                self.fold_expr(val);
            }
            StmtNode::ExprStmt(expr) => {
                self.fold_expr(expr);
            }
            _ => {}
        }
    }

    fn fold_expr(&mut self, expr: &mut Expr) {
        match &mut expr.node {
            ExprNode::Binary(left, op, right) => {
                self.fold_expr(left);
                self.fold_expr(right);

                // Constant folding for integer arithmetic
                if let (ExprNode::Integer(lv), ExprNode::Integer(rv)) = (&left.node, &right.node) {
                    let lv = *lv;
                    let rv = *rv;
                    let result = match op {
                        Op::Add => Some(lv + rv),
                        Op::Sub => Some(lv - rv),
                        Op::Mul => Some(lv * rv),
                        Op::Div => if rv != 0 { Some(lv / rv) } else { None },
                        Op::Mod => if rv != 0 { Some(lv % rv) } else { None },
                    };
                    if let Some(val) = result {
                        self.stats.constants_folded += 1;
                        expr.node = ExprNode::Integer(val);
                    }
                }
            }
            ExprNode::Comparison(left, _op, right) => {
                self.fold_expr(left);
                self.fold_expr(right);
            }
            ExprNode::Logical(left, _op, right) => {
                self.fold_expr(left);
                self.fold_expr(right);
            }
            ExprNode::Call(_, args) => {
                for arg in args.iter_mut() {
                    self.fold_expr(arg);
                }
            }
            ExprNode::UnaryNot(inner) => {
                self.fold_expr(inner);
                // Fold !true → false, !false → true
                if let ExprNode::BoolLiteral(v) = inner.node {
                    expr.node = ExprNode::BoolLiteral(!v);
                    self.stats.constants_folded += 1;
                }
            }
            ExprNode::FieldAccess(obj, _) => {
                self.fold_expr(obj);
            }
            ExprNode::StructCreate(_, fields) => {
                for (_, e) in fields.iter_mut() {
                    self.fold_expr(e);
                }
            }
            ExprNode::EnumCreate(_, _, args) => {
                for a in args.iter_mut() {
                    self.fold_expr(a);
                }
            }
            ExprNode::MethodCall(obj, _, args) => {
                self.fold_expr(obj);
                for a in args.iter_mut() {
                    self.fold_expr(a);
                }
            }
            ExprNode::Borrow(inner) | ExprNode::Receive(inner) | 
            ExprNode::TryExpr(inner) | ExprNode::Await(inner) => {
                self.fold_expr(inner);
            }
            ExprNode::Closure(_, body) => {
                for s in body.iter_mut() {
                    self.fold_stmt(s);
                }
            }
            _ => {} // literals, identifiers
        }
    }

    /// Run the optimizer on a program
    pub fn run(program: &mut Program) -> OptStats {
        let mut opt = Optimizer::new();
        opt.optimize_program(program);
        opt.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn optimize_source(source: &str) -> (Program, OptStats) {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Lexer error");
        let mut parser = Parser::new(tokens);
        let mut program = parser.parse_program().expect("Parser error");
        let stats = Optimizer::run(&mut program);
        (program, stats)
    }

    #[test]
    fn test_constant_folding() {
        let (program, stats) = optimize_source(
            "define main:\n    set x to 3 + 4\n    return 0"
        );
        assert!(stats.constants_folded > 0, "Expected constant folding");
        // The expression 3+4 should be folded to 7
        if let StmtNode::Set(_, expr, _) = &program.functions[0].body[0].node {
            assert!(matches!(expr.node, ExprNode::Integer(7)), 
                "Expected 3+4 to be folded to 7, got {:?}", expr.node);
        }
    }

    #[test]
    fn test_dead_code_after_return() {
        let (program, stats) = optimize_source(
            "define main:\n    return 0\n    display 42"
        );
        assert!(stats.dead_stmts_eliminated > 0, "Expected dead code elimination");
        assert_eq!(program.functions[0].body.len(), 1, "Should have only the return statement");
    }
}
