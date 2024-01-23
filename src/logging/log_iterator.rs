use crate::file::block_id::BlockId;
use crate::file::file_manager::FileManager;
pub struct LogIterator {}

impl LogIterator {
    pub fn new(file_manager: &mut FileManager, current_block: &mut BlockId) -> Self {
        Self {}
    }
}
