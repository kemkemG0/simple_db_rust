use crate::{
    file::page::Page,
    tx::transaction::{Transaction, TransactionError},
};

use super::log_record::{LogRecord, Op};

pub struct SetIntRecord {}
impl SetIntRecord {
    pub fn new(page: Page) -> Self {
        todo!()
    }
}

impl LogRecord for SetIntRecord {
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
