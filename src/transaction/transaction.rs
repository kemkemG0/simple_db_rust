use std::sync::{Arc, Mutex};

use crate::{
    buffer::buffer_manager::BufferManager,
    file::{block_id::BlockId, file_manager::FileManager},
    log::log_manager::LogManager,
};

pub struct Transaction {
    file_manager: Arc<FileManager>,
    log_manager: Arc<Mutex<LogManager>>,
    buffer_manager: Arc<Mutex<BufferManager>>,
}

impl Transaction {
    pub fn new(
        file_manager: Arc<FileManager>,
        log_manager: Arc<Mutex<LogManager>>,
        buffer_manager: Arc<Mutex<BufferManager>>,
    ) -> Self {
        Self {
            file_manager,
            log_manager,
            buffer_manager,
        }
    }

    pub fn commit(&mut self) {}
    pub fn rollback(&mut self) {}
    pub fn recover(&mut self) {}

    pub fn pin(&mut self, block_id: &BlockId) {}
    pub fn unpin(&mut self, block_id: &BlockId) {}

    pub fn get_int(&self, block_id: &BlockId, offset: usize) -> u32 {
        0
    }
    pub fn get_string(&self, block_id: &BlockId, offset: usize) -> &str {
        ""
    }
    pub fn set_int(&mut self, block_id: &BlockId, offset: usize, val: u32, ok_to_log: bool) {}

    pub fn set_string(&mut self, block_id: &BlockId, offset: usize, val: &str, ok_to_log: bool) {}

    pub fn available_buffers(&self) -> u16 {
        0
    }

    pub fn size(&self, file_name: &str) -> usize {
        0
    }
    pub fn append(&mut self, file_name: &str) -> BlockId {
        BlockId::new("", 0)
    }
    pub fn block_size(&self) -> usize {
        0
    }
}
