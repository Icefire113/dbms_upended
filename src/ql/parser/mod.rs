use std::{iter::Peekable, slice::Iter};
use tracing::trace;

use crate::ql::{
    parser::{
        column_type::ColumnType,
        error::QLParseError,
        expression::{Expr, literal::Literal, unary_op::UnaryOp},
        statement::QLStatement,
        statements::{
            alter_stmt::{
                AlterAction, AlterActionModifiers, AlterMode, AlterObject, AlterStatement,
            },
            create_stmt::{ColumnModifiers, CreateStatement, CreateType},
            delete_stmt::DeleteStatement,
            drop_stmt::{DropStatement, DropType},
            insert_stmt::InsertStatement,
            load_stmt::LoadStatement,
            select_stmt::{JoinType, SelectStatement},
            update_stmt::UpdateStatement,
            use_stmt::UseStatement,
        },
    },
    tokenizer::token::{
        Keyword, LiteralToken, Operator, Token, TokenType, get_prefix_bp, get_token_bp,
    },
};

pub mod column_type;
pub mod error;
pub mod expression;
pub mod statement;
pub mod statements;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            pos: 0,
        }
    }

    /// Consumes this parser and parses our token list into a list of statements
    pub fn parse(mut self) -> Result<Vec<QLStatement>, QLParseError> {
        let mut queries: Vec<QLStatement> = Vec::new();
        while let Some(_) = self.tokens.peek() {
            let q: QLStatement = self.parse_single_query()?;
            trace!("Parsed query: {:#?}", q);
            queries.push(q);
        }
        Ok(queries)
    }

    fn parse_single_query(&mut self) -> Result<QLStatement, QLParseError> {
        match self.advance() {
            Some(tok) => match &tok.token_type {
                TokenType::Keyword(keyword) => match keyword {
                    Keyword::Create => self.parse_create(),
                    Keyword::Select => self.parse_select(),
                    Keyword::Insert => self.parse_insert(),
                    Keyword::Update => self.parse_update(),
                    Keyword::Delete => self.parse_delete(),
                    Keyword::Alter => self.parse_alter(),
                    Keyword::Drop => self.parse_drop(),
                    Keyword::Use => self.parse_use(),
                    Keyword::Load => self.parse_load(),
                    _ => Err(QLParseError::IllegalToken(tok.token_type.clone(), self.pos)),
                },
                TokenType::Unknown(_) => {
                    Err(QLParseError::UnknownToken(tok.token_type.clone(), self.pos))
                }
                token_type => Err(QLParseError::IllegalToken(token_type.clone(), self.pos)),
            },
            None => Err(QLParseError::EOF),
        }
    }

    // ===========================================================================
    // Individual query parsers
    // ===========================================================================

    fn parse_create(&mut self) -> Result<QLStatement, QLParseError> {
        match self.expect_one_of_keywords(&[Keyword::Table, Keyword::Index, Keyword::Database])? {
            Keyword::Table => {
                let tbl_name: String = self.expect_ident()?.to_owned();
                self.expect_token(TokenType::LParen)?;
                let mut columns: Vec<(String, ColumnType, ColumnModifiers)> = Vec::new();
                loop {
                    columns.push(self.expect_column_decl()?);

                    // After a column: either a comma (more columns follow) or a closing paren (done).
                    match self.expect_one_of_tokens(&[TokenType::Comma, TokenType::RParen])? {
                        TokenType::Comma => continue,
                        TokenType::RParen => break,
                        _ => unreachable!(),
                    }
                }
                self.expect_end_of_query()?;
                Ok(QLStatement::Create(CreateStatement {
                    create_type: CreateType::Table(columns),
                    name: tbl_name,
                }))
            }
            Keyword::Database => {
                let db_name: String = self.expect_ident()?.to_owned();
                self.expect_end_of_query()?;
                Ok(QLStatement::Create(CreateStatement {
                    create_type: CreateType::Database,
                    name: db_name,
                }))
            }
            Keyword::Index => {
                let idx_name: String = self.expect_ident()?.to_owned();
                self.expect_keyword(Keyword::On)?;
                let tbl_and_col: String = self.expect_ident()?.to_owned();
                self.expect_end_of_query()?;
                Ok(QLStatement::Create(CreateStatement {
                    create_type: CreateType::Index(tbl_and_col),
                    name: idx_name,
                }))
            }
            _ => unreachable!(),
        }
    }

    fn parse_select(&mut self) -> Result<QLStatement, QLParseError> {
        let filter_cols: Option<Vec<String>> = if self.expect_operator(Operator::Star).is_ok() {
            None
        } else {
            let mut cols: Vec<String> = Vec::new();
            while let Ok(ident) = self.expect_ident() {
                cols.push(ident.to_owned());
                if self.expect_token(TokenType::Comma).is_err() {
                    break;
                }
            }
            Some(cols)
        };

        self.expect_keyword(Keyword::From)?;
        let primary_table: String = self.expect_ident()?.to_owned();

        let mut joins: Vec<(String, JoinType)> = Vec::new();

        while let Ok(kw) = self.expect_one_of_keywords(&[
            Keyword::Inner,
            Keyword::Right,
            Keyword::Left,
            Keyword::Full,
            Keyword::Cross,
        ]) {
            self.expect_keyword(Keyword::Join)?;
            let joined_table: String = self.expect_ident()?.to_owned();
            match kw {
                Keyword::Inner => {
                    self.expect_keyword(Keyword::On)?;
                    joins.push((joined_table, JoinType::Inner(self.parse_expr(0)?)))
                }
                Keyword::Right => {
                    self.expect_keyword(Keyword::On)?;
                    joins.push((joined_table, JoinType::Right(self.parse_expr(0)?)))
                }
                Keyword::Left => {
                    self.expect_keyword(Keyword::On)?;
                    joins.push((joined_table, JoinType::Left(self.parse_expr(0)?)))
                }
                Keyword::Full => {
                    self.expect_keyword(Keyword::On)?;
                    joins.push((joined_table, JoinType::Full(self.parse_expr(0)?)))
                }
                Keyword::Cross => joins.push((joined_table, JoinType::Cross)),
                _ => unreachable!(),
            }
        }

        let where_cond: Option<Expr> = if self.expect_keyword(Keyword::Where).is_ok() {
            Some(self.parse_expr(0)?)
        } else {
            None
        };
        self.expect_end_of_query()?;

        Ok(QLStatement::Select(SelectStatement {
            filter_cols,
            primary_table,
            where_clause: where_cond,
            joins,
        }))
    }

    fn parse_load(&mut self) -> Result<QLStatement, QLParseError> {
        let tbl_name = self.expect_ident()?.to_owned();
        self.expect_keyword(Keyword::From)?;

        let file_path = self.expect_literal_string()?;
        self.expect_end_of_query()?;
        Ok(QLStatement::Load(LoadStatement {
            target: tbl_name,
            file_path,
        }))
    }

    fn parse_insert(&mut self) -> Result<QLStatement, QLParseError> {
        self.expect_keyword(Keyword::Into)?;
        let tbl_name: String = self.expect_ident()?.to_owned();
        let mut cols: Option<Vec<String>> = None;

        if !self.peek_is_keyword(&[Keyword::Values]) {
            cols = Some(self.expect_identifier_list()?);
        }
        self.expect_keyword(Keyword::Values)?;
        let mut rows: Vec<Vec<Literal>> = Vec::new();
        while let Ok(row) = self.expect_literal_list() {
            rows.push(row);
            if self.expect_token(TokenType::Comma).is_err() {
                break;
            }
        }

        self.expect_end_of_query()?;
        Ok(QLStatement::Insert(InsertStatement {
            name: tbl_name,
            columns: cols,
            values: rows,
        }))
    }

    fn parse_update(&mut self) -> Result<QLStatement, QLParseError> {
        let tbl_name = self.expect_ident()?.to_owned();
        self.expect_keyword(Keyword::Set)?;
        let mut cols: Vec<(String, Literal)> = Vec::new();
        loop {
            let col_name = self.expect_ident()?.to_owned();
            self.expect_operator(Operator::Equals)?;
            let val = self.expect_literal()?;
            cols.push((col_name, val));
            if self.expect_token(TokenType::Comma).is_err() {
                break;
            }
        }
        let mut where_cond: Option<Expr> = None;
        if self.expect_keyword(Keyword::Where).is_ok() {
            where_cond = Some(self.parse_expr(0)?);
        }
        self.expect_end_of_query()?;

        Ok(QLStatement::Update(UpdateStatement {
            target: tbl_name,
            set: cols,
            where_cond,
        }))
    }

    fn parse_delete(&mut self) -> Result<QLStatement, QLParseError> {
        self.expect_keyword(Keyword::From)?;
        let tbl_name: String = self.expect_ident()?.to_owned();
        self.expect_keyword(Keyword::Where)?;
        let where_cond = self.parse_expr(0)?;
        dbg!(&where_cond);
        self.expect_end_of_query()?;
        Ok(QLStatement::Delete(DeleteStatement {
            name: tbl_name,
            where_cond,
        }))
    }

    fn parse_alter(&mut self) -> Result<QLStatement, QLParseError> {
        match self.expect_one_of_keywords(&[Keyword::Table, Keyword::Column])? {
            Keyword::Table => {
                let tbl_name: String = self.expect_ident()?.to_owned();
                let mode: AlterMode =
                    match self.expect_one_of_keywords(&[Keyword::Add, Keyword::Drop])? {
                        Keyword::Add => AlterMode::Add,
                        Keyword::Drop => AlterMode::Drop,
                        _ => unreachable!(),
                    };
                self.expect_keyword(Keyword::Column)?;

                if mode == AlterMode::Add {
                    let (col_name, col_type, col_modifs) = self.expect_column_decl()?;
                    self.expect_end_of_query()?;

                    Ok(QLStatement::Alter(AlterStatement {
                        object: AlterObject::Table,
                        object_name: tbl_name,
                        mode,
                        action: AlterAction::AddColumn(col_name, col_type, col_modifs),
                    }))
                } else {
                    // AlterMode::Drop
                    let col_name = self.expect_ident()?.to_owned();
                    self.expect_end_of_query()?;

                    Ok(QLStatement::Alter(AlterStatement {
                        object: AlterObject::Table,
                        object_name: tbl_name,
                        mode,
                        action: AlterAction::DropColumn(col_name),
                    }))
                }
            }
            Keyword::Column => {
                let column_name: String = self.expect_ident()?.to_owned();
                let mode: AlterMode =
                    match self.expect_one_of_keywords(&[Keyword::Add, Keyword::Drop])? {
                        Keyword::Add => AlterMode::Add,
                        Keyword::Drop => AlterMode::Drop,
                        _ => unreachable!(),
                    };
                self.expect_keyword(Keyword::Modifier)?;

                let modifier =
                    match self.expect_one_of_keywords(&[Keyword::Unique, Keyword::Nullable])? {
                        Keyword::Unique => AlterActionModifiers::Unique,
                        Keyword::Nullable => AlterActionModifiers::Nullable,
                        _ => unreachable!(),
                    };
                self.expect_end_of_query()?;

                Ok(QLStatement::Alter(AlterStatement {
                    object: AlterObject::Column,
                    object_name: column_name,
                    mode,
                    action: AlterAction::Modifier(modifier),
                }))
            }
            _ => unreachable!(),
        }
    }

    fn parse_drop(&mut self) -> Result<QLStatement, QLParseError> {
        let drop_type = match self.expect_one_of_keywords(&[
            Keyword::Table,
            Keyword::Database,
            Keyword::Index,
        ])? {
            Keyword::Table => DropType::Table,
            Keyword::Database => DropType::Database,
            Keyword::Index => DropType::Index,
            _ => unreachable!(),
        };
        let target = self.expect_ident()?.to_owned();

        self.expect_end_of_query()?;

        Ok(QLStatement::Drop(DropStatement { drop_type, target }))
    }

    fn parse_use(&mut self) -> Result<QLStatement, QLParseError> {
        let target = self.expect_ident()?.to_owned();
        self.expect_end_of_query()?;
        Ok(QLStatement::Use(UseStatement { target }))
    }

    // ===========================================================================
    // Expression Parsing
    // ===========================================================================

    /// Parses an expression via pratt parsing
    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, QLParseError> {
        // first grab the left hand side of the expression
        let mut root = self.parse_prefix()?;

        // then while we can keep grabbing more things which a left binding power that is greater than
        // the minimum, we keep adding them to the expression tree
        loop {
            let op = match self.peek() {
                Some(t) => t.token_type.clone(),
                None => break,
            };

            // Handle special cases like `is null` or `is not null`
            if op == TokenType::Keyword(Keyword::Is) {
                self.advance();
                let l_bind_pow = 7u8;
                if l_bind_pow < min_bp {
                    break;
                }

                let op = match self.expect_one_of_keywords(&[Keyword::Null, Keyword::Not])? {
                    Keyword::Null => UnaryOp::IsNull,
                    Keyword::Not => {
                        self.expect_keyword(Keyword::Null)?;
                        UnaryOp::IsNotNull
                    }
                    _ => unreachable!(),
                };
                root = Expr::UnaryOp {
                    op,
                    expr: Box::new(root),
                };
                continue;
            }

            if let Some((l_bind_pow, r_bind_pow)) = get_token_bp(&op) {
                if l_bind_pow < min_bp {
                    break;
                }
                // consume op
                self.advance();
                root = Expr::BinaryOp {
                    lhs: Box::new(root),
                    op: op.try_into()?,
                    rhs: Box::new(self.parse_expr(r_bind_pow)?),
                }
            } else {
                break;
            }
        }

        Ok(root)
    }

    /// Parses a single expression literal (identifier, literal) or a unary operator
    /// basically anything that could be on the left hand side of an expression
    fn parse_prefix(&mut self) -> Result<Expr, QLParseError> {
        if let Ok(lit) = self.expect_literal() {
            Ok(Expr::Literal(lit))
        } else if self.expect_keyword(Keyword::Not).is_ok() {
            let bind_pow = get_prefix_bp(&TokenType::Keyword(Keyword::Not))
                .expect("Token does not have a registered binding power");
            let expr = self.parse_expr(bind_pow)?;
            Ok(Expr::UnaryOp {
                op: TokenType::Keyword(Keyword::Not).try_into()?,
                expr: Box::new(expr),
            })
        } else if self.expect_operator(Operator::Minus).is_ok() {
            let bind_pow = get_prefix_bp(&TokenType::Operator(Operator::Minus))
                .expect("Token does not have a registered binding power");
            let expr = self.parse_expr(bind_pow)?;
            Ok(Expr::UnaryOp {
                op: TokenType::Operator(Operator::Minus).try_into()?,
                expr: Box::new(expr),
            })
        } else if self.expect_token(TokenType::LParen).is_ok() {
            let expr = self.parse_expr(0)?;
            self.expect_token(TokenType::RParen)?;
            Ok(expr)
        } else if let Ok(ident) = self.expect_ident() {
            Ok(Expr::Identifier(ident.to_owned()))
        } else if let Some(t) = self.peek() {
            Err(QLParseError::UnknownToken(t.token_type.clone(), self.pos))
        } else {
            Err(QLParseError::EOF)
        }
    }

    // ===========================================================================
    // Helpers
    // ===========================================================================

    fn expect_column_decl(
        &mut self,
    ) -> Result<(String, ColumnType, ColumnModifiers), QLParseError> {
        let col_name = self.expect_ident()?.to_owned();
        let col_type = self
            .expect_one_of_keywords(&[
                Keyword::Int,
                Keyword::BigInt,
                Keyword::Float,
                Keyword::BigFloat,
                Keyword::String,
                Keyword::Bool,
            ])?
            .try_into()
            .unwrap();

        let mut col_mods: ColumnModifiers = ColumnModifiers {
            nullable: false,
            unique: false,
            indexed: None,
        };

        // Peek before consuming, so a non-modifier token (comma/rparen) is left intact.
        while self.peek_is_keyword(&[Keyword::Nullable, Keyword::Indexed, Keyword::Unique]) {
            let tok = self.expect_one_of_keywords(&[
                Keyword::Nullable,
                Keyword::Indexed,
                Keyword::Unique,
            ])?;
            match tok {
                Keyword::Nullable => col_mods.nullable = true,
                Keyword::Unique => col_mods.unique = true,
                Keyword::Indexed => {
                    col_mods.indexed = Some(self.expect_ident()?.to_owned());
                }
                _ => unreachable!(),
            }
        }
        Ok((col_name, col_type, col_mods))
    }

    /// Consumes the next sequence of tokens that looks like a list of literals
    /// (basically a comma seperated list of literal values with parenthesis around them)
    fn expect_literal_list(&mut self) -> Result<Vec<Literal>, QLParseError> {
        let mut lits: Vec<Literal> = Vec::new();
        self.expect_token(TokenType::LParen)?;
        while let Ok(lit) = self.expect_literal() {
            lits.push(lit);
            if self.expect_token(TokenType::Comma).is_err() {
                break;
            }
        }
        self.expect_token(TokenType::RParen)?;
        Ok(lits)
    }

    fn expect_identifier_list(&mut self) -> Result<Vec<String>, QLParseError> {
        self.expect_token(TokenType::LParen)?;
        let mut idents: Vec<String> = Vec::new();
        while let Ok(ident) = self.expect_ident() {
            idents.push(ident.to_owned());
            if self.expect_token(TokenType::Comma).is_err() {
                break;
            }
        }
        self.expect_token(TokenType::RParen)?;

        Ok(idents)
    }

    fn expect_literal(&mut self) -> Result<Literal, QLParseError> {
        if let Ok(b) = self.expect_literal_bool() {
            Ok(b.into())
        } else if let Ok(s) = self.expect_literal_string() {
            Ok(s.into())
        } else if let Ok(n) = self.expect_literal_i32() {
            Ok(n.into())
        } else if let Ok(n) = self.expect_literal_i64() {
            Ok(n.into())
        } else if let Ok(n) = self.expect_literal_f32() {
            Ok(n.into())
        } else if let Ok(n) = self.expect_literal_f64() {
            Ok(n.into())
        } else if let Ok(_) = self.expect_literal_null() {
            Ok(Literal::Null)
        } else {
            Err(QLParseError::ExpectedLiteral(self.pos))
        }
    }

    fn expect_literal_null(&mut self) -> Result<(), QLParseError> {
        match self.peek() {
            Some(tok) if matches!(tok.token_type, TokenType::Keyword(Keyword::Null)) => {
                self.advance();
                Ok(())
            }
            _ => Err(QLParseError::ExpectedLiteralNull(self.pos)),
        }
    }

    fn expect_literal_bool(&mut self) -> Result<bool, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Keyword(Keyword::True)) => {
                self.advance();
                Ok(true)
            }
            Some(TokenType::Keyword(Keyword::False)) => {
                self.advance();
                Ok(false)
            }
            _ => Err(QLParseError::ExpectedLiteralBool(self.pos)),
        }
    }

    fn expect_literal_f64(&mut self) -> Result<f64, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Literal(LiteralToken::BigFloat(n))) => {
                self.advance();
                Ok(*n)
            }
            _ => Err(QLParseError::ExpectedLiteralBigFloat(self.pos)),
        }
    }

    fn expect_literal_f32(&mut self) -> Result<f32, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Literal(LiteralToken::Float(n))) => {
                self.advance();
                Ok(*n)
            }
            _ => Err(QLParseError::ExpectedLiteralFloat(self.pos)),
        }
    }

    fn expect_literal_i64(&mut self) -> Result<i64, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Literal(LiteralToken::BigInt(n))) => {
                self.advance();
                Ok(*n)
            }
            _ => Err(QLParseError::ExpectedLiteralBigInt(self.pos)),
        }
    }

    fn expect_literal_i32(&mut self) -> Result<i32, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Literal(LiteralToken::Int(n))) => {
                self.advance();
                Ok(*n)
            }
            _ => Err(QLParseError::ExpectedLiteralInt(self.pos)),
        }
    }

    fn expect_literal_string(&mut self) -> Result<String, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Literal(LiteralToken::String(s))) => {
                self.advance();
                Ok(s.clone())
            }
            _ => Err(QLParseError::ExpectedLiteralString(self.pos)),
        }
    }

    fn expect_token(&mut self, expected: TokenType) -> Result<TokenType, QLParseError> {
        match self.peek() {
            Some(tok) if tok.token_type == expected => {
                self.advance();
                Ok(expected)
            }
            _ => Err(QLParseError::ExpectedToken(expected, self.pos)),
        }
    }

    fn expect_operator(&mut self, expected: Operator) -> Result<Operator, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Operator(op)) if *op == expected => {
                self.advance();
                Ok(expected)
            }
            _ => Err(QLParseError::ExpectedOperator(expected, self.pos)),
        }
    }

    /// Expects that the next token is an identifier (either an identifier or a quoted identifier)
    fn expect_ident(&mut self) -> Result<&str, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Identifier(_)) | Some(TokenType::QuotedIdentifier(_)) => {
                let tok = self.advance().unwrap();
                match &tok.token_type {
                    TokenType::Identifier(ident) | TokenType::QuotedIdentifier(ident) => Ok(ident),
                    _ => unreachable!(),
                }
            }
            _ => Err(QLParseError::ExpectedIdent(self.pos)),
        }
    }

    fn expect_keyword(&mut self, expected_kw: Keyword) -> Result<(), QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Keyword(keyword)) if *keyword == expected_kw => {
                self.advance();
                Ok(())
            }
            _ => Err(QLParseError::ExpectedKeyword(expected_kw, self.pos)),
        }
    }

    fn expect_end_of_query(&mut self) -> Result<(), QLParseError> {
        match self.peek() {
            Some(tok) if tok.token_type == TokenType::SemiColon => {
                self.advance();
                Ok(())
            }
            Some(_) => Err(QLParseError::ExpectedEndOfQuery(self.pos)),
            None => Ok(()),
        }
    }

    fn expect_one_of_keywords(&mut self, kws: &[Keyword]) -> Result<Keyword, QLParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Keyword(keyword)) if kws.contains(keyword) => {
                let kw = *keyword;
                self.advance();
                Ok(kw)
            }
            _ => Err(QLParseError::ExpectedOneOfKeywords(kws.to_vec(), self.pos)),
        }
    }

    fn expect_one_of_tokens(&mut self, expected: &[TokenType]) -> Result<TokenType, QLParseError> {
        match self.peek() {
            Some(tok) if expected.iter().any(|e| *e == tok.token_type) => {
                let tt = tok.token_type.clone();
                self.advance();
                Ok(tt)
            }
            _ => Err(QLParseError::ExpectedOneOfTokens(
                expected.to_vec(),
                self.pos,
            )),
        }
    }

    /// Non-consuming check: is the next token a keyword in `kws`?
    fn peek_is_keyword(&mut self, kws: &[Keyword]) -> bool {
        matches!(
            self.tokens.peek(),
            Some(tok) if matches!(&tok.token_type, TokenType::Keyword(k) if kws.contains(k))
        )
    }

    fn peek(&mut self) -> Option<&&'a Token> {
        self.tokens.peek()
    }

    /// Consumes the next token in the stream and returns it while incrementing
    /// the position counter
    fn advance(&mut self) -> Option<&'a Token> {
        match self.tokens.next() {
            Some(t) => {
                self.pos += 1;
                Some(t)
            }
            None => None,
        }
    }
}
