#[derive(Debug)]
pub struct DropStatement {
    pub drop_type: DropType,
    pub target: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DropType {
    Table,
    Database,
    Index,
}
