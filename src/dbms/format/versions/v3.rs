use std::collections::HashMap;

use bitcode::{Decode, Encode};

use crate::dbms::format::versions::v1::ColumnTypeV1;

#[derive(Debug, Encode, Decode)]
pub struct DBFormatV3 {
    /// A mapping of table names to its format
    pub tables: HashMap<String, TableFormatV3>,
}

#[derive(Debug, Encode, Decode)]
pub(crate) struct TableFormatV3 {
    /// A mapping of column names to type
    pub cols: HashMap<String, ColumnTypeV1>,
    /// A mapping of index names to index formats
    pub indexes: HashMap<String, IndexFormatV3>,
}

#[derive(Debug, Encode, Decode)]
pub(crate) struct IndexFormatV3 {
    pub index_name: String,
    pub target_column: String,
}
