#[cfg(test)]
mod tests {
    use std::{
        fs,
        iter::zip,
        mem::size_of,
        sync::{Arc, Mutex},
    };

    use crate::{app::simple_db::SimpleDB, file::page::Page, log::log_manager::LogManager};

    #[test]
    fn test_log() {
        let db = SimpleDB::new(String::from("./test_log/log"), 400, 8);
        let lm = db.log_manager();
        create_records(lm.clone(), 1, 35);
        assert_log_records(lm.clone(), (1..=35).rev().collect());

        create_records(lm.clone(), 36, 70);
        lm.lock().unwrap().flush(65);
        assert_log_records(lm.clone(), (1..=70).rev().collect());

        fs::remove_dir_all("test_log").unwrap();
    }

    fn assert_log_records(lm: Arc<Mutex<LogManager>>, expected: Vec<u32>) {
        let mut lm = lm.lock().unwrap();
        let itr = lm.iterator();

        for (rec, exp) in zip(itr, expected) {
            let page = Page::from_bytes(&rec);
            let s = page.get_string(0);
            let n = Page::max_length(s.len());
            let val = page.get_int(n);

            assert_eq!(s, format!("record: {}", exp));
            assert_eq!(val, exp + 100);
        }
    }

    fn create_records(lm: Arc<Mutex<LogManager>>, start: u32, end: u32) {
        println!("create_records");
        for i in start..=end {
            let rec_string = format!("record: {}", i);
            let rec = create_log_records(rec_string, i + 100);
            let lsn = lm.lock().unwrap().append(&rec);

            assert_eq!(lsn, i)
        }
    }

    fn create_log_records(rec_string: String, n: u32) -> Vec<u8> {
        let n_pos = Page::max_length(rec_string.len());
        let b = vec![0_u8; n_pos + size_of::<u32>()];
        let mut page = Page::from_bytes(&b);
        page.set_string(0, rec_string);
        page.set_int(n_pos, n);
        page.contents().to_vec()
    }
}
