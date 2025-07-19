use crate::file_system::binary_writer::BinaryWriter;
use crate::kanban::board::Board;

pub struct Project {
    name: String,
    description: String,
    boards: Vec<Board>,
}

const FILE_VERSION: u8 = 0;

fn write_project_header(bw: &mut BinaryWriter, name: &str, description: &str) {
    // Version
    bw.write_byte(FILE_VERSION);
    // Name
    bw.write_string_with_length(name, true);
    // Description
    bw.write_string_with_length(description, false);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_project_header() {
        let mut bw: BinaryWriter = BinaryWriter::new();
        write_project_header(&mut bw, "Test Name", "Test Description");
        let expected_bytes: [u8; 28] = [
            0x00, 0x09, 0x54, 0x65, 0x73, 0x74, 0x20, 0x4E, 0x61, 0x6D, 0x65, 0x10, 0x54, 0x65,
            0x73, 0x74, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E,
        ];
        assert_eq!(&expected_bytes, bw.as_bytes());
    }
}
