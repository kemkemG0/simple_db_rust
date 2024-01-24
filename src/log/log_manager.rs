use std::{io::Error, mem::size_of, sync::Arc};

use crate::file::{block_id::BlockId, file_manager::FileManager, page::Page};

use super::log_iterator::LogIterator;

pub struct LogManager {
    file_manager: Arc<FileManager>,
    log_file: String,
    log_page: Page,
    current_block: BlockId,
    // lsn: log sequence number
    latest_lsn: u32,
    latest_saved_lsn: u32,
}

impl LogManager {
    pub fn new(file_manager: Arc<FileManager>, log_file: &str) -> Result<Self, Error> {
        let mut log_page = Page::new(file_manager.block_size());
        let log_size = file_manager.length(log_file)?;
        let current_block = {
            if log_size == 0 {
                match _append_new_block(&file_manager, log_file, &mut log_page) {
                    Ok(block_id) => block_id,
                    Err(e) => return Err(e),
                }
            } else {
                let current_block = BlockId::new(log_file, log_size - 1);
                file_manager.read(&current_block, &mut log_page)?;
                current_block
            }
        };
        Ok(Self {
            file_manager,
            log_file: String::from(log_file),
            log_page,
            current_block,
            latest_lsn: 0,
            latest_saved_lsn: 0,
        })
    }

    /**
     * flushes the log to disk, ensuring lsn records are persisted to disk.
     * @param lsn the log sequence number that must be written to disk
     */
    pub fn flush(&mut self, lsn: u32) -> Result<(), Error> {
        if lsn >= self.latest_saved_lsn {
            self._flush()?;
        }
        Ok(())
    }

    /**
     * returns an iterator for the log records,
     * which will be returned in reverse order starting with the most recent.
     */
    pub fn iterator(&mut self) -> Result<LogIterator, Error> {
        self._flush()?;
        LogIterator::new(self.file_manager.clone(), &self.current_block)
    }

    /**
     * add a record to the log and returns its log sequence number.
     * appeding a record to log does not guarantee that it is immediately written to disk.
     * @param log_record the log record to be added
     */
    pub fn append(&mut self, log_record: &Vec<u8>) -> Result<u32, Error> {
        // boundary: offset of the most recently added log record
        let mut boundary = self.log_page.get_int(0) as i32;
        let bytes_needed = (log_record.len() + size_of::<u32>()) as i32;

        if (boundary - bytes_needed) < (size_of::<u32>() as i32) {
            // it doesn't fit in the current block so move to the next one
            self._flush()?;
            self.current_block = self.append_new_block()?;
            boundary = self.log_page.get_int(0) as i32;
        }

        let rec_offset = boundary - bytes_needed;
        self.log_page.set_bytes(rec_offset as usize, log_record);
        self.log_page.set_int(0, rec_offset as u32);

        self.latest_lsn += 1;
        Ok(self.latest_lsn)
    }

    pub fn get_last_saved_lsn(&self) -> u32 {
        self.latest_saved_lsn
    }

    fn append_new_block(&mut self) -> Result<BlockId, Error> {
        _append_new_block(
            &self.file_manager,
            &self.log_file.clone(),
            &mut self.log_page,
        )
    }

    fn _flush(&mut self) -> Result<(), Error> {
        self.file_manager
            .write(&self.current_block, &mut self.log_page)?;
        self.latest_saved_lsn = self.latest_lsn;
        Ok(())
    }
}

fn _append_new_block(
    file_manager: &FileManager,
    log_file: &str,
    log_page: &mut Page,
) -> Result<BlockId, Error> {
    let block_id = file_manager.append(log_file)?;
    // Use the first four bytes as boundary, which is the offset of the most recently added log record
    // if the block_size is 400, then the boundary is 400 at the beginning
    log_page.set_int(0, file_manager.block_size() as u32);
    file_manager.write(&block_id, log_page)?;
    Ok(block_id)
}
