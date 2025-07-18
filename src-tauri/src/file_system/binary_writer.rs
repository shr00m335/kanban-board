pub struct BinaryWriter {
    bytes: Vec<u8>,
}

impl BinaryWriter {
    pub fn new() -> BinaryWriter {
        return BinaryWriter { bytes: Vec::new() };
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.bytes;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_bytes() {
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(&[0x01, 0x02, 0x03], bw.as_bytes());
    }

    #[test]
    fn test_as_bytes() {
        // Test Empty
        let bw = BinaryWriter { bytes: Vec::new() };
        assert_eq!(&Vec::<u8>::new(), bw.as_bytes());
        // Test with content
        let bw = BinaryWriter {
            bytes: [0x01, 0x02, 0x03].to_vec(),
        };
        assert_eq!(&[0x01, 0x02, 0x03], bw.as_bytes());
    }
}
