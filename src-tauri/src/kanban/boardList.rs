use crate::{
    errors::kanban_error::{KanbanError, KanbanErrorKind},
    file_system::{binary_reader::BinaryReader, binary_writer::BinaryWriter},
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct BoardList {
    pub title: String,
    pub items: Vec<String>,
}

fn write_board_list(bw: &mut BinaryWriter, board_list: &BoardList) -> Result<(), KanbanError> {
    bw.write_string_with_length(&board_list.title, true);
    // Board Items
    bw.write_leb128(board_list.items.len().try_into().map_err(|_| {
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

pub(crate) fn write_all_board_lists(
    bw: &mut BinaryWriter,
    board_lists: &[BoardList],
) -> Result<(), KanbanError> {
    bw.write_leb128(board_lists.len().try_into().map_err(|_| {
        KanbanError::new(
            KanbanErrorKind::NumberError,
            "Failed to covert u32 to usize",
        )
    })?);
    for board_list in board_lists.iter() {
        write_board_list(bw, &board_list)?;
    }
    Ok(())
}

pub(crate) fn read_board_list(br: &mut BinaryReader) -> Result<BoardList, KanbanError> {
    // Board list title
    let title: String = br.next_string(true)?;
    // Board items
    let items_count: usize = br.next_leb128_number()?;
    let mut items: Vec<String> = Vec::with_capacity(items_count);
    for _ in 0..items_count {
        items.push(br.next_string(false)?);
    }
    Ok(BoardList { title, items })
}

pub(crate) fn read_all_board_lists(br: &mut BinaryReader) -> Result<Vec<BoardList>, KanbanError> {
    let lists_count = br.next_leb128_number()?;
    let mut lists = Vec::with_capacity(lists_count);
    for _ in 0..lists_count {
        lists.push(read_board_list(br)?);
    }
    Ok(lists)
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

    #[test]
    fn test_write_all_board_lists() {
        let mut bw = BinaryWriter::new();
        let test_list_1 = BoardList {
            title: "Test Board 1".to_string(),
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let test_list_2 = BoardList {
            title: "Test Board 2".to_string(),
            items: ["Item 1", "Item 2"].map(|s| s.to_string()).to_vec(),
        };
        let test_list_3 = BoardList {
            title: "Test Board 3".to_string(),
            items: ["Item 1"].map(|s| s.to_string()).to_vec(),
        };
        let result = write_all_board_lists(&mut bw, &[test_list_1, test_list_2, test_list_3]);
        assert!(result.is_ok());
        let expected_bytes = &[
            0x03, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x31,
            0x03, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x32, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x33, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x32, 0x02, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x33, 0x01, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x31,
        ];
        assert_eq!(expected_bytes, bw.as_bytes());
    }

    #[test]
    fn test_read_board_list() {
        let test_data = [
            11, 0x54, 0x65, 0x73, 0x74, 0x20, 0x4C, 0x69, 0x73, 0x74, 0x20, 0x31, 3, 11, 0x54,
            0x65, 0x73, 0x74, 0x20, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 11, 0x54, 0x65, 0x73, 0x74,
            0x20, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 11, 0x54, 0x65, 0x73, 0x74, 0x20, 0x49, 0x74,
            0x65, 0x6D, 0x20, 0x33,
        ];
        let expected_list = BoardList {
            title: "Test List 1".to_string(),
            items: ["Test Item 1", "Test Item 2", "Test Item 3"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        };
        let mut br = BinaryReader::new(&test_data);
        let result = read_board_list(&mut br);
        assert!(result.is_ok());
        assert_eq!(expected_list, result.unwrap());
    }

    #[test]
    fn test_read_all_board_lists() {
        let test_data = [
            0x03, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x31,
            0x03, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x32, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x33, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x32, 0x02, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x33, 0x01, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x31,
        ];
        let test_list_1 = BoardList {
            title: "Test Board 1".to_string(),
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let test_list_2 = BoardList {
            title: "Test Board 2".to_string(),
            items: ["Item 1", "Item 2"].map(|s| s.to_string()).to_vec(),
        };
        let test_list_3 = BoardList {
            title: "Test Board 3".to_string(),
            items: ["Item 1"].map(|s| s.to_string()).to_vec(),
        };
        let expected_lists = [test_list_1, test_list_2, test_list_3];
        let mut br = BinaryReader::new(&test_data);
        let result = read_all_board_lists(&mut br);
        assert!(result.is_ok());
        assert_eq!(expected_lists.to_vec(), result.unwrap());
    }
}
