use crate::{
    errors::kanban_error::{KanbanError, KanbanErrorKind},
    file_system::binary_writer::BinaryWriter,
};

#[derive(Debug, serde::Serialize)]
pub struct Board {
    name: String,
    items: Vec<String>,
}

fn write_board(bw: &mut BinaryWriter, board: &Board) -> Result<(), KanbanError> {
    // Write board name
    bw.write_string_with_length(&board.name, true);
    // Write board items count
    bw.write_leb128(board.items.len().try_into().map_err(|_| {
        KanbanError::new(
            KanbanErrorKind::NumberError,
            "Failed to convert u32 to usize",
        )
    })?);
    // Write board items
    for item in board.items.iter() {
        bw.write_string_with_length(&item, false);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_board() {
        let mut bw = BinaryWriter::new();
        let test_board = Board {
            name: "Test Board".to_string(),
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let result = write_board(&mut bw, &test_board);
        assert!(result.is_ok());
        let expected_bytes = &[
            0x0A, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x03, 0x06, 0x49,
            0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x06, 0x49,
            0x74, 0x65, 0x6D, 0x20, 0x33,
        ];
        assert_eq!(expected_bytes, bw.as_bytes());
    }
}
