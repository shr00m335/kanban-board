use crate::{
    errors::kanban_error::{KanbanError, KanbanErrorKind},
    file_system::{binary_reader::BinaryReader, binary_writer::BinaryWriter},
};

#[derive(Debug, serde::Serialize, Clone, PartialEq)]
pub struct Board {
    pub name: String,
    pub items: Vec<String>,
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

pub(crate) fn write_all_boards(bw: &mut BinaryWriter, boards: &[Board]) -> Result<(), KanbanError> {
    // Write number of boards
    bw.write_leb128(boards.len().try_into().map_err(|_| {
        KanbanError::new(
            KanbanErrorKind::NumberError,
            "Failed to convert u32 to usize",
        )
    })?);
    // Write boards
    for board in boards.iter() {
        write_board(bw, board)?;
    }
    Ok(())
}

fn read_board(br: &mut BinaryReader) -> Result<Board, KanbanError> {
    let name = br.next_string(true)?;
    let items_count = br.next_leb128_number()?;
    let mut items = Vec::<String>::new();
    for _ in 0..items_count {
        items.push(br.next_string(false)?);
    }
    Ok(Board { name, items })
}

pub(crate) fn read_all_boards(br: &mut BinaryReader) -> Result<Vec<Board>, KanbanError> {
    let boards_count = br.next_leb128_number()?;
    let mut boards = Vec::<Board>::new();
    for _ in 0..boards_count {
        boards.push(read_board(br)?);
    }
    Ok(boards)
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

    #[test]
    fn test_write_all_boards() {
        let mut bw = BinaryWriter::new();
        let test_board_1 = Board {
            name: "Test Board 1".to_string(),
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let test_board_2 = Board {
            name: "Test Board 2".to_string(),
            items: ["Item 1", "Item 2"].map(|s| s.to_string()).to_vec(),
        };
        let test_board_3 = Board {
            name: "Test Board 3".to_string(),
            items: ["Item 1"].map(|s| s.to_string()).to_vec(),
        };
        let result = write_all_boards(&mut bw, &[test_board_1, test_board_2, test_board_3]);
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
    fn test_read_board() {
        let mut br = BinaryReader::new(&[
            0x0A, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x03, 0x06, 0x49,
            0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x06, 0x49,
            0x74, 0x65, 0x6D, 0x20, 0x33,
        ]);
        let expected_board = Board {
            name: "Test Board".to_string(),
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let result = read_board(&mut br);
        assert!(result.is_ok());
        assert_eq!(expected_board, result.unwrap());
    }

    #[test]
    fn test_read_all_boards() {
        let mut br = BinaryReader::new(&[
            0x03, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x31,
            0x03, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x32, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x33, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x32, 0x02, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x33, 0x01, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x31,
        ]);
        let expected_boards = [
            Board {
                name: "Test Board 1".to_string(),
                items: ["Item 1", "Item 2", "Item 3"]
                    .map(|s| s.to_string())
                    .to_vec(),
            },
            Board {
                name: "Test Board 2".to_string(),
                items: ["Item 1", "Item 2"].map(|s| s.to_string()).to_vec(),
            },
            Board {
                name: "Test Board 3".to_string(),
                items: ["Item 1"].map(|s| s.to_string()).to_vec(),
            },
        ]
        .to_vec();
        let result = read_all_boards(&mut br);
        assert!(result.is_ok());
        assert_eq!(expected_boards, result.unwrap());
    }
}
