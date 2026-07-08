use crate::ql::{
    parser::expression::{
        error::{BinaryOperatorApplyError, TokenToOperatorError},
        literal::Literal,
        traits::BinaryApply,
    },
    tokenizer::token::{Keyword, Operator, TokenType},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
}

impl BinaryApply<Literal> for BinaryOp {
    type Error = BinaryOperatorApplyError;

    fn apply(&self, l: Literal, r: Literal) -> Result<Literal, Self::Error> {
        match self {
            Self::Eq => Ok(Literal::Bool(l == r)),
            Self::NotEq => Ok(Literal::Bool(l != r)),
            Self::Lt => Ok(Literal::Bool(l < r)),
            Self::LtEq => Ok(Literal::Bool(l <= r)),
            Self::Gt => Ok(Literal::Bool(l > r)),
            Self::GtEq => Ok(Literal::Bool(l >= r)),
            Self::And => match (l, r) {
                (Literal::Bool(l), Literal::Bool(r)) => Ok(Literal::Bool(l && r)),
                (l, r) => Err(BinaryOperatorApplyError::IncompatibleLiterals(l, r, *self)),
            },
            Self::Or => match (l, r) {
                (Literal::Bool(l), Literal::Bool(r)) => Ok(Literal::Bool(l || r)),
                (l, r) => Err(BinaryOperatorApplyError::IncompatibleLiterals(l, r, *self)),
            },
            Self::Plus => todo!(),
            Self::Minus => todo!(),
            Self::Mul => todo!(),
            Self::Div => todo!(),
            Self::Mod => todo!(),
            Self::Pow => todo!(),
        }
    }
}

impl TryFrom<TokenType> for BinaryOp {
    type Error = TokenToOperatorError;

    fn try_from(tok: TokenType) -> Result<Self, Self::Error> {
        match tok {
            TokenType::Operator(Operator::Equals) => Ok(BinaryOp::Eq),
            TokenType::Operator(Operator::NotEq) => Ok(BinaryOp::NotEq),
            TokenType::Operator(Operator::Lt) => Ok(BinaryOp::Lt),
            TokenType::Operator(Operator::Lte) => Ok(BinaryOp::LtEq),
            TokenType::Operator(Operator::Gt) => Ok(BinaryOp::Gt),
            TokenType::Operator(Operator::Gte) => Ok(BinaryOp::GtEq),
            TokenType::Operator(Operator::Plus) => Ok(BinaryOp::Plus),
            TokenType::Operator(Operator::Minus) => Ok(BinaryOp::Minus),
            TokenType::Operator(Operator::Star) => Ok(BinaryOp::Mul),
            TokenType::Operator(Operator::Divide) => Ok(BinaryOp::Div),
            TokenType::Operator(Operator::Modulus) => Ok(BinaryOp::Mod),
            TokenType::Keyword(Keyword::And) => Ok(BinaryOp::And),
            TokenType::Keyword(Keyword::Or) => Ok(BinaryOp::Or),
            other => Err(TokenToOperatorError::InvalidOperator(other.clone())),
        }
    }
}
