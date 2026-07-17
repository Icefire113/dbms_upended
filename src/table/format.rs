use bitcode::{Decode, Encode};

use crate::ql::{parser::error::QLParseError, tokenizer::token::Keyword};



#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub enum ColumnType {
    Int,
    BigInt,
    Float,
    BigFloat,
    Bool,
    String,
}

impl TryFrom<Keyword> for ColumnType {
    type Error = QLParseError;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Int => Ok(ColumnType::Int),
            Keyword::BigInt => Ok(ColumnType::BigInt),
            Keyword::Float => Ok(ColumnType::Float),
            Keyword::BigFloat => Ok(ColumnType::BigFloat),
            Keyword::String => Ok(ColumnType::String),
            Keyword::Bool => Ok(ColumnType::Bool),
            _ => Err(Self::Error::KeywordIsNotColumnType(value)),
        }
    }
}
