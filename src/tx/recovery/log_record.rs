use crate::{
    file::page::Page,
    tx::transaction::{Transaction, TransactionError},
};

use super::{
    checkpoint_record::CheckPointRecord, commit_record::CommitRecord,
    rollback_record::RollbackRecord, setint_record::SetIntRecord,
    setstring_record::SetStringRecord, start_record::StartRecord,
};

pub enum Op {
    CheckPoint = 0,
    Start = 1,
    Commit = 2,
    Rollback = 3,
    SetInt = 4,
    SetString = 5,
}

impl TryFrom<u32> for Op {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Op::CheckPoint),
            1 => Ok(Op::Start),
            2 => Ok(Op::Commit),
            3 => Ok(Op::Rollback),
            4 => Ok(Op::SetInt),
            5 => Ok(Op::SetString),
            _ => Err(()),
        }
    }
}

pub trait LogRecord {
    fn op(&self) -> Op;
    fn tx_number(&self) -> Option<usize>;
    fn undo(&self, tx: &mut Transaction);
}

pub fn create_log_record(rec: Vec<u8>) -> Result<Box<dyn LogRecord>, TransactionError> {
    let page = Page::from_bytes(&rec);
    match page.get_int(0).try_into() {
        Ok(Op::CheckPoint) => Ok(Box::new(CheckPointRecord::new())),
        Ok(Op::Start) => Ok(Box::new(StartRecord::new(page))),
        Ok(Op::Commit) => Ok(Box::new(CommitRecord::new(page))),
        Ok(Op::Rollback) => Ok(Box::new(RollbackRecord::new(page))),
        Ok(Op::SetInt) => Ok(Box::new(SetIntRecord::new(page))),
        Ok(Op::SetString) => Ok(Box::new(SetStringRecord::new(page)?)),
        Err(_) => Err(TransactionError::General),
    }
}
