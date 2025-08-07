use std::fs;

use tauri::Manager;

use crate::errors::kanban_error::{KanbanError, KanbanErrorKind};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Configs {
    pub autoSaveInterval: u32,
    pub newListDefaultColor: String,
}

pub fn save_configs<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    configs: &Configs,
) -> Result<(), KanbanError> {
    let config_path = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::TauriError, e))?
        .join("configs.json");
    let config_json = serde_json::to_string(&configs)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::TextError, e))?;
    fs::write(&config_path, config_json)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    Ok(())
}

pub fn read_configs<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<Configs, KanbanError> {
    let config_path = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::TauriError, e))?
        .join("configs.json");
    let file_content = fs::read_to_string(&config_path)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    let configs: Configs = serde_json::from_str(&file_content)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    Ok(configs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_save_configs() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let test_configs = Configs {
            autoSaveInterval: 300,
            newListDefaultColor: "#FFFFFF".to_string(),
        };
        let result = save_configs(app, &test_configs);
        assert!(result.is_ok());
        let config_path = app
            .path()
            .app_data_dir()
            .map_err(|e| KanbanError::from_source(KanbanErrorKind::TauriError, e))
            .expect("Failed to get path")
            .join("configs.json");
        let config_file_content = fs::read_to_string(&config_path).expect("Failed to read file");
        assert_eq!(
            "{\"autoSaveInterval\":300,\"newListDefaultColor\":\"#FFFFFF\"}",
            config_file_content
        );
        fs::remove_file(&config_path).expect("Failed to remove file");
    }

    #[test]
    fn test_read_configs() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let test_configs = Configs {
            autoSaveInterval: 300,
            newListDefaultColor: "#FFFFFF".to_string(),
        };
        save_configs(app, &test_configs).expect("Failed to save config");
        let result = read_configs(&app);
        assert!(result.is_ok());
        assert_eq!(test_configs, result.unwrap());
        let config_path = app
            .path()
            .app_data_dir()
            .map_err(|e| KanbanError::from_source(KanbanErrorKind::TauriError, e))
            .expect("Failed to get path")
            .join("configs.json");
        fs::remove_file(&config_path).expect("Failed to remove file");
    }
}
