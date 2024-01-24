use std::{
    io::Error,
    sync::{Arc, Mutex},
    thread::{current, park_timeout},
    time::SystemTimeError,
};

use crate::{
    file::{block_id::BlockId, file_manager::FileManager},
    log::log_manager::LogManager,
};

use super::buffer::Buffer;

#[derive(Debug)]
pub enum BufferAbortError {
    Time(SystemTimeError),
    IO(Error),
    General,
}

impl From<SystemTimeError> for BufferAbortError {
    fn from(err: SystemTimeError) -> Self {
        BufferAbortError::Time(err)
    }
}

impl From<Error> for BufferAbortError {
    fn from(err: Error) -> Self {
        BufferAbortError::IO(err)
    }
}

pub struct BufferManager {
    buffer_pool: Vec<Buffer>,
    num_available: u16,
}

impl BufferManager {
    const MAX_TIME: u128 = 10 * 1000; // 10 seconds
    pub fn new(
        file_manager: Arc<FileManager>,
        log_manager: Arc<Mutex<LogManager>>,
        num_buffers: u16,
    ) -> Self {
        let mut buffer_pool = Vec::with_capacity(num_buffers as usize);
        for _ in 0..num_buffers {
            buffer_pool.push(Buffer::new(file_manager.clone(), log_manager.clone()));
        }
        Self {
            buffer_pool,
            num_available: num_buffers,
        }
    }

    pub fn available(&self) -> u16 {
        self.num_available
    }

    pub fn flush_all(&mut self, txnum: u32) {
        for buffer in self.buffer_pool.iter_mut() {
            if let Some(t) = buffer.modifing_tx_num() {
                if t == txnum {
                    buffer.flush();
                }
            }
        }
    }

    pub fn unpin(&mut self, idx: usize) {
        self.buffer_pool[idx].unpin();
        if !self.buffer_pool[idx].is_pinned() {
            self.num_available += 1;
            // notify other threads that there is a unpinned buffer and it is available
            current().unpark();
        }
    }

    pub fn pin(&mut self, blk: &BlockId) -> Result<usize, BufferAbortError> {
        let time_stamp = now_mill_sec()?;
        let mut idx = self.try_to_pin(blk)?;
        while idx.is_none() && !self.waiting_too_long(time_stamp)? {
            // wait for a unpinned buffer
            park_timeout(std::time::Duration::from_millis(Self::MAX_TIME as u64));
            idx = self.try_to_pin(blk)?;
        }
        match idx {
            Some(i) => Ok(i),
            None => Err(BufferAbortError::General),
        }
    }

    pub fn try_to_pin(&mut self, blk: &BlockId) -> Result<Option<usize>, Error> {
        let mut idx = self.find_existing_buffer(blk);
        if idx.is_none() {
            idx = self.choose_unpinned_buffer();
            if let Some(i) = idx {
                self.buffer_pool[i].assign_to_block(blk.clone())?;
            } else {
                return Ok(None);
            }
        }
        if let Some(i) = idx {
            if !self.buffer_pool[i].is_pinned() {
                self.num_available -= 1;
            }
            self.buffer_pool[i].pin();
        }
        Ok(idx)
    }

    pub fn find_existing_buffer(&mut self, blk: &BlockId) -> Option<usize> {
        for (idx, buffer) in self.buffer_pool.iter().enumerate() {
            if let Some(b) = buffer.block() {
                if b == blk {
                    return Some(idx);
                }
            }
        }
        None
    }

    pub fn choose_unpinned_buffer(&mut self) -> Option<usize> {
        for (idx, buffer) in self.buffer_pool.iter().enumerate() {
            if !buffer.is_pinned() {
                return Some(idx);
            }
        }
        None
    }

    pub fn waiting_too_long(&self, start_time: u128) -> Result<bool, SystemTimeError> {
        Ok(now_mill_sec()? - start_time > Self::MAX_TIME)
    }

    pub fn get_buffer(&mut self, idx: usize) -> &mut Buffer {
        &mut self.buffer_pool[idx]
    }
}

fn now_mill_sec() -> Result<u128, SystemTimeError> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis())
}
