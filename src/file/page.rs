use std::{mem::size_of, string::FromUtf8Error};

pub struct Page {
    pub byte_buffer: Vec<u8>,
}

impl Page {
    // A constructor for creating data buffers
    pub fn new(block_size: usize) -> Self {
        Self {
            byte_buffer: vec![0; block_size],
        }
    }

    pub fn from_bytes(b: &[u8]) -> Self {
        Self {
            byte_buffer: b.to_vec(),
        }
    }

    pub fn get_int(&self, offset: usize) -> u32 {
        u32::from_le_bytes(
            self.byte_buffer[offset..offset + size_of::<u32>()]
                .try_into()
                .expect("slice with incorrect length"),
        )
    }

    pub fn set_int(&mut self, offset: usize, n: u32) {
        let le_bytes = n.to_le_bytes();
        self.byte_buffer[offset..offset + le_bytes.len()].copy_from_slice(&le_bytes);
    }

    pub fn get_bytes(&self, offset: usize) -> &[u8] {
        let len = self.get_int(offset);
        &self.byte_buffer[offset + size_of::<u32>()..offset + size_of::<u32>() + len as usize]
    }

    pub fn get_string(&self, offset: usize) -> Result<String,FromUtf8Error> {
        let len = self.get_int(offset);
        String::from_utf8(
            self.byte_buffer[offset + size_of::<u32>()..offset + size_of::<u32>() + len as usize]
                .to_vec(),
        )
    }

    pub fn set_bytes(&mut self, offset: usize, b: &[u8]) {
        let len = b.len();
        self.set_int(offset, len as u32);
        self.byte_buffer[offset + size_of::<u32>()..offset + size_of::<u32>() + len]
            .copy_from_slice(b);
    }

    pub fn set_string(&mut self, offset: usize, s: &str) {
        self.set_bytes(offset, s.as_bytes())
    }

    pub fn max_length(strlen: usize) -> usize {
        size_of::<u32>() + strlen * size_of::<u8>()
    }

    pub fn contents(&self) -> &[u8] {
        &self.byte_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_string() {
        // Test that we can set and get a string
        let mut page = Page::new(4096);
        let s = "Hello, world!";
        page.set_string(0, s);
        assert_eq!(page.get_string(0).unwrap(), s);

        // Test with larger string
        page.set_string(
            100,
            "Hello, world! This is a longer string.Hello, world! This is a longer string.Hello, world! This is a longer string",
        );
        assert_eq!(
            page.get_string(100).unwrap(),
            "Hello, world! This is a longer string.Hello, world! This is a longer string.Hello, world! This is a longer string"
                
        );
    }
}
