pub mod configs_commands;
pub mod project_commands;

#[derive(Debug, serde::Serialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}
