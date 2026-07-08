use crate::ql::{
    parser::expression::error::TokenToOperatorError,
    tokenizer::token::{Keyword, Operator, TokenType},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Minus,
    Plus,
    Not,
    IsNull,
    IsNotNull,
}

impl TryFrom<TokenType> for UnaryOp {
    type Error = TokenToOperatorError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Operator(Operator::Minus) => Ok(UnaryOp::Minus),
            TokenType::Operator(Operator::Plus) => Ok(UnaryOp::Plus),
            TokenType::Keyword(Keyword::Not) => Ok(UnaryOp::Not),
            other => Err(TokenToOperatorError::InvalidOperator(other.clone())),
        }
    }
}
