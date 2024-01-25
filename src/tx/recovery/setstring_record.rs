use std::fmt;
use std::io::Error;
use std::option::Option;
use std::sync::{Arc, Mutex};
use std::{mem::size_of, string::FromUtf8Error};

use crate::log;
use crate::log::log_manager::LogManager;
use crate::{
    file::{block_id::BlockId, page::Page},
    tx::transaction::{Transaction, TransactionError},
};

use super::log_record::{LogRecord, Op};

pub struct SetStringRecord {
    tx_num: usize,
    offset: usize,
    val: String,
    block_id: BlockId,
}
impl SetStringRecord {
    pub fn new(page: Page) -> Result<Self, FromUtf8Error> {
        let tx_pos = size_of::<u32>();
        let tx_num = page.get_int(tx_pos) as usize;

        let f_pos = tx_pos + size_of::<u32>();
        let file_name = page.get_string(f_pos)?;

        let b_pos = f_pos + Page::max_length(file_name.len());
        let blk_num = page.get_int(b_pos);
        let block_id = BlockId::new(&file_name, blk_num.into());

        let o_pos = b_pos + size_of::<u32>();
        let offset = page.get_int(o_pos) as usize;

        let v_pos = o_pos + size_of::<u32>();
        let val = page.get_string(v_pos)?;

        Ok(Self {
            tx_num,
            offset,
            val,
            block_id,
        })
    }
    fn write_to_log(
        log_manager: Arc<Mutex<LogManager>>,
        tx_num: usize,
        block_id: &BlockId,
        offset: usize,
        val: &str,
    ) -> Result<u32, Error> {
        let tx_pos = size_of::<u32>();
        let f_pos = tx_pos + size_of::<u32>();
        let b_pos = f_pos + Page::max_length(block_id.filename().len());
        let o_pos = f_pos + size_of::<u32>();
        let v_pos = o_pos + size_of::<u32>();

        let rec_len = v_pos + Page::max_length(val.len());
        let rec = vec![0_u8; rec_len];

        let mut page = Page::from_bytes(&rec);

        page.set_int(0, Op::SetString as u32);
        page.set_int(tx_pos, tx_num as u32);
        page.set_string(f_pos, block_id.filename());
        page.set_int(b_pos, block_id.number() as u32);
        page.set_int(o_pos, offset as u32);
        page.set_string(v_pos, val);

        log_manager
            .lock()
            .unwrap()
            .append(&page.contents().to_vec())
    }
}

impl LogRecord for SetStringRecord {
    fn op(&self) -> Op {
        Op::SetString
    }

    fn tx_number(&self) -> Option<usize> {
        Some(self.tx_num)
    }

    fn undo(&self, tx: &mut Transaction) {
        tx.pin(&self.block_id);
        tx.set_string(&self.block_id, self.offset, &self.val, false); // false: do not log
        tx.unpin(&self.block_id);
    }
}

impl fmt::Display for SetStringRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<SETSTRING {} {} {} {}>",
            self.tx_num, self.block_id, self.offset, self.val
        )
    }
}
