pub struct BinaryWriter {
    bytes: Vec<u8>,
}

impl BinaryWriter {
    pub fn new() -> BinaryWriter {
        return BinaryWriter { bytes: Vec::new() };
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn write_leb128(&mut self, mut value: u32) {
        loop {
            let mut byte: u8 = (value & 0x7F) as u8; // Extract the right most 7 bits
            value >>= 7;

            if value != 0 {
                byte |= 0x80; // Set continuation bit
            }

            self.write_byte(byte);

            if value == 0 {
                break;
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.bytes;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_byte() {
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_byte(0x01);
        assert_eq!(&[0x01], bw.as_bytes());
    }

    #[test]
    fn test_write_bytes() {
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(&[0x01, 0x02, 0x03], bw.as_bytes());
    }

    #[test]
    fn test_write_leb128() {
        // Test 0
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_leb128(0);
        assert_eq!(&[0x00], bw.as_bytes());

        // Test 127
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_leb128(127);
        assert_eq!(&[0x7F], bw.as_bytes());

        // Test 128
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_leb128(128);
        assert_eq!(&[0x80, 0x01], bw.as_bytes());

        // Test 300
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_leb128(300);
        assert_eq!(&[0xAC, 0x02], bw.as_bytes());

        // Test 16384
        let mut bw: BinaryWriter = BinaryWriter::new();
        bw.write_leb128(16384);
        assert_eq!(&[0x80, 0x80, 0x01], bw.as_bytes());
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
