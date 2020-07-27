pub type Id = String;

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Const(i64),
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
