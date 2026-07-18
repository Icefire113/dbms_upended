use crate::ql::parser::column_type::ColumnType;

#[derive(Debug)]
pub struct CreateStatement {
    pub create_type: CreateType,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnModifiers {
    pub nullable: bool,
    pub unique: bool,
    pub indexed: Option<String>,
}

#[derive(Debug)]
pub enum CreateType {
    /// Contains the identifier of the index: `table_name.column_name`
    Index(String),
    /// Contains a list of columns in the form of `(name, type, modifiers)`
    Table(Vec<(String, ColumnType, ColumnModifiers)>),
    Database,
}
