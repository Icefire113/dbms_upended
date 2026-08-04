use crate::ql::{
    parser::expression::{
        error::{TokenToOperatorError, UnaryOperatorApplyError},
        literal::Literal,
        traits::UnaryApply,
    },
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

impl UnaryApply<Literal> for UnaryOp {
    type Error = UnaryOperatorApplyError;

    fn apply(&self, r: Literal) -> Result<Literal, Self::Error> {
        match self {
            Self::Minus => match r {
                Literal::Int(n) => Ok(Literal::Int(-n)),
                Literal::BigInt(n) => Ok(Literal::BigInt(-n)),
                Literal::Float(n) => Ok(Literal::Float(-n)),
                Literal::BigFloat(n) => Ok(Literal::BigFloat(-n)),
                _ => Err(UnaryOperatorApplyError::IncompatibleLiterals(r, *self)),
            },
            Self::Plus => Ok(r),
            Self::Not => match r {
                Literal::Bool(true) => Ok(Literal::Bool(false)),
                Literal::Bool(false) => Ok(Literal::Bool(true)),
                _ => Err(UnaryOperatorApplyError::IncompatibleLiterals(r, *self)),
            },
            Self::IsNull => match r {
                Literal::Null => Ok(Literal::Bool(true)),
                _ => Ok(Literal::Bool(false)),
            },
            Self::IsNotNull => match r {
                Literal::Null => Ok(Literal::Bool(false)),
                _ => Ok(Literal::Bool(true)),
            },
        }
    }
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
