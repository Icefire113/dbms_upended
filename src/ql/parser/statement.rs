use crate::ql::parser::statements::{
    create_stmt::CreateStatement, drop_stmt::DropStatement, load_stmt::LoadStatement,
    use_stmt::UseStatement,
};

#[derive(Debug)]
pub enum QLStatement {
    Use(UseStatement),
    Drop(DropStatement),
    Create(CreateStatement),
    Load(LoadStatement),
}
