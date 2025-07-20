pub mod project_commands;

pub struct CommandResult<T> {
    pub success: bool,
    pub data: T,
    pub message: String,
}
