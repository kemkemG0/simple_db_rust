#[cfg(test)]
mod tests {

    mod buffer_test {
        use std::fs;

        use crate::{app::simple_db::SimpleDB, file::block_id::BlockId};

        #[test]
        fn test_buffer() {
            let db = SimpleDB::new("test_buffer", 400, 3).unwrap();
            let binding = db.buffer_manager();
            let mut bm = binding.lock().unwrap();

            let file_name = "test_buffer";

            let idx_1 = bm.pin(&BlockId::new(file_name, 1)).unwrap();
            let buffer_1 = bm.get_buffer(idx_1);
            let p1 = buffer_1.contents();
            let n = p1.get_int(80);
            p1.set_int(80, n + 1);
            assert_eq!(1, n + 1);
            assert!(buffer_1.modifing_tx_num().is_none());
            buffer_1.set_modified(1, 0);
            assert_eq!(buffer_1.modifing_tx_num().unwrap(), 1);
            bm.unpin(idx_1);

            // one of the following pin should flush the buffer_1
            let mut idx_2 = bm.pin(&BlockId::new(file_name, 2)).unwrap();
            bm.pin(&BlockId::new(file_name, 3)).unwrap();
            bm.pin(&BlockId::new(file_name, 4)).unwrap();

            bm.unpin(idx_2);
            idx_2 = bm.pin(&BlockId::new(file_name, 1)).unwrap();
            let buffer_2 = bm.get_buffer(idx_2);
            let p2 = buffer_2.contents();
            p2.set_int(80, 9999);
            buffer_2.set_modified(1, 0);
            bm.unpin(idx_2);

            fs::remove_dir_all("test_buffer").unwrap();
        }
    }

    mod buffer_manager_test {}
}
