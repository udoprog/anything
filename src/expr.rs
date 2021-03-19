use crate::db;

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A constant expression.
    Constant(Box<db::Constant>),
    /// A binary expression.
    Binary(Box<ExprBinary>),
}

/// The operation of a binary expression.
#[derive(Debug, Clone, Copy)]
pub enum ExprBinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
}

/// A binary expression.
#[derive(Debug)]
pub struct ExprBinary {
    /// The operation of a binary expression.
    pub op: ExprBinaryOp,
    /// The left-hand side of the expression.
    pub lhs: Expr,
    /// The right-hand side of the expression.
    pub rhs: Expr,
}
