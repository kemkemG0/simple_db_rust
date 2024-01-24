use crate::{
    file::{self, block_id::BlockId, file_manager::FileManager, page::Page},
    log::log_manager::LogManager,
};
use std::{
    io::Error,
    sync::{Arc, Mutex},
};

pub struct Buffer {
    file_manager: Arc<FileManager>,
    log_manager: Arc<Mutex<LogManager>>,
    contents: Page,
    block_id: Option<BlockId>,
    pins: u32,
    tx_num: Option<u32>,
    lsn: Option<u32>,
}

impl Buffer {
    pub fn new(file_manager: Arc<FileManager>, log_manager: Arc<Mutex<LogManager>>) -> Self {
        let fm = file_manager.clone();
        let lm = log_manager.clone();
        let contents = Page::new(fm.block_size());
        Self {
            file_manager: fm,
            log_manager: lm,
            contents,
            block_id: None,
            pins: 0,
            tx_num: None,
            lsn: None,
        }
    }
    pub fn contents(&mut self) -> &mut Page {
        &mut self.contents
    }
    pub fn block(&self) -> Option<&BlockId> {
        self.block_id.as_ref()
    }

    pub fn is_pinned(&self) -> bool {
        self.pins > 0
    }

    pub fn set_modified(&mut self, txnum: u32, lsn: u32) {
        self.tx_num = Some(txnum);
        if lsn > 0 {
            self.lsn = Some(lsn);
        }
    }

    pub fn modifing_tx_num(&self) -> Option<u32> {
        self.tx_num
    }

    pub fn pin(&mut self) {
        self.pins += 1
    }

    pub fn unpin(&mut self) {
        self.pins -= 1
    }

    pub fn assign_to_block(&mut self, block_id: BlockId) -> Result<(), Error> {
        self.flush()?;
        self.block_id = Some(block_id);
        self.file_manager
            .read(self.block_id.as_ref().unwrap(), &mut self.contents)?;
        self.pins = 0;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        if self.tx_num.is_some() {
            if let Some(lsn) = self.lsn {
                self.log_manager.lock().unwrap().flush(lsn)?;
            }
            if let Some(blk) = &self.block_id {
                self.file_manager.write(blk, &mut self.contents)?;
            }
            self.tx_num = None;
        }
        Ok(())
    }
}
