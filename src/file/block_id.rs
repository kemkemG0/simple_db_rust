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
    pub fn new(filename: &str, blk_num: u64) -> BlockId {
        BlockId {
            filename: String::from(filename),
            blk_num,
        }
    }

    pub fn filename(&self) -> &str {
        self.filename.as_str()
    }

    pub fn number(&self) -> u64 {
        self.blk_num
    }

    pub fn hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        format!("{}", self).hash(&mut hasher);
        hasher.finish()
    }
    pub fn is_null(&self) -> bool {
        self.filename().is_empty()
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[file_name]: {}, [block]: {}",
            self.filename, self.blk_num
        )
    }
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.filename == other.filename && self.blk_num == other.blk_num
    }
}

// implement clone for BlockId
impl Clone for BlockId {
    fn clone(&self) -> Self {
        BlockId {
            filename: self.filename.clone(),
            blk_num: self.blk_num,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_id() {
        let block_id = BlockId::new("test", 1);
        assert_eq!(block_id.filename(), "test");
        assert_eq!(block_id.number(), 1);
        assert_eq!(block_id.to_string(), "[file_name]: test, [block]: 1");

        let block_id2 = BlockId::new("test", 1);
        assert!(block_id == block_id2);

        let block_id3 = BlockId::new("test", 2);
        assert!(block_id != block_id3);

        let block_id4 = BlockId::new("test2", 1);
        assert!(block_id != block_id4);

        let block_id5 = BlockId::new("test2", 2);
        assert!(block_id != block_id5);
    }
}
