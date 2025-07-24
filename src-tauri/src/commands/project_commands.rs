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

#[tauri::command]
pub fn get_all_projects<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> CommandResult<Vec<project::Project>> {
    let result = project::get_all_projects_info(&app);
    if result.is_err() {
        let err = result.unwrap_err();
        return CommandResult {
            success: false,
            data: None,
            message: Some(err.message),
        };
    }
    let projects = result.unwrap();
    CommandResult {
        success: true,
        data: Some(projects),
        message: None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tauri::Manager;

    #[test]
    #[serial]
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

    #[test]
    #[serial]
    fn test_get_all_projects() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let project_1 = project::create_project(app, "Test Project 1", "Test Description 1")
            .expect("Failed to create test project");
        let project_2 = project::create_project(app, "Test Project 2", "Test Description 2")
            .expect("Failed to create test project");
        let project_3 = project::create_project(app, "Test Project 3", "Test Description 3")
            .expect("Failed to create test project");
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join("projects");
        let file_path = project_path.join("invalid_id");
        fs::write(file_path, Vec::<u8>::new()).expect("Failed failed to create test project");
        let result = get_all_projects(app.clone());
        assert!(result.success);
        assert!(result.message.is_none());
        assert!(result.data.is_some());
        let projects = result.data.unwrap();
        assert_eq!(3, projects.len());
        assert!(projects.iter().find(|x| x.id == project_1.id).is_some());
        assert!(projects.iter().find(|x| x.id == project_2.id).is_some());
        assert!(projects.iter().find(|x| x.id == project_3.id).is_some());
        fs::remove_file(
            project_path.join(
                project_1
                    .id
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<String>(),
            ),
        )
        .expect("Failed to remove file");
        fs::remove_file(
            project_path.join(
                project_2
                    .id
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<String>(),
            ),
        )
        .expect("Failed to remove file");
        fs::remove_file(
            project_path.join(
                project_3
                    .id
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<String>(),
            ),
        )
        .expect("Failed to remove file");
        fs::remove_file(project_path.join("invalid_id")).expect("Failed to remove file");
    }

    #[test]
    #[serial]
    fn test_get_all_projects_with_error() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join("projects");
        if fs::exists(&project_path).expect("Unable to check whether path exists") {
            fs::remove_dir(&project_path).expect("Failed to remove dir");
        }
        let result = get_all_projects(app.clone());
        assert!(!result.success);
        assert!(result.data.is_none());
        assert!(result.message.is_some());
    }
}
