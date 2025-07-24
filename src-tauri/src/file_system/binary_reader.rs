use crate::errors::kanban_error::{KanbanError, KanbanErrorKind};
use std::{fs, path::Path, str};

#[derive(Debug)]
pub struct BinaryReader {
    bytes: Vec<u8>,
    address: usize,
}

impl BinaryReader {
    pub fn new(bytes: &[u8]) -> Self {
        BinaryReader {
            bytes: bytes.to_vec(),
            address: 0,
        }
    }

    pub fn read_from_file(path: &Path) -> std::io::Result<Self> {
        let bytes = fs::read(path)?;
        Ok(BinaryReader::new(&bytes))
    }

    fn check_bound(&self, length: usize) -> Result<(), KanbanError> {
        if self.address + length - 1 >= self.bytes.len() {
            return Err(KanbanError {
                kind: KanbanErrorKind::ProjectError,
                message: format!(
                    "Failed to read project file: Out of bound (reading {} out of {})",
                    self.address + length,
                    self.bytes.len()
                ),
                source: None,
            });
        }
        Ok(())
    }

    pub fn next_byte(&mut self) -> Result<u8, KanbanError> {
        self.check_bound(1)?;
        let byte: u8 = self.bytes[self.address];
        self.address += 1;
        Ok(byte)
    }

    pub fn next_bytes(&mut self, length: usize) -> Result<Vec<u8>, KanbanError> {
        self.check_bound(length)?;
        let bytes: Vec<u8> = self.bytes[self.address..self.address + length].to_vec();
        self.address += length;
        Ok(bytes)
    }

    pub fn next_string_by_length(&mut self, length: usize) -> Result<String, KanbanError> {
        self.check_bound(length)?;
        let bytes: &[u8] = &self.bytes[self.address..self.address + length];
        let s = str::from_utf8(bytes)
            .map_err(|e| KanbanError::from_source(KanbanErrorKind::TextError, e))?
            .to_string();
        self.address += length;
        Ok(s)
    }

    pub fn next_leb128_number(&mut self) -> Result<usize, KanbanError> {
        let previous_address: usize = self.address;
        let mut result: usize = 0;
        let mut shift: u32 = 0;
        loop {
            let byte_result = self.next_byte();
            if byte_result.is_err() {
                self.address = previous_address;
                return Err(KanbanError::new(
                    KanbanErrorKind::NumberError,
                    "Failed to parse LEB128 number: Badly formatted.",
                ));
            }
            let byte = byte_result.unwrap();
            result |= ((byte & 0x7F) as usize) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        Ok(result)
    }
    pub fn next_string(&mut self) -> Result<String, KanbanError> {
        let previous_address: usize = self.address;
        let len = self.next_leb128_number()?;
        let result = self.next_string_by_length(len);
        if result.is_err() {
            self.address = previous_address;
            return Err(result.unwrap_err());
        }
        Ok(result.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_read_from_file() {
        let test_content: [u8; 3] = [0x01, 0x02, 0x03];
        let dir = TempDir::new("kanban-test").expect("Failed to create directory");
        let path = dir.path().join("test.bin");
        fs::write(&path, &test_content).expect("Failed to create test file");
        let mut br = BinaryReader::read_from_file(&path).expect("Failed to read file");
        assert_eq!(
            test_content.to_vec(),
            br.next_bytes(3).expect("Failed to read bytes")
        );
    }

    #[test]
    fn test_read_from_file_not_exists() {
        let dir = TempDir::new("kanban-test").expect("Failed to create directory");
        let path = dir.path().join("test.bin");
        let result = BinaryReader::read_from_file(&path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(std::io::ErrorKind::NotFound, err.kind());
    }

    #[test]
    fn test_next_byte() {
        let mut br = BinaryReader::new(&[0x01, 0x02, 0x03]);
        assert_eq!(0x01, br.next_byte().expect("Failed to read byte"));
        assert_eq!(0x02, br.next_byte().expect("Failed to read byte"));
        assert_eq!(0x03, br.next_byte().expect("Failed to read byte"));
        let result = br.next_byte();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, err.kind);
        assert_eq!(
            "Failed to read project file: Out of bound (reading 4 out of 3)",
            err.message
        );
    }

    #[test]
    fn test_next_bytes() {
        let mut br = BinaryReader::new(&[0x01, 0x02, 0x03]);
        assert_eq!(
            [0x01, 0x02].to_vec(),
            br.next_bytes(2).expect("Failed to read byte")
        );
        let result = br.next_bytes(2);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, err.kind);
        assert_eq!(
            "Failed to read project file: Out of bound (reading 4 out of 3)",
            err.message
        );
    }

    #[test]
    fn test_next_string_by_length() {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&[
            0x01, 0x02, 0x03, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64,
            0x21, 0xFD, 0xFE, 0xFF, 0x04,
        ]);
        let mut br = BinaryReader { bytes, address: 3 };
        assert_eq!(
            "Hello World!",
            br.next_string_by_length(12).expect("Failed to read string")
        );
        let result = br.next_string_by_length(3);
        assert!(result.is_err());
        assert_eq!(KanbanErrorKind::TextError, result.unwrap_err().kind);
        br.next_bytes(3).expect("Failed to read bytes");
        let result = br.next_string_by_length(3);
        assert!(result.is_err());
        assert_eq!(KanbanErrorKind::ProjectError, result.unwrap_err().kind);
    }

    #[test]
    fn test_next_leb128_number() {
        // Test 0
        let mut br: BinaryReader = BinaryReader::new(&[0x00]);
        assert_eq!(0, br.next_leb128_number().expect("Failed to parse number"));

        // Test 127
        let mut br: BinaryReader = BinaryReader::new(&[0x7F]);
        assert_eq!(
            127,
            br.next_leb128_number().expect("Failed to parse number")
        );

        // Test 128
        let mut br: BinaryReader = BinaryReader::new(&[0x80, 0x01]);
        assert_eq!(
            128,
            br.next_leb128_number().expect("Failed to parse number")
        );

        // Test 300
        let mut br: BinaryReader = BinaryReader::new(&[0xAC, 0x02]);
        assert_eq!(
            300,
            br.next_leb128_number().expect("Failed to parse number")
        );

        // Test 16384
        let mut br: BinaryReader = BinaryReader::new(&[0x80, 0x80, 0x01, 0x00, 0x01]);
        assert_eq!(
            16384,
            br.next_leb128_number().expect("Failed to parse number")
        );
        assert_eq!(0x00, br.next_byte().expect("Failed to read byte"));
        assert_eq!(0x01, br.next_byte().expect("Failed to read byte"));
    }

    #[test]
    fn test_next_leb128_number_bad_format() {
        let mut br: BinaryReader = BinaryReader::new(&[0x80, 0x81, 0x82]);
        let result = br.next_leb128_number();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::NumberError, err.kind);
        assert_eq!(
            "Failed to parse LEB128 number: Badly formatted.",
            err.message
        );
    }

    #[test]
    fn test_next_string() {
        let mut br = BinaryReader::new(&[
            0x0C, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x03,
            0xFD, 0xFE, 0xFF, 0x04,
        ]);
        assert_eq!(
            "Hello World!",
            br.next_string().expect("Failed to read string")
        );
        let result = br.next_string();
        assert!(result.is_err());
        assert_eq!(KanbanErrorKind::TextError, result.unwrap_err().kind);
        assert_eq!(0x03, br.next_byte().expect("Failed to read byte"))
    }
}
