use std::{
    collections::HashMap,
    thread::{current, park_timeout},
    time::SystemTimeError,
};

use crate::file::block_id::BlockId;

pub enum LockAbortError {
    SystemTimeError(SystemTimeError),
    General,
}

impl From<SystemTimeError> for LockAbortError {
    fn from(err: SystemTimeError) -> Self {
        LockAbortError::SystemTimeError(err)
    }
}

pub struct LockTable {
    max_time: u128,
    locks: HashMap<BlockId, i32>,
}

impl LockTable {
    const MAX_TIME: u128 = 10 * 1000;

    pub fn new() -> LockTable {
        LockTable {
            max_time: Self::MAX_TIME,
            locks: HashMap::new(),
        }
    }

    pub fn s_lock(&mut self, blk: &BlockId) -> Result<(), LockAbortError> {
        let time_stamp = now_mill_sec()?;
        while self.has_x_lock(blk) && !self.waiting_too_long(time_stamp)? {
            park_timeout(std::time::Duration::from_millis(self.max_time as u64));
        }
        if self.has_x_lock(blk) {
            return Err(LockAbortError::General);
        }
        self.locks.insert(blk.clone(), self.get_lock_value(blk) + 1);

        Ok(())
    }

    pub fn x_lock(&mut self, blk: &BlockId) -> Result<(), LockAbortError> {
        let time_stamp = now_mill_sec()?;
        while self.has_other_s_lock(blk) && !self.waiting_too_long(time_stamp)? {
            park_timeout(std::time::Duration::from_millis(self.max_time as u64));
        }
        if self.has_other_s_lock(blk) {
            return Err(LockAbortError::General);
        }
        self.locks.insert(blk.clone(), -1);
        Ok(())
    }

    pub fn unlock(&mut self, blk: &BlockId) -> Result<(), SystemTimeError> {
        let val = self.get_lock_value(blk);
        if val > 1 {
            self.locks.insert(blk.clone(), val - 1);
        } else {
            self.locks.remove(blk);
            current().unpark();
        }
        Ok(())
    }

    pub fn has_x_lock(&self, blk: &BlockId) -> bool {
        self.get_lock_value(blk) < 0
    }

    pub fn has_other_s_lock(&self, blk: &BlockId) -> bool {
        self.get_lock_value(blk) > 1
    }

    pub fn waiting_too_long(&self, time_stamp: u128) -> Result<bool, SystemTimeError> {
        Ok(now_mill_sec()? - time_stamp > self.max_time)
    }

    pub fn get_lock_value(&self, blk: &BlockId) -> i32 {
        match self.locks.get(blk) {
            Some(v) => *v,
            None => 0,
        }
    }
}

fn now_mill_sec() -> Result<u128, SystemTimeError> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis())
}
