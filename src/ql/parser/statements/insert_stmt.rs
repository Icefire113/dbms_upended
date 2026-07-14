use crate::ql::parser::expression::literal::Literal;

#[derive(Debug)]
pub struct InsertStatement {
    /// The name of the table we are inserting into
    pub name: String,
    pub columns: Option<Vec<String>>,
    pub values: Vec<Vec<Literal>>,
}
