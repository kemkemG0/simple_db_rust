use std::sync::{Arc, Mutex};

use super::lock_table::LockTable;

pub struct ConcurrencyManager {
    lock_table: Arc<Mutex<LockTable>>,
    tx_num: usize,
}

impl ConcurrencyManager {
    fn new() -> Self {
        Self {
            lock_table: Arc::new(Mutex::new(LockTable::new())),
            tx_num: 0,
        }
    }
}
