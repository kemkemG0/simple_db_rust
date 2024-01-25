#[cfg(test)]
mod tests {
    use std::{
        fs,
        iter::zip,
        mem::size_of,
        sync::{Arc, Mutex},
    };

    use crate::{app::simple_db::SimpleDB, file::block_id::BlockId, tx::transaction::Transaction};
    #[test]
    fn test_transaction() {
        let db = SimpleDB::new("./test_transaction", 400, 8).unwrap();
        let mut tx1 = Transaction::new(
            db.file_manager().clone(),
            db.log_manager().clone(),
            db.buffer_manager().clone(),
        );

        let blk = BlockId::new("testfile", 1);
        tx1.pin(&blk);
        tx1.set_int(&blk, 80, 1, false);
        tx1.set_string(&blk, 40, "one", false);
        tx1.commit();

        let mut tx2 = Transaction::new(
            db.file_manager().clone(),
            db.log_manager().clone(),
            db.buffer_manager().clone(),
        );
        tx2.pin(&blk);
        let i_val = tx2.get_int(&blk, 80);
        let s_val = tx2.get_string(&blk, 40);

        assert_eq!(i_val, 1);
        assert_eq!(s_val, "one");

        let new_i_val = i_val + 1;
        let new_s_val = s_val.to_string() + "!";
        tx2.set_int(&blk, 80, new_i_val, true);
        tx2.set_string(&blk, 40, &new_s_val, true);
        tx2.commit();

        let mut tx3 = Transaction::new(
            db.file_manager().clone(),
            db.log_manager().clone(),
            db.buffer_manager().clone(),
        );
        tx3.pin(&blk);
        let i_val = tx3.get_int(&blk, 80);
        let s_val = tx3.get_string(&blk, 40);

        assert_eq!(i_val, 2);
        assert_eq!(s_val, "one!");

        tx3.set_int(&blk, 80, 9999, true);
        println!("pre rollback");
        assert_eq!(tx3.get_int(&blk, 80), 9999);
        tx3.rollback();

        let mut tx4: Transaction = Transaction::new(
            db.file_manager().clone(),
            db.log_manager().clone(),
            db.buffer_manager().clone(),
        );
        tx4.pin(&blk);
        println!("post rollback");
        assert_eq!(tx4.get_int(&blk, 80), 2);
        tx4.commit();

        fs::remove_dir_all("./test_transaction").unwrap();
    }
}
