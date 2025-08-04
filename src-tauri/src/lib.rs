mod commands;
mod errors;
mod file_system;
mod kanban;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::project_commands::create_project_command,
            commands::project_commands::get_all_projects,
            commands::project_commands::read_project,
            commands::project_commands::save_project,
            commands::project_commands::delete_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
