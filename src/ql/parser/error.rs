use thiserror::Error;

use crate::ql::{
    parser::expression::error::TokenToOperatorError,
    tokenizer::token::{Keyword, Operator, TokenType},
};

#[derive(Debug, Error)]
pub enum QLParseError {
    #[error("EOF")]
    EOF,

    #[error("Expected one of keywords {:?} at token position {}", _0, _1)]
    ExpectedOneOfKeywords(Vec<Keyword>, usize),

    #[error("Expected keyword {:?} at token position {}", _0, _1)]
    ExpectedKeyword(Keyword, usize),

    #[error("Unknown token {:?} at token position {}", _0, _1)]
    UnknownToken(TokenType, usize),

    #[error("Illegal token {:?} at token position {}", _0, _1)]
    IllegalToken(TokenType, usize),

    #[error("Expected identifier at token position {}", _0)]
    ExpectedIdent(usize),

    #[error("Expected one of tokens {:?} at token position {}", _0, _1)]
    ExpectedOneOfTokens(Vec<TokenType>, usize),

    #[error("Expected token {:?} at token position {}", _0, _1)]
    ExpectedToken(TokenType, usize),

    #[error("Keyword {:?} is not a valid column type", _0)]
    KeywordIsNotColumnType(Keyword),

    /// Either a semicolon or EOF
    #[error("Expected end of query at token position {}", _0)]
    ExpectedEndOfQuery(usize),

    #[error("Expected literal string at token position {}", _0)]
    ExpectedLiteralString(usize),

    #[error("Expected literal int at token position {}", _0)]
    ExpectedLiteralInt(usize),

    #[error("Expected literal bigint at token position {}", _0)]
    ExpectedLiteralBigInt(usize),

    #[error("Expected literal float at token position {}", _0)]
    ExpectedLiteralFloat(usize),

    #[error("Expected literal bigfloat at token position {}", _0)]
    ExpectedLiteralBigFloat(usize),

    #[error("Expected literal bool at token position {}", _0)]
    ExpectedLiteralBool(usize),

    #[error("Expected literal null at token position {}", _0)]
    ExpectedLiteralNull(usize),

    #[error("Expected literal value at token position {}", _0)]
    ExpectedLiteral(usize),

    #[error("Expected operator {:?} at token position {}", _0, _1)]
    ExpectedOperator(Operator, usize),

    #[error("Could not convert token to operator")]
    TokenToOperatorConversionError(#[from] TokenToOperatorError),
}
