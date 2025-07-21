use crate::errors::kanban_error::{KanbanError, KanbanErrorKind};

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
}

#[cfg(test)]
mod test {
    use super::*;

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
}
