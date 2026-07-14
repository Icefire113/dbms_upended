use crate::ql::parser::expression::Expr;

#[derive(Debug)]
pub struct SelectStatement {
    /// If `None` then all columns are selected
    pub filter_cols: Option<Vec<String>>,
    pub targets: Vec<String>,
    pub where_clause: Option<Expr>,
}
