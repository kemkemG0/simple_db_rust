use crate::tx::transaction::{Transaction, TransactionError};

use super::log_record::{LogRecord, Op};

pub struct CheckPointRecord {}
impl CheckPointRecord {
    pub fn new() -> Self {
        todo!()
    }
}

impl LogRecord for CheckPointRecord {
    fn op(&self) -> Op {
        todo!()
    }

    fn tx_number(&self) -> Option<usize> {
        todo!()
    }

    fn undo(&self, tx: &mut Transaction) {
        todo!()
    }
}
