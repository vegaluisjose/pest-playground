use crate::ast::*;
use std::fmt;

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = match self {
            Expr::Ref(n) => n.to_string(),
            Expr::Const(n) => n.to_string(),
            Expr::BinOp(op, lhs, rhs) => {
                format!("{} {} {}", lhs.to_string(), op.to_string(), rhs.to_string())
            }
        };
        write!(f, "{}", expr)
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {};", self.id, self.expr)
    }
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut body = String::new();
        for stmt in self.body.iter() {
            body.push_str(&stmt.to_string());
            body.push_str("\n");
        }
        write!(f, "{}", body)
    }
}