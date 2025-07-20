use crate::errors::kanban_error::{KanbanError, KanbanErrorKind};
use crate::file_system::binary_writer::BinaryWriter;
use crate::kanban::board::Board;
use uuid::Uuid;

pub struct Project {
    id: String,
    name: String,
    description: String,
    boards: Vec<Board>,
}

pub trait AppPathProvider {
    type Path: PathProvider;
    fn path(&self) -> &Self::Path;
}

pub trait PathProvider {
    fn app_data_dir(&self) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>;
}

impl<R: tauri::Runtime> AppPathProvider for tauri::AppHandle<R> {
    type Path = tauri::path::PathResolver<R>;
    fn path(&self) -> &Self::Path {
        tauri::Manager::path(self)
    }
}

impl<R: tauri::Runtime> PathProvider for tauri::path::PathResolver<R> {
    fn app_data_dir(&self) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        self.app_data_dir().map_err(|e| Box::new(e) as _)
    }
}

const FILE_VERSION: u8 = 0;
const PROJECT_PATH: &str = "projects";

fn write_project_header(bw: &mut BinaryWriter, id: &Uuid, name: &str, description: &str) {
    // Version
    bw.write_byte(FILE_VERSION);
    // ID
    bw.write_bytes(id.as_bytes());
    // Name
    bw.write_string_with_length(name, true);
    // Description
    bw.write_string_with_length(description, false);
}

fn write_project_to_file<P: AppPathProvider>(
    app: &P,
    bw: &BinaryWriter,
) -> Result<(), KanbanError> {
    let file_content: &[u8] = bw.as_bytes();
    // Check project header
    if file_content.len() < 20 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Missing project header".to_string(),
        ));
    }
    // Get project ID
    let file_name: String = file_content[1..17]
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    // Get Project Path
    let project_path = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_box_source(KanbanErrorKind::TauriError, e))?
        .join(PROJECT_PATH)
        .join(file_name);
    // Write to file
    bw.write_to_file(&project_path)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, os::unix::fs::PermissionsExt};
    use tauri::Manager;
    use tempdir::TempDir;

    #[test]
    fn test_write_project_header() {
        let mut bw: BinaryWriter = BinaryWriter::new();
        let id: Uuid = Uuid::new_v4();
        write_project_header(&mut bw, &id, "Test Name", "Test Description");
        let mut expected_bytes: Vec<u8> = Vec::new();
        expected_bytes.push(0x00);
        expected_bytes.extend_from_slice(id.as_bytes());
        expected_bytes.extend_from_slice(&[
            0x09, 0x54, 0x65, 0x73, 0x74, 0x20, 0x4E, 0x61, 0x6D, 0x65, 0x10, 0x54, 0x65, 0x73,
            0x74, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E,
        ]);
        assert_eq!(&expected_bytes, bw.as_bytes());
    }

    #[test]
    fn test_write_project_to_file() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test data
        let mut bw: BinaryWriter = BinaryWriter::new();
        let id: Uuid = Uuid::new_v4();
        write_project_header(&mut bw, &id, "Test Project", "Test Description");
        // Test result
        let result = write_project_to_file(app, &bw);
        assert!(result.is_ok());
        let file_name: String = id.as_bytes().iter().map(|b| format!("{:02X}", b)).collect();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(file_name);
        assert!(fs::exists(&project_path).expect("Failed to check exists"));
        fs::remove_file(project_path).expect("Failed to remove file");
    }

    #[test]
    fn test_write_project_to_file_no_header() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test data
        let bw: BinaryWriter = BinaryWriter::new();
        // Test result
        let result = write_project_to_file(app, &bw);
        assert!(result.is_err());
        assert_eq!(KanbanErrorKind::ProjectError, result.unwrap_err().kind);
    }

    #[test]
    fn test_write_project_to_file_app_data_dir_error() {
        // Mock app handle
        struct MockAppPathProvider {
            path: MockPath,
        }

        impl AppPathProvider for MockAppPathProvider {
            type Path = MockPath;
            fn path(&self) -> &Self::Path {
                &self.path
            }
        }
        struct MockPath;
        impl PathProvider for MockPath {
            fn app_data_dir(
                &self,
            ) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
                Err("tauri path error".into())
            }
        }
        // Test
        let mock_app = MockAppPathProvider { path: MockPath };
        let mut bw = BinaryWriter::new();
        let id = Uuid::new_v4();
        write_project_header(&mut bw, &id, "Test Project", "Test Description");
        let result = write_project_to_file(&mock_app, &bw);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::TauriError, err.kind);
        assert_eq!("tauri path error", err.message);
    }

    #[test]
    fn test_write_project_to_file_readonly_dir() {
        // Mock app handle
        struct MockAppPathProvider {
            path: MockPath,
        }

        impl AppPathProvider for MockAppPathProvider {
            type Path = MockPath;
            fn path(&self) -> &Self::Path {
                &self.path
            }
        }
        struct MockPath {
            path: std::path::PathBuf,
        }
        impl PathProvider for MockPath {
            fn app_data_dir(
                &self,
            ) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
                Ok(self.path.clone())
            }
        }
        // Create readonly dir
        let readonly_dir = TempDir::new("kanban-test").expect("Failed to create directory");
        fs::set_permissions(readonly_dir.path(), fs::Permissions::from_mode(0o555))
            .expect("Failed to set permission");
        // Test
        let mock_app = MockAppPathProvider {
            path: MockPath {
                path: readonly_dir.path().to_path_buf(),
            },
        };
        let mut bw = BinaryWriter::new();
        let id = Uuid::new_v4();
        write_project_header(&mut bw, &id, "Test Project", "Test Description");
        let result = write_project_to_file(&mock_app, &bw);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::IoError, err.kind);
    }
}
