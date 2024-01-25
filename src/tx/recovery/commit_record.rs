use crate::{file::page::Page, tx::transaction::Transaction};

use super::log_record::{LogRecord, Op};

pub struct CommitRecord {}
impl CommitRecord {
    pub fn new(page: Page) -> Self {
        todo!()
    }
}

impl LogRecord for CommitRecord {
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
