use std::collections::HashMap;

use bitcode::{Decode, Encode};

use crate::dbms::format::{Migrate, versions::v2::DBFormatV2};

#[derive(Debug, Encode, Decode)]
pub(crate) struct DBFormatV1 {
    pub name: String,
    pub tables: HashMap<String, TableFormatV1>,
}

#[derive(Debug, Encode, Decode)]
pub(crate) struct TableFormatV1 {
    name: String,
    /// A mapping of column names to type
    pub cols: HashMap<String, ColumnTypeV1>,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub(crate) enum ColumnTypeV1 {
    Int,
    BigInt,
    Float,
    BigFloat,
    Bool,
    String,
}

impl Migrate for DBFormatV1 {
    type Next = DBFormatV2;

    fn migrate(self) -> Self::Next {
        Self::Next {
            name: self.name,
            tables: self.tables,
            indexes: HashMap::new(),
        }
    }
}
