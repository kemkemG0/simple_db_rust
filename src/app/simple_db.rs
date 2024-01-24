use std::{
    io::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{file::file_manager::FileManager, log::log_manager::LogManager};
pub struct SimpleDB {
    file_manager: Arc<FileManager>,
    log_manager: Arc<Mutex<LogManager>>,
}

impl SimpleDB {
    pub fn new(db_dir: &str, block_size: usize, buffer_size: u16) -> Result<Self, Error> {
        let file_manager = Arc::new(FileManager::new(PathBuf::from(db_dir), block_size)?);
        let log_manager = Arc::new(Mutex::new(LogManager::new(
            file_manager.clone(),
            "simpledb.log",
        )?));
        Ok(Self {
            file_manager,
            log_manager,
        })
    }
    pub fn file_manager(&self) -> Arc<FileManager> {
        self.file_manager.clone()
    }
    pub fn log_manager(&self) -> Arc<Mutex<LogManager>> {
        self.log_manager.clone()
    }
}
