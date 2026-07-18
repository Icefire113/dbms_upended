use crate::ql::parser::{column_type::ColumnType, statements::create_stmt::ColumnModifiers};

#[derive(Debug)]
pub struct AlterStatement {
    pub object: AlterObject,
    pub object_name: String,
    pub mode: AlterMode,
    pub action: AlterAction,
}

#[derive(Debug)]
pub enum AlterObject {
    Column,
    Table,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlterMode {
    Add,
    Drop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlterAction {
    AddColumn(String, ColumnType, ColumnModifiers),
    DropColumn(String),
    /// Mode is infered from AlterStatement.mode
    Modifier(AlterActionModifiers),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlterActionModifiers {
    Unique,
    Nullable,
}
