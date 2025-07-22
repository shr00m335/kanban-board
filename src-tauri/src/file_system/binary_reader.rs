use crate::errors::kanban_error::{KanbanError, KanbanErrorKind};
use std::str;

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

    fn peek(&self) -> Result<u8, KanbanError> {
        self.check_bound(1)?;
        Ok(self.bytes[self.address])
    }

    pub fn next_byte(&mut self) -> Result<u8, KanbanError> {
        self.check_bound(1)?;
        let byte: u8 = self.bytes[self.address];
        self.address += 1;
        Ok(byte)
    }

    pub fn next_bytes(&mut self, length: usize) -> Result<&[u8], KanbanError> {
        self.check_bound(length)?;
        let bytes: &[u8] = &self.bytes[self.address..self.address + length];
        self.address += length;
        Ok(bytes)
    }

    pub fn next_string_by_length(&mut self, length: usize) -> Result<&str, KanbanError> {
        self.check_bound(length)?;
        let bytes: &[u8] = &self.bytes[self.address..self.address + length];
        let str = str::from_utf8(bytes)
            .map_err(|e| KanbanError::from_source(KanbanErrorKind::TextError, e))?;
        self.address += length;
        Ok(str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peek() {
        let mut test_bytes: Vec<u8> = Vec::new();
        test_bytes.extend_from_slice(&[0x01, 0x02, 0x03]);
        let mut br = BinaryReader {
            bytes: test_bytes.to_vec(),
            address: 1,
        };
        let _ = br.next_bytes(2);
        let result = br.peek();
        assert!(result.is_err());
        assert_eq!(KanbanErrorKind::ProjectError, result.unwrap_err().kind);
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
            &[0x01, 0x02],
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
}
