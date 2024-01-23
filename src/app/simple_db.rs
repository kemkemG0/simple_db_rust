use crate::file::file_manager::FileManager;
pub struct SimpleDB {
    file_manager: FileManager,
}

impl SimpleDB {
    pub fn new(db_dir: String, block_size: u16, buffer_size: u16) -> Self {
        Self {
            file_manager: FileManager::new(db_dir, block_size),
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
            let db = SimpleDB::new(String::from("test_dir"), 400, 8);
            let fm = db.file_manager();

            let blk = BlockId::new(String::from("test_file"), 2);
            let mut p1 = Page::new(fm.block_size());

            let pos1 = 88;

            p1.set_string(pos1, String::from("abcdefghijklmn"));
            let size = Page::max_length(14);
            let pos2 = pos1 + size;

            p1.set_int(pos2, 345);
            fm.write(&blk, &mut p1);

            let mut p2 = Page::new(fm.block_size());
            fm.read(&blk, &mut p2);

            println!("offset {} contains {}", pos2, p2.get_int(pos2));
            println!("offset {} contains {}", pos1, p2.get_string(pos1));
        }
    }
}
