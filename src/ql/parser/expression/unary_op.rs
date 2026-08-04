use crate::ql::{
    parser::expression::error::TokenToOperatorError,
    tokenizer::token::{Keyword, Operator, Token},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Minus,
    Plus,
    Not,
    IsNull,
    IsNotNull,
}

impl TryFrom<Token> for UnaryOp {
    type Error = TokenToOperatorError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Operator(Operator::Minus) => Ok(UnaryOp::Minus),
            Token::Operator(Operator::Plus) => Ok(UnaryOp::Plus),
            Token::Keyword(Keyword::Not) => Ok(UnaryOp::Not),
            other => Err(TokenToOperatorError::InvalidOperator(other.clone())),
        }
    }
}
