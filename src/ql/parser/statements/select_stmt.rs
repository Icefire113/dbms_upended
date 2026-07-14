use crate::ql::parser::expression::Expr;

#[derive(Debug)]
pub struct SelectStatement {
    /// If `None` then all columns are selected
    pub filter_cols: Option<Vec<String>>,
    pub primary_table: String,
    pub joins: Vec<(String, JoinType)>,
    pub where_clause: Option<Expr>,
}

#[derive(Debug)]
pub enum JoinType {
    Inner(Expr),
    Right(Expr),
    Left(Expr),
    Full(Expr),
    Cross,
}
