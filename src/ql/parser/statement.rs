use crate::ql::parser::statements::{
    alter_stmt::AlterStatement, create_stmt::CreateStatement, drop_stmt::DropStatement,
    insert_stmt::InsertStatement, load_stmt::LoadStatement, use_stmt::UseStatement,
};

#[derive(Debug)]
pub enum QLStatement {
    Use(UseStatement),
    Drop(DropStatement),
    Create(CreateStatement),
    Load(LoadStatement),
    Insert(InsertStatement),
    Alter(AlterStatement),
}
