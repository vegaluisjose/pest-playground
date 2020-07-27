use std::rc::Rc;

pub type Id = String;

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Const(i64),
    BinOp(Op, Rc<Expr>, Rc<Expr>),
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub id: Id,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub body: Vec<Stmt>,
}
