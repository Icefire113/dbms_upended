use thiserror::Error;

use crate::ql::{
    parser::expression::{binary_op::BinaryOp, literal::Literal},
    tokenizer::token::TokenType,
};

#[derive(Debug, Error)]
pub enum BinaryOperatorApplyError {
    #[error("Incompatible literals {0:?} and {1:?} for binary operator {2:?}")]
    IncompatibleLiterals(Literal, Literal, BinaryOp),
}

#[derive(Debug, Error)]
pub enum TokenToOperatorError {
    #[error("Invalid operator {0:?}")]
    InvalidOperator(TokenType),
}
