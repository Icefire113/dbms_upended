use crate::ql::parser::expression::Expr;

#[derive(Debug)]
pub struct DeleteStatement {
    pub name: String,
    pub where_cond: Expr,
}
