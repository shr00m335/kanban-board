#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct BoardList {
    title: String,
    items: Vec<String>,
}
