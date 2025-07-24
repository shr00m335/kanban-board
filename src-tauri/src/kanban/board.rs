use crate::file_system::binary_writer::BinaryWriter;

#[derive(Debug, serde::Serialize)]
pub struct Board {
    name: String,
    items: Vec<String>,
}
