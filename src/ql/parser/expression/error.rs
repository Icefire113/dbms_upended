use thiserror::Error;

use crate::ql::{
    parser::expression::{binary_op::BinaryOp, literal::Literal, unary_op::UnaryOp},
    tokenizer::token::Token,
};

#[derive(Debug, Error)]
pub enum BinaryOperatorApplyError {
    #[error("Incompatible literals {0:?} and {1:?} for binary operator {2:?}")]
    IncompatibleLiterals(Literal, Literal, BinaryOp),
}

#[derive(Debug, Error)]
pub enum UnaryOperatorApplyError {
    #[error("Incompatible literal {0:?} for unary operator {1:?}")]
    IncompatibleLiterals(Literal, UnaryOp),
}

#[derive(Debug, Error)]
pub enum TokenToOperatorError {
    #[error("Invalid operator {0:?}")]
    InvalidOperator(Token),
}
