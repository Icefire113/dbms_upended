use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Write},
    path::Path,
};

use tracing::{debug, trace};
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    dbms::{
        error::{DBFormatLoadError, DBFormatSaveError},
        format::versions::{v1::DBFormatV1, v2::DBFormatV2, v3::DBFormatV3},
    },
    util::{self, errors::UtilReadError},
};

mod versions;

pub use versions::v3::DBFormatV3 as DBFormat;

const DB_FORMAT_FILENAME: &'static str = "db_fmt";
const DB_FORMAT_MAGIC: [u8; 4] = *b"dbfm";
const DB_FORMAT_CURRENT_VERSION: u32 = 3;

/// Describes a database format that can be migrated to a newer format
pub(crate) trait Migrate: bitcode::Encode + for<'a> bitcode::Decode<'a> {
    type Next: bitcode::Encode + for<'a> bitcode::Decode<'a>;
    fn migrate(self) -> Self::Next;
}

impl DBFormat {
    /// Loads a database's format from a database folder
    pub fn load(path: impl AsRef<Path>) -> Result<Self, DBFormatLoadError> {
        let path = path.as_ref();
        debug!("Loading db {}", path.display());
        let file = File::open(path.join(DB_FORMAT_FILENAME))
            .map_err(|e| DBFormatLoadError::LoadingDbFormatFile(e))?;
        let mut reader = BufReader::new(file);

        if DB_FORMAT_MAGIC != util::read_n_bytes(&mut reader, 4)?.as_slice() {
            return Err(DBFormatLoadError::DbFormatInvalidHeaderMagic);
        }

        let ver: u32 = util::read_u32_le(&mut reader)?;
        trace!("format version: {}", ver);
        let data_hash: u64 = util::read_u64_be(&mut reader)?;
        trace!("hash in file: {:X}", data_hash);
        let data_size: u64 = util::read_u64_le(&mut reader)?;
        trace!("expected data size: {}", data_size);
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

        Ok(match ver {
            1 => bitcode::decode::<DBFormatV1>(&data)?.migrate().migrate(),
            2 => bitcode::decode::<DBFormatV2>(&data)?.migrate(),
            3 => bitcode::decode::<DBFormatV3>(&data)?,
            v => return Err(DBFormatLoadError::DbFormatFileUnknownVersion(v)),
        })
    }

    /// Saves a database's format to a database folder
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), DBFormatSaveError> {
        let path = path.as_ref().join(DB_FORMAT_FILENAME);
        let file = File::options()
            .write(true)
            .read(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);
        let data: Vec<u8> = bitcode::encode(self);
        let hash: u64 = xxh3_64(&data);
        writer.write(&DB_FORMAT_MAGIC)?;
        writer.write(&DB_FORMAT_CURRENT_VERSION.to_le_bytes())?;
        writer.write(&hash.to_be_bytes())?;
        writer.write(&data.len().to_le_bytes())?;
        writer.write(&data)?;

        Ok(())
    }
}
