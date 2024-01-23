use std::path::PathBuf;

use crate::file::file_manager::FileManager;
pub struct SimpleDB {
    file_manager: FileManager,
}

impl SimpleDB {
    pub fn new(db_dir: String, block_size: usize, buffer_size: u16) -> Self {
        Self {
            file_manager: FileManager::new(PathBuf::from(db_dir), block_size),
        }
    }
    pub fn file_manager(&self) -> &FileManager {
        &self.file_manager
    }
}

#[cfg(test)]
mod tests {
    mod file_test {
        use crate::{
            app::simple_db::SimpleDB,
            file::{block_id::BlockId, page::Page},
        };
        #[test]
        fn test_simple_db() {
            let db = SimpleDB::new(String::from("fileTest"), 400, 8);
            let fm = db.file_manager();
            assert_eq!(fm.block_size(), 400);
            let block_id = BlockId::new(String::from("test_file"), 2);
            let mut p1 = Page::new(fm.block_size());
            let pos1 = 88;
            let content = String::from("abcdefghijklm");
            p1.set_string(pos1, content.clone());
            assert_eq!(p1.get_string(pos1), content);
            let size = Page::max_length(content.len());
            let pos2 = pos1 + size;

            p1.set_int(pos2, 345);

            fm.write(&block_id, &mut p1).unwrap();

            let mut p2 = Page::new(fm.block_size());
            fm.read(&block_id, &mut p2).unwrap();

            assert_eq!(p2.get_string(pos1), content);
            assert_eq!(p2.get_int(pos2), 345);
        }
    }
}
