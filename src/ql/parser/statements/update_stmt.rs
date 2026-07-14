use crate::ql::parser::expression::{Expr, literal::Literal};

#[derive(Debug)]
pub struct UpdateStatement {
    pub target: String,
    pub set: Vec<(String, Literal)>,
    pub where_cond: Option<Expr>,
}
