use crate::file_system::binary_writer::BinaryWriter;
use crate::kanban::board::Board;
use uuid::Uuid;

pub struct Project {
    id: String,
    name: String,
    description: String,
    boards: Vec<Board>,
}

const FILE_VERSION: u8 = 0;

fn write_project_header(bw: &mut BinaryWriter, id: &Uuid, name: &str, description: &str) {
    // Version
    bw.write_byte(FILE_VERSION);
    // ID
    bw.write_bytes(id.as_bytes());
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
        let id: Uuid = Uuid::new_v4();
        write_project_header(&mut bw, &id, "Test Name", "Test Description");
        let mut expected_bytes: Vec<u8> = Vec::new();
        expected_bytes.push(0x00);
        expected_bytes.extend_from_slice(id.as_bytes());
        expected_bytes.extend_from_slice(&[
            0x09, 0x54, 0x65, 0x73, 0x74, 0x20, 0x4E, 0x61, 0x6D, 0x65, 0x10, 0x54, 0x65, 0x73,
            0x74, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E,
        ]);
        assert_eq!(&expected_bytes, bw.as_bytes());
    }
}
