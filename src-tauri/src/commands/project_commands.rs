use super::CommandResult;
use crate::kanban::project;

#[tauri::command]
pub fn create_project_command<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    name: &str,
    description: &str,
) -> CommandResult<project::Project> {
    let result = project::create_project(&app, name, description);
    if result.is_err() {
        let err = result.unwrap_err();
        return CommandResult {
            success: false,
            data: None,
            message: Some(err.message),
        };
    }
    let project = result.unwrap();
    CommandResult {
        success: true,
        data: Some(project),
        message: None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tauri::Manager;

    #[test]
    fn test_create_project_command() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test
        let result = create_project_command(app.clone(), "Test Project", "Test Description");
        assert!(result.success);
        assert!(result.data.is_some());
        let project = result.data.unwrap();
        assert_eq!("Test Project", project.name);
        assert_eq!("Test Description", project.description);
        assert_eq!(0, project.boards.len());
        assert!(result.message.is_none());
        let file_name: String = project.id.iter().map(|b| format!("{:02X}", b)).collect();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join("projects")
            .join(file_name);
        assert!(fs::exists(&project_path).expect("Failed to check exists"));
        fs::remove_file(project_path).expect("Failed to remove file");
    }

    #[test]
    fn test_create_project_with_error() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test
        let result = create_project_command(app.clone(), "", "");
        assert!(!result.success);
        assert!(result.data.is_none());
        assert!(result.message.is_some());
        assert_eq!(
            "Empty Name or Description: The name and description of the project must not be empty",
            result.message.unwrap()
        );
    }
}
