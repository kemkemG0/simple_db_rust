use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    file::file_manager::FileManager,
    log::log_manager::{self, LogManager},
};
pub struct SimpleDB {
    file_manager: Arc<FileManager>,
    log_manager: Arc<Mutex<LogManager>>,
}

impl SimpleDB {
    pub fn new(db_dir: String, block_size: usize, buffer_size: u16) -> Self {
        let file_manager = Arc::new(FileManager::new(PathBuf::from(db_dir), block_size));
        let log_manager = Arc::new(Mutex::new(LogManager::new(
            file_manager.clone(),
            String::from("simpledb.log"),
        )));
        Self {
            file_manager,
            log_manager,
        }
    }
    pub fn file_manager(&self) -> Arc<FileManager> {
        self.file_manager.clone()
    }
    pub fn log_manager(&self) -> Arc<Mutex<LogManager>> {
        self.log_manager.clone()
    }
}
