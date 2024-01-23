use std::mem::size_of;

use crate::file::block_id::BlockId;
use crate::file::file_manager::FileManager;
use crate::file::page::Page;
pub struct LogIterator<'a> {
    file_manager: &'a FileManager,
    block_id: BlockId,
    page: Page,
    current_pos: u32,
    boundary: u32,
}

impl<'a> LogIterator<'a> {
    pub fn new(file_manager: &'a FileManager, current_block: BlockId) -> Self {
        let mut page = Page::new(file_manager.block_size());
        file_manager.read(&current_block, &mut page).unwrap();
        let boundary = page.get_int(0);

        Self {
            file_manager,
            block_id: current_block,
            page,
            current_pos: boundary,
            boundary,
        }
    }
    pub fn has_next(&self) -> bool {
        (self.current_pos < self.file_manager.block_size() as u32) || self.block_id.number() > 0
    }
    fn move_to_block(&mut self) {
        self.file_manager
            .read(&self.block_id, &mut self.page)
            .unwrap();
        self.boundary = self.page.get_int(0);
        self.current_pos = self.boundary;
    }
}

impl<'a> Iterator for LogIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        if self.current_pos == self.file_manager.block_size() as u32 {
            self.block_id =
                BlockId::new(self.block_id.filename().clone(), self.block_id.number() - 1);
            self.move_to_block();
        }
        let rec = self.page.get_bytes(self.current_pos as usize);
        self.current_pos += rec.len() as u32 + size_of::<u32>() as u32;
        Some(rec)
    }
}
