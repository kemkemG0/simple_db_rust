use std::{mem::size_of, sync::Mutex};

use crate::file::{self, block_id::BlockId, file_manager::FileManager, page::Page};

use super::log_iterator::LogIterator;

pub struct LogManager {
    file_manager: FileManager,
    log_file: String,
    log_page: Page,
    current_block: BlockId,
    // lsn: log sequence number
    latest_lsn: u32,
    latest_saved_lsn: u32,
    mutex: Mutex<()>,
}

impl LogManager {
    pub fn new(mut file_manager: FileManager, log_file: String) -> Self {
        let mut log_page = Page::new(file_manager.block_size());
        let log_size = file_manager.length(log_file.clone());
        let current_block = {
            if log_size == 0 {
                _append_new_block(&mut file_manager, &log_file, &mut log_page)
            } else {
                let current_block = BlockId::new(log_file.clone(), log_size - 1);
                file_manager.read(&current_block, &mut log_page).unwrap();
                current_block
            }
        };
        Self {
            file_manager,
            log_file,
            log_page,
            current_block,
            latest_lsn: 0,
            latest_saved_lsn: 0,
            mutex: Mutex::new(()),
        }
    }
    pub fn flush(&mut self, lsn: u32) {
        if lsn >= self.latest_saved_lsn {
            self._flush()
        }
    }
    pub fn iterator(&mut self) -> LogIterator {
        self._flush();
        LogIterator::new(&mut self.file_manager, &mut self.current_block)
    }

    pub fn append(&mut self, log_record: &Vec<u8>) -> u32 {
        let mut boundary = self.log_page.get_int(0);
        let bytes_needed = (log_record.len() + size_of::<u32>()) as u32;

        if (boundary - bytes_needed) < size_of::<u32>() as u32 {
            // it doesn't fit in the current block so move to the next one
            self._flush();
            self.current_block = self.append_new_block();
            boundary = self.log_page.get_int(0);
        }

        let rec_offset = boundary - bytes_needed;
        self.log_page.set_bytes(rec_offset as usize, log_record);
        self.log_page.set_int(0, rec_offset);

        self.latest_lsn += 1;
        self.latest_lsn
    }

    pub fn getLastSavedLSN(&self) -> u32 {
        self.latest_saved_lsn
    }

    fn append_new_block(&mut self) -> BlockId {
        let _guard = self.mutex.lock().unwrap();
        _append_new_block(
            &mut self.file_manager,
            &self.log_file.clone(),
            &mut self.log_page,
        )
    }

    fn _flush(&mut self) {
        let _guard = self.mutex.lock().unwrap();
        self.file_manager
            .write(&self.current_block, &mut self.log_page)
            .unwrap();
        self.latest_saved_lsn = self.latest_lsn;
    }
}

fn _append_new_block(
    file_manager: &mut FileManager,
    log_file: &str,
    log_page: &mut Page,
) -> BlockId {
    let block_id = file_manager.append(String::from(log_file)).unwrap();
    log_page.set_int(0, file_manager.block_size() as u32);
    file_manager.write(&block_id, log_page);
    block_id
}
