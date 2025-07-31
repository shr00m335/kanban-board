use crate::{
    errors::kanban_error::{KanbanError, KanbanErrorKind},
    file_system::binary_writer::BinaryWriter,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct BoardList {
    title: String,
    items: Vec<String>,
}

pub(crate) fn write_board_list(
    bw: &mut BinaryWriter,
    board_list: &BoardList,
) -> Result<(), KanbanError> {
    bw.write_string_with_length(&board_list.title, true);
    // Board Items
    bw.write_leb128(board_list.items.len().try_into().map_err(|e| {
        KanbanError::new(
            KanbanErrorKind::NumberError,
            "Failed to covert u32 to usize",
        )
    })?);
    for item in board_list.items.iter() {
        bw.write_string_with_length(item, false);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_board_list() {
        let test_list = BoardList {
            title: "Test List 1".to_string(),
            items: ["Test Item 1", "Test Item 2", "Test Item 3"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        };
        let expected_data = [
            11, 0x54, 0x65, 0x73, 0x74, 0x20, 0x4C, 0x69, 0x73, 0x74, 0x20, 0x31, 3, 11, 0x54,
            0x65, 0x73, 0x74, 0x20, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 11, 0x54, 0x65, 0x73, 0x74,
            0x20, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 11, 0x54, 0x65, 0x73, 0x74, 0x20, 0x49, 0x74,
            0x65, 0x6D, 0x20, 0x33,
        ];
        let mut bw = BinaryWriter::new();
        let result = write_board_list(&mut bw, &test_list);
        assert!(result.is_ok());
        assert_eq!(&expected_data, bw.as_bytes());
    }
}
