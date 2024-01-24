use std::io::Error;
use std::mem::size_of;
use std::sync::Arc;

use crate::file::block_id::BlockId;
use crate::file::file_manager::FileManager;
use crate::file::page::Page;
pub struct LogIterator {
    file_manager: Arc<FileManager>,
    block_id: BlockId,
    page: Page,
    current_pos: u32,
    boundary: u32,
}

impl LogIterator {
    pub fn new(file_manager: Arc<FileManager>, current_block: &BlockId) -> Self {
        let page = Page::new(file_manager.block_size());
        let mut l = Self {
            file_manager,
            block_id: current_block.clone(),
            page,
            current_pos: 0,
            boundary: 0,
        };
        l.move_to_block(current_block);
        l
    }
    pub fn has_next(&self) -> bool {
        (self.current_pos < self.file_manager.block_size() as u32) || self.block_id.number() > 0
    }
    fn move_to_block(&mut self, block_id: &BlockId) -> Result<(), Error> {
        self.file_manager.read(block_id, &mut self.page)?;
        self.boundary = self.page.get_int(0);
        self.current_pos = self.boundary;
        Ok(())
    }
}

impl Iterator for LogIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        if self.current_pos == self.file_manager.block_size() as u32 {
            self.block_id = BlockId::new(self.block_id.filename(), self.block_id.number() - 1);
            self.move_to_block(&self.block_id.clone());
        }
        let rec = self.page.get_bytes(self.current_pos as usize);
        self.current_pos += rec.len() as u32 + size_of::<u32>() as u32;
        Some(rec.to_vec())
    }
}
