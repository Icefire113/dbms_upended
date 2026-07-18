use std::collections::HashMap;

use bitcode::{Decode, Encode};

use crate::dbms::format::{
    Migrate,
    versions::{
        v1::TableFormatV1,
        v3::{DBFormatV3, IndexFormatV3, TableFormatV3},
    },
};

#[derive(Debug, Encode, Decode)]
pub(crate) struct DBFormatV2 {
    pub name: String,
    /// A mapping of column names to type
    pub tables: HashMap<String, TableFormatV1>,
    /// A mapping of index names to index formats
    pub indexes: HashMap<String, IndexFormatV1>,
}

#[derive(Debug, Encode, Decode)]
pub(crate) struct IndexFormatV1 {
    pub target_table: String,
    pub target_column: String,
}

impl Migrate for DBFormatV2 {
    type Next = DBFormatV3;

    fn migrate(self) -> Self::Next {
        Self::Next {
            tables: self
                .tables
                .into_iter()
                .map(|(tbl_name, tbl_fmt_v1)| {
                    (
                        tbl_name.clone(),
                        TableFormatV3 {
                            cols: tbl_fmt_v1.cols,
                            indexes: self
                                .indexes
                                .iter()
                                .filter_map(|(index_name, idx_fmt_v1)| {
                                    if idx_fmt_v1.target_table == tbl_name {
                                        Some((
                                            index_name.clone(),
                                            IndexFormatV3 {
                                                index_name: index_name.clone(),
                                                target_column: idx_fmt_v1.target_column.clone(),
                                            },
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }
}
