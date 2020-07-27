pub type Id = String;

#[derive(Clone, Debug)]
pub enum Expr {
    Const(i64),
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub body: Vec<Expr>,
}