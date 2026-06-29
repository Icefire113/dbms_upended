use std::collections::HashMap;

use bitcode::{Decode, Encode};

use crate::row::format::ColType;

#[derive(Debug, Encode, Decode)]
pub struct TableFormat {
    name: String,
    /// A mapping of column names to type
    cols: HashMap<String, ColType>,
}

impl TableFormat {
    pub fn tbl_name(&self) -> &str {
        &self.name
    }
}
