use crate::ql::parser::expression::{binary_op::BinaryOp, literal::Literal, unary_op::UnaryOp};

pub mod binary_op;
pub mod error;
pub mod literal;
pub mod traits;
pub mod unary_op;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),

    BinaryOp {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    },

    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
}
