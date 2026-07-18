use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, ErrorKind},
    path::Path,
};

use bitcode::{Decode, Encode};
use tracing::{debug, trace};
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    dbms::error::DBFormatLoadError,
    util::{self, errors::UtilReadError},
};

const DB_FORMAT_FILENAME: &'static str = "db_fmt";
const DB_FORMAT_MAGIC: [u8; 4] = *b"dbfm";
const DB_FORMAT_CURRENT_VERSION: u32 = 1;

#[derive(Debug, Encode, Decode)]
pub struct DBFormat {
    name: String,
    tables: HashMap<String, TableFormat>,
}

impl DBFormat {
    /// Loads a database's format from a database folder
    pub fn load(path: impl AsRef<Path>, db_name: &str) -> Result<Self, DBFormatLoadError> {
        let path = path.as_ref();
        debug!("Loading db {}", path.display());
        let file = File::open(path.join(DB_FORMAT_FILENAME))
            .map_err(|e| DBFormatLoadError::LoadingDbFormatFile(e))?;
        let mut reader = BufReader::new(file);

        if DB_FORMAT_MAGIC != util::read_n_bytes(&mut reader, 4)?.as_slice() {
            return Err(DBFormatLoadError::DbFormatInvalidHeaderMagic);
        }

        let ver: u32 = util::read_u32_le(&mut reader)?;
        if DB_FORMAT_CURRENT_VERSION != ver {
            return Err(DBFormatLoadError::DbFormatFileUnknownVersion(ver));
        }

        let data_hash: u64 = util::read_u64_be(&mut reader)?;
        let data_size: u64 = util::read_u64_le(&mut reader)?;
        trace!("expected data size: {}", data_size);
        trace!("hash in file: {:X}", data_hash);
        let data: Vec<u8> = util::read_n_bytes(
            &mut reader,
            data_size
                .try_into()
                .expect("This dbms is not fully supported on x86 hardware, please use x86_64"),
        )
        .map_err(|e| match &e {
            UtilReadError::IoError(error) => match error.kind() {
                ErrorKind::UnexpectedEof => DBFormatLoadError::DbFormatFileCorrupted,
                _ => DBFormatLoadError::UtilReadError(e),
            },
            UtilReadError::Utf8ParseError(_) => DBFormatLoadError::UtilReadError(e),
        })?;

        let hash = xxh3_64(&data);
        if hash != data_hash {
            trace!("expected hash: {:X}, got hash: {:X}", data_hash, hash);
            return Err(DBFormatLoadError::DbFormatFileInvalidChecksum);
        }
        trace!("Hash OK");

        let db_format: DBFormat = bitcode::decode(&data)?;
        if db_format.name != db_name {
            trace!(
                "listed db name: {}, format db name: {}",
                db_name, db_format.name
            );
            return Err(DBFormatLoadError::DbFormatMismatchedName);
        }

        for (tbl_name, tbl_format) in &db_format.tables {
            if tbl_name != tbl_format.tbl_name() {
                trace!(
                    "listed table name: {}, format table name: {}",
                    tbl_name,
                    tbl_format.tbl_name()
                );
                return Err(DBFormatLoadError::DbFormatMismatchedTableName);
            }
        }

        Ok(db_format)
    }
}

#[derive(Debug, Encode, Decode)]
pub struct TableFormat {
    name: String,
    /// A mapping of column names to type
    cols: HashMap<String, ColumnType>,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub enum ColumnType {
    Int,
    BigInt,
    Float,
    BigFloat,
    Bool,
    String,
}

impl TableFormat {
    pub fn tbl_name(&self) -> &str {
        &self.name
    }
}
