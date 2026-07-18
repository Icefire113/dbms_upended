use std::{collections::HashMap, fs, path::PathBuf};

use tracing::info;

use crate::dbms::{error::DBMSError, format::DBFormat};

pub mod error;
pub mod format;

#[derive(Debug)]
pub struct DBMS {
    root: PathBuf,
    db_schemas: HashMap<String, DBFormat>,
}

impl DBMS {
    pub fn new(root: impl Into<PathBuf>, create: bool) -> Result<Self, DBMSError> {
        let root_path = root.into();
        let root = match root_path.canonicalize() {
            Ok(root) => root,
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        // not found, create if user told us to
                        if create {
                            info!("Creating dbms root");
                            fs::create_dir(&root_path)?;
                            root_path.canonicalize()?
                        } else {
                            return Err(DBMSError::DbmsRootDoesNotExist);
                        }
                    }
                    _ => return Err(DBMSError::Io(e)),
                }
            }
        };

        Ok(Self {
            root,
            db_schemas: HashMap::new(),
        })
    }

    /// Loads the database(s)'s metadata/ schema from disk, note that this
    /// will migrate older schemas to the current format version
    pub fn load_schemas(&mut self) -> Result<(), DBMSError> {
        for dir in fs::read_dir(&self.root)? {
            let dir = dir?;

            if dir.metadata()?.is_dir() {
                let db_name = dir
                    .file_name()
                    .to_str()
                    .ok_or(DBMSError::DbNameIsNotUtf8(dir.file_name()))?
                    .to_owned();
                let db_format = DBFormat::load(self.root.join(&db_name))?;
                self.db_schemas.insert(db_name, db_format);
            }
        }
        Ok(())
    }

    /// Saves the database(s)'s metadata/ schema to disk, note that this will
    /// save in a format that matches what is currently loaded in memory, this should always
    /// be the latest format
    pub fn save_schemas(&self) -> Result<(), DBMSError> {
        for (db_name, db_format) in &self.db_schemas {
            let path = self.root.join(&db_name);
            db_format.save(path)?;
        }
        Ok(())
    }
}
