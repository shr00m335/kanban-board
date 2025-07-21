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
}
