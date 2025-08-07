use crate::{commands::CommandResult, kanban::config};

#[tauri::command]
pub fn save_configs<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    configs: config::Configs,
) -> CommandResult<()> {
    let result = config::save_configs(&app, &configs);
    if result.is_err() {
        return CommandResult {
            success: false,
            data: None,
            message: Some(result.unwrap_err().message),
        };
    }
    CommandResult {
        success: true,
        data: None,
        message: None,
    }
}

#[tauri::command]
pub fn get_configs<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> CommandResult<config::Configs> {
    let result = config::read_configs(&app);
    if result.is_err() {
        return CommandResult {
            success: false,
            data: None,
            message: Some(result.unwrap_err().message),
        };
    }
    CommandResult {
        success: true,
        data: Some(result.unwrap()),
        message: None,
    }
}
