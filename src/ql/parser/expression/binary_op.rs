use crate::ql::{
    parser::expression::{
        error::{BinaryOperatorApplyError, TokenToOperatorError},
        literal::Literal,
        traits::BinaryApply,
    },
    tokenizer::token::{Keyword, Operator, Token},
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
            Self::Plus => match (l, r) {
                (Literal::Int(l), Literal::Int(r)) => Ok(Literal::Int(l + r)),
                (Literal::Int(l), Literal::BigInt(r)) => Ok(Literal::BigInt(l as i64 + r)),
                (Literal::Int(l), Literal::Float(r)) => Ok(Literal::Float(l as f32 + r)),
                (Literal::Int(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l as f64 + r)),
                (Literal::BigInt(l), Literal::Int(r)) => Ok(Literal::BigInt(l + r as i64)),
                (Literal::BigInt(l), Literal::BigInt(r)) => Ok(Literal::BigInt(l + r)),
                (Literal::BigInt(l), Literal::Float(r)) => {
                    Ok(Literal::BigFloat(l as f64 + r as f64))
                }
                (Literal::BigInt(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l as f64 + r)),
                (Literal::Float(l), Literal::Int(r)) => Ok(Literal::Float(l + r as f32)),
                (Literal::Float(l), Literal::BigInt(r)) => {
                    Ok(Literal::BigFloat(l as f64 + r as f64))
                }
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l + r)),
                (Literal::Float(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l as f64 + r)),
                (Literal::BigFloat(l), Literal::Int(r)) => Ok(Literal::BigFloat(l + r as f64)),
                (Literal::BigFloat(l), Literal::BigInt(r)) => Ok(Literal::BigFloat(l + r as f64)),
                (Literal::BigFloat(l), Literal::Float(r)) => Ok(Literal::BigFloat(l + r as f64)),
                (Literal::BigFloat(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l + r)),
                (l, r) => Err(BinaryOperatorApplyError::IncompatibleLiterals(l, r, *self)),
            },
            Self::Minus => match (l, r) {
                (Literal::Int(l), Literal::Int(r)) => Ok(Literal::Int(l - r)),
                (Literal::Int(l), Literal::BigInt(r)) => Ok(Literal::BigInt(l as i64 - r)),
                (Literal::Int(l), Literal::Float(r)) => Ok(Literal::Float(l as f32 - r)),
                (Literal::Int(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l as f64 - r)),
                (Literal::BigInt(l), Literal::Int(r)) => Ok(Literal::BigInt(l - r as i64)),
                (Literal::BigInt(l), Literal::BigInt(r)) => Ok(Literal::BigInt(l - r)),
                (Literal::BigInt(l), Literal::Float(r)) => {
                    Ok(Literal::BigFloat(l as f64 - r as f64))
                }
                (Literal::BigInt(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l as f64 - r)),
                (Literal::Float(l), Literal::Int(r)) => Ok(Literal::Float(l - r as f32)),
                (Literal::Float(l), Literal::BigInt(r)) => {
                    Ok(Literal::BigFloat(l as f64 - r as f64))
                }
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l - r)),
                (Literal::Float(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l as f64 - r)),
                (Literal::BigFloat(l), Literal::Int(r)) => Ok(Literal::BigFloat(l - r as f64)),
                (Literal::BigFloat(l), Literal::BigInt(r)) => Ok(Literal::BigFloat(l - r as f64)),
                (Literal::BigFloat(l), Literal::Float(r)) => Ok(Literal::BigFloat(l - r as f64)),
                (Literal::BigFloat(l), Literal::BigFloat(r)) => Ok(Literal::BigFloat(l - r)),
                (l, r) => Err(BinaryOperatorApplyError::IncompatibleLiterals(l, r, *self)),
            },
            Self::Mul => todo!(),
            Self::Div => todo!(),
            Self::Mod => todo!(),
            Self::Pow => todo!(),
        }
    }
}

impl TryFrom<Token> for BinaryOp {
    type Error = TokenToOperatorError;

    fn try_from(tok: Token) -> Result<Self, Self::Error> {
        match tok {
            Token::Operator(Operator::Equals) => Ok(BinaryOp::Eq),
            Token::Operator(Operator::NotEq) => Ok(BinaryOp::NotEq),
            Token::Operator(Operator::Lt) => Ok(BinaryOp::Lt),
            Token::Operator(Operator::Lte) => Ok(BinaryOp::LtEq),
            Token::Operator(Operator::Gt) => Ok(BinaryOp::Gt),
            Token::Operator(Operator::Gte) => Ok(BinaryOp::GtEq),
            Token::Operator(Operator::Plus) => Ok(BinaryOp::Plus),
            Token::Operator(Operator::Minus) => Ok(BinaryOp::Minus),
            Token::Operator(Operator::Star) => Ok(BinaryOp::Mul),
            Token::Operator(Operator::Divide) => Ok(BinaryOp::Div),
            Token::Operator(Operator::Modulus) => Ok(BinaryOp::Mod),
            Token::Operator(Operator::Pow) => Ok(BinaryOp::Pow),
            Token::Keyword(Keyword::And) => Ok(BinaryOp::And),
            Token::Keyword(Keyword::Or) => Ok(BinaryOp::Or),
            other => Err(TokenToOperatorError::InvalidOperator(other.clone())),
        }
    }
}
