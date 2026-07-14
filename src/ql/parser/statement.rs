use crate::ql::parser::statements::{
    alter_stmt::AlterStatement, create_stmt::CreateStatement, delete_stmt::DeleteStatement,
    drop_stmt::DropStatement, insert_stmt::InsertStatement, load_stmt::LoadStatement,
    select_stmt::SelectStatement, update_stmt::UpdateStatement, use_stmt::UseStatement,
};

#[derive(Debug)]
pub enum QLStatement {
    Use(UseStatement),
    Drop(DropStatement),
    Create(CreateStatement),
    Load(LoadStatement),
    Insert(InsertStatement),
    Alter(AlterStatement),
    Delete(DeleteStatement),
    Update(UpdateStatement),
    Select(SelectStatement),
}
