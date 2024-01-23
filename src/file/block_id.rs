use std::{
    cmp::PartialEq,
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
};

pub struct BlockId {
    filename: String,
    blk_num: u64,
}

impl BlockId {
    pub fn new(filename: String, blk_num: u64) -> BlockId {
        BlockId { filename, blk_num }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn number(&self) -> u64 {
        self.blk_num
    }

    pub fn hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        format!("{}", self).hash(&mut hasher);
        hasher.finish()
    }
    pub fn isNull(&self) -> bool {
        self.filename().is_empty()
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file_name: {}, block: {}", self.filename, self.blk_num)
    }
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.filename == other.filename && self.blk_num == other.blk_num
    }
}
