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
    func_return_types: HashMap<String, Type>,
    current_return_type: Type,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            func_return_types: HashMap::new(),
            current_return_type: Type::Int,
        }
    }

    fn analyze_return_types(&mut self, program: &Program) {
        self.func_return_types.clear();
        
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
        self.func_return_types.insert("ep_md5".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_sha256".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_net_connect".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_listen".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_accept".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_send".to_string(), Type::Int);
        self.func_return_types.insert("ep_net_recv".to_string(), Type::DynStr);
        self.func_return_types.insert("ep_net_close".to_string(), Type::Int);
        
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
        self.analyze_return_types(program);

        // Write C Runtime library headers & source code directly at the top
        self.out.push_str(RUNTIME_HEADER_AND_SRC);
        self.out.push_str("\n/* User Function Prototypes */\n");

        for func in &program.functions {
            let mut params_str = Vec::new();
            for _ in &func.params {
                params_str.push("long long");
            }
            let name = if func.name == "main" { "_main".to_string() } else { func.name.clone() };
            self.out.push_str(&format!("long long {}({});\n", name, params_str.join(", ")));
        }
        self.out.push_str("\n");

        for func in &program.functions {
            self.gen_function(func)?;
        }

        // Standard C main function
        self.out.push_str(C_MAIN_BOOTSTRAPPER);

        Ok(self.out.clone())
    }

    fn gen_function(&mut self, func: &Function) -> Result<(), String> {
        let mut var_types = HashMap::new();

        for param in &func.params {
            var_types.insert(param.clone(), Type::Int);
        }
        self.collect_var_types(&func.body, &mut var_types);

        self.current_return_type = *self.func_return_types.get(&func.name).unwrap_or(&Type::Int);

        let name = if func.name == "main" { "_main".to_string() } else { func.name.clone() };
        
        let mut params_decl = Vec::new();
        for param in &func.params {
            params_decl.push(format!("long long {}", param));
        }

        self.out.push_str(&format!("long long {}({}) {{\n", name, params_decl.join(", ")));
        
        // Declare all local variables (except parameters) as long long initialized to 0
        for (var_name, _) in &var_types {
            if !func.params.contains(var_name) {
                self.out.push_str(&format!("    long long {} = 0;\n", var_name));
            }
        }
        self.out.push_str("    long long ret_val = 0;\n\n");

        // Generate function body
        for stmt in &func.body {
            self.gen_statement(stmt, &var_types)?;
        }

        // Cleanup label & deallocations
        self.out.push_str("L_cleanup:\n");
        for (var_name, _) in &var_types {
            if !func.params.contains(var_name) {
                let t = var_types.get(var_name);
                if t == Some(&Type::List) {
                    self.out.push_str(&format!("    free_list({});\n", var_name));
                }
            }
        }
        self.out.push_str("    return ret_val;\n}\n\n");

        Ok(())
    }

    fn gen_statement(
        &mut self,
        stmt: &Stmt,
        var_types: &HashMap<String, Type>,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Set(name, expr) => {
                let t = var_types.get(name);
                let expr_str = self.gen_expr(expr, var_types)?;

                if t == Some(&Type::List) {
                    self.out.push_str("    {\n");
                    self.out.push_str(&format!("        long long tmp_val = {};\n", expr_str));
                    self.out.push_str(&format!("        free_list({});\n", name));
                    self.out.push_str(&format!("        {} = tmp_val;\n", name));
                    self.out.push_str("    }\n");
                } else {
                    self.out.push_str(&format!("    {} = {};\n", name, expr_str));
                }
            }
            Stmt::Return(expr) => {
                let expr_str = self.gen_expr(expr, var_types)?;
                self.out.push_str(&format!("    ret_val = {};\n", expr_str));

                // Ownership transfer: null out local variables being returned
                if let Expr::Identifier(name) = expr {
                    let t = var_types.get(name);
                    if t == Some(&Type::List) {
                        self.out.push_str(&format!("    {} = 0;\n", name));
                    }
                }
                self.out.push_str("    goto L_cleanup;\n");
            }
            Stmt::Display(expr) => {
                let t = self.infer_type(expr, var_types);
                let expr_str = self.gen_expr(expr, var_types)?;
                if t == Type::Str || t == Type::DynStr {
                    self.out.push_str(&format!("    printf(\"%s\\n\", (char*){});\n", expr_str));
                } else {
                    self.out.push_str(&format!("    printf(\"%lld\\n\", {});\n", expr_str));
                }
            }
            Stmt::If(cond, then_branch, else_branch) => {
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
            Stmt::RepeatWhile(cond, body) => {
                let cond_str = self.gen_expr(cond, var_types)?;
                self.out.push_str(&format!("    while ({}) {{\n", cond_str));
                for s in body {
                    self.gen_statement(s, var_types)?;
                }
                self.out.push_str("    }\n");
            }
        }
        Ok(())
    }

    fn gen_expr(
        &mut self,
        expr: &Expr,
        var_types: &HashMap<String, Type>,
    ) -> Result<String, String> {
        match expr {
            Expr::Integer(val) => Ok(format!("{}", val)),
            Expr::StringLiteral(s) => {
                let escaped = s
                    .replace("\\", "\\\\")
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace("\r", "\\r");
                Ok(format!("(long long)\"{}\"", escaped))
            }
            Expr::Identifier(name) => Ok(name.clone()),
            Expr::Binary(left, op, right) => {
                let left_str = self.gen_expr(left, var_types)?;
                let right_str = self.gen_expr(right, var_types)?;
                let op_str = match op {
                    Op::Add => "+",
                    Op::Sub => "-",
                    Op::Mul => "*",
                    Op::Div => "/",
                };
                Ok(format!("({} {} {})", left_str, op_str, right_str))
            }
            Expr::Comparison(left, op, right) => {
                let left_str = self.gen_expr(left, var_types)?;
                let right_str = self.gen_expr(right, var_types)?;
                
                let is_string = self.infer_type(left, var_types) == Type::Str || self.infer_type(left, var_types) == Type::DynStr;
                if is_string {
                    let cmp_op = match op {
                        CompOp::LessThan => "< 0",
                        CompOp::GreaterThan => "> 0",
                        CompOp::Equals => "== 0",
                        CompOp::NotEquals => "!= 0",
                    };
                    Ok(format!("(strcmp((char*){}, (char*){}) {})", left_str, right_str, cmp_op))
                } else {
                    let op_str = match op {
                        CompOp::LessThan => "<",
                        CompOp::GreaterThan => ">",
                        CompOp::Equals => "==",
                        CompOp::NotEquals => "!=",
                    };
                    Ok(format!("({} {} {})", left_str, op_str, right_str))
                }
            }
            Expr::Logical(left, op, right) => {
                let left_str = self.gen_expr(left, var_types)?;
                let right_str = self.gen_expr(right, var_types)?;
                let op_str = match op {
                    LogicalOp::And => "&&",
                    LogicalOp::Or => "||",
                };
                Ok(format!("({} {} {})", left_str, op_str, right_str))
            }
            Expr::Call(name, args) => {
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

                let call_str = format!("{}({})", name, formatted_args.join(", "));
                
                match name.as_str() {
                    "read_file_content" | "get_argument" | "substring" | "string_from_list" | "ep_net_recv" | "ep_md5" | "ep_sha256" => {
                        Ok(format!("(long long){}", call_str))
                    }
                    _ => Ok(call_str),
                }
            }
        }
    }
}

const RUNTIME_HEADER_AND_SRC: &str = r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <netdb.h>

/* Networking standard library primitives */
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

/* SHA256 standard library implementation */
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

/* MD5 standard library implementation */
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
        return empty;
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
        return empty;
    }
    long long total_len = strlen(s);
    if (start < 0 || start >= total_len || len <= 0) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    if (start + len > total_len) {
        len = total_len - start;
    }
    char* sub = malloc(len + 1);
    if (!sub) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
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
        return empty;
    }
    char* s = malloc(list->length + 1);
    if (!s) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
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

long long display_string(const char* s) {
    if (s) puts(s);
    return 0;
}
"#;

const C_MAIN_BOOTSTRAPPER: &str = r#"
/* Bootstrapper C main */
int main(int argc, char** argv) {
    init_ep_args(argc, argv);
    _main();
    return 0;
}
"#;
