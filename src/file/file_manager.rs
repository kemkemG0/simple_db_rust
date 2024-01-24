use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs, io, path};

use crate::file::block_id::BlockId;
use crate::file::page::Page;

pub struct FileManager {
    db_directory: path::PathBuf,
    block_size: usize,
    is_new: bool,
    open_files: Mutex<HashMap<PathBuf, Arc<Mutex<fs::File>>>>,
}

impl FileManager {
    pub fn new(db_directory: path::PathBuf, block_size: usize) -> Self {
        let is_new = !db_directory.exists();
        if is_new {
            match fs::create_dir_all(&db_directory) {
                Ok(_) => {}
                Err(e) => panic!("cannot create directory: {}", e),
            }
        }

        // remove any leftover temp tables
        match fs::read_dir(path::Path::new(&db_directory)) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_file() {
                                match fs::remove_file(path) {
                                    Ok(_) => {}
                                    Err(e) => println!("cannot remove file: {}", e),
                                }
                            }
                        }
                        Err(e) => println!("cannot read directory: {}", e),
                    }
                }
            }
            Err(e) => panic!("cannot read directory: {}", e),
        }

        Self {
            db_directory,
            block_size,
            is_new,
            open_files: Mutex::new(HashMap::new()),
        }
    }

    pub fn read(&self, blk: &BlockId, page: &mut Page) -> Result<(), io::Error> {
        let file_io = self.get_file(blk.filename())?;
        let mut file_io = file_io.lock().unwrap();
        let offset = blk.number() * self.block_size as u64;
        file_io.seek(io::SeekFrom::Start(offset))?;
        let mut tmp_buff = vec![0; self.block_size];
        let read_len = file_io.read(&mut tmp_buff)?;
        if read_len == self.block_size {
            page.byte_buffer = tmp_buff;
        }
        Ok(())
    }
    pub fn write(&self, blk: &BlockId, page: &mut Page) -> Result<(), io::Error> {
        let file_io = self.get_file(blk.filename())?;
        let mut file_io = file_io.lock().unwrap();
        let offset = blk.number() * self.block_size as u64;
        file_io.seek(io::SeekFrom::Start(offset))?;
        file_io.write_all(&page.byte_buffer[0..self.block_size()])?;
        file_io.flush()?;
        Ok(())
    }

    pub fn append(&self, filename: &str) -> Result<BlockId, io::Error> {
        let new_blk_num = self.length(filename);
        let blk = BlockId::new(filename, new_blk_num);
        let file_io = self.get_file(blk.filename())?;
        let mut file_io = file_io.lock().unwrap();
        let b = vec![0; self.block_size()];
        file_io.seek(io::SeekFrom::Start(blk.number() * self.block_size() as u64))?;
        file_io.write_all(&b)?;
        file_io.flush()?;
        Ok(blk)
    }
    pub fn is_new(&self) -> bool {
        self.is_new
    }
    pub fn length(&self, file_name: &str) -> u64 {
        let file_io = self.get_file(file_name).unwrap();
        let file_io = file_io.lock().unwrap();
        let metadata = file_io.metadata().unwrap();
        metadata.len() / self.block_size as u64
    }
    pub fn block_size(&self) -> usize {
        self.block_size
    }

    fn get_file(&self, filename: &str) -> Result<Arc<Mutex<fs::File>>, io::Error> {
        let path = self.db_directory.join(filename);
        let mut open_files = self.open_files.lock().unwrap();
        if let Some(file) = open_files.get(&path) {
            return Ok(file.clone());
        }
        let arc_file = Arc::new(Mutex::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.clone())?,
        ));
        open_files.insert(path, arc_file.clone());
        Ok(arc_file)
    }
}
