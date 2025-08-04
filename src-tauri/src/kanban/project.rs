use crate::errors::kanban_error::{KanbanError, KanbanErrorKind};
use crate::file_system::binary_reader::BinaryReader;
use crate::file_system::binary_writer::BinaryWriter;
use crate::kanban::board;
use serde;
use std::fs;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Project {
    pub id: [u8; 16],
    pub name: String,
    pub description: String,
    pub boards: Vec<board::Board>,
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

pub fn create_project<P: AppPathProvider>(
    app: &P,
    name: &str,
    description: &str,
) -> Result<Project, KanbanError> {
    // Check project name and description
    if name.len() == 0 || description.len() == 0 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Empty Name or Description: The name and description of the project must not be empty",
        ));
    }
    if name.len() > 256 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Name too long: Project name must be between 1 and 256 characters",
        ));
    }
    let mut bw: BinaryWriter = BinaryWriter::new();
    let id: Uuid = Uuid::new_v4();
    write_project_header(&mut bw, &id, name, description);
    bw.write_byte(0x00); // Write initial board count
    write_project_to_file(app, &bw)?;
    let project = Project {
        id: id.as_bytes().clone(),
        name: name.to_string(),
        description: description.to_string(),
        boards: Vec::new(),
    };
    Ok(project)
}

fn read_project_info<P: AppPathProvider>(
    app: &P,
    project_id: &str,
) -> Result<Project, KanbanError> {
    if project_id.len() != 32 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Invalid project ID",
        ));
    }
    // Project path
    let project_path = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_box_source(KanbanErrorKind::TauriError, e))?
        .join(PROJECT_PATH)
        .join(project_id);
    let mut br = BinaryReader::read_from_file(&project_path)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    // Version
    let version: u8 = br.next_byte()?;
    if version != 0x00 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Project version not supported",
        ));
    }
    // Project ID
    let project_id: Vec<u8> = br.next_bytes(16)?;
    // Project Name
    let project_name: String = br.next_string(true)?;
    // Project Description
    let project_description: String = br.next_string(false)?;
    Ok(Project {
        id: <[u8; 16]>::try_from(project_id).map_err(|_| {
            KanbanError::new(
                KanbanErrorKind::ProjectError,
                "Invalid project ID length".to_string(),
            )
        })?,
        name: project_name,
        description: project_description,
        boards: Vec::new(),
    })
}

pub fn get_all_projects_info<P: AppPathProvider>(app: &P) -> Result<Vec<Project>, KanbanError> {
    let project_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_box_source(KanbanErrorKind::TauriError, e))?
        .join(PROJECT_PATH);
    let project_ids = fs::read_dir(project_dir)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<String>>();
    let projects = project_ids
        .iter()
        .map(|id| read_project_info(app, id))
        .filter_map(|p| p.ok())
        .collect::<Vec<Project>>();
    Ok(projects)
}

pub fn save_project<P: AppPathProvider>(
    app: &P,
    project: &Project,
) -> Result<Project, KanbanError> {
    let mut bw: BinaryWriter = BinaryWriter::new();
    write_project_header(
        &mut bw,
        &Uuid::from_bytes(project.id),
        &project.name,
        &project.description,
    );
    board::write_all_boards(&mut bw, &project.boards)?;
    write_project_to_file(app, &bw)?;
    Ok(project.clone())
}

pub fn read_project<P: AppPathProvider>(
    app: &P,
    project_id: &[u8],
) -> Result<Project, KanbanError> {
    // Check project id
    if project_id.len() != 16 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Invalid project ID",
        ));
    }
    // Get project path
    let file_name: String = project_id.iter().map(|b| format!("{:02X}", b)).collect();
    let project_path = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_box_source(KanbanErrorKind::TauriError, e))?
        .join(PROJECT_PATH)
        .join(file_name);
    // Read project file
    let mut br = BinaryReader::read_from_file(&project_path)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    // Version
    let version: u8 = br.next_byte()?;
    if version != 0x00 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Project version not supported",
        ));
    }
    // Project ID
    let project_id: Vec<u8> = br.next_bytes(16)?;
    // Project Name
    let name: String = br.next_string(true)?;
    // Project Description
    let description: String = br.next_string(false)?;
    // Boards
    let boards = board::read_all_boards(&mut br)?;
    Ok(Project {
        id: <[u8; 16]>::try_from(project_id).map_err(|_| {
            KanbanError::new(
                KanbanErrorKind::ProjectError,
                "Invalid project ID length".to_string(),
            )
        })?,
        name,
        description,
        boards,
    })
}

pub fn delete_project<P: AppPathProvider>(app: &P, project_id: &[u8]) -> Result<(), KanbanError> {
    // Check project id
    if project_id.len() != 16 {
        return Err(KanbanError::new(
            KanbanErrorKind::ProjectError,
            "Invalid project ID",
        ));
    }
    let file_name: String = project_id.iter().map(|b| format!("{:02X}", b)).collect();
    let project_path = app
        .path()
        .app_data_dir()
        .map_err(|e| KanbanError::from_box_source(KanbanErrorKind::TauriError, e))?
        .join(PROJECT_PATH)
        .join(file_name);
    if fs::exists(&project_path)
        .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?
    {
        fs::remove_file(&project_path)
            .map_err(|e| KanbanError::from_source(KanbanErrorKind::IoError, e))?;
    } else {
        return Err(KanbanError::new(
            KanbanErrorKind::IoError,
            "Project does not exists",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::kanban::boardList::BoardList;

    use super::*;
    use serial_test::serial;
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
    #[serial]
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

    #[test]
    #[serial]
    fn test_create_project() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test data
        let result = create_project(app, "Test Project", "Test Description");
        // Test result
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!("Test Project", project.name);
        assert_eq!("Test Description", project.description);
        assert_eq!(0, project.boards.len());
        let file_name: String = project.id.iter().map(|b| format!("{:02X}", b)).collect();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(file_name);
        assert!(fs::exists(&project_path).expect("Failed to check exists"));
        fs::remove_file(project_path).expect("Failed to remove file");
    }

    #[test]
    fn test_create_project_empty_name_and_description() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test data (empty name)
        let result = create_project(app, "", "Test Description");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, error.kind);
        assert_eq!(
            "Empty Name or Description: The name and description of the project must not be empty",
            error.message
        );
        // Test data (empty description)
        let result = create_project(app, "Test Project", "");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, error.kind);
        assert_eq!(
            "Empty Name or Description: The name and description of the project must not be empty",
            error.message
        );
    }

    #[test]
    fn test_create_project_name_too_long() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test data (empty name)
        let result = create_project(
            app,
            &(0..257).map(|_| "X").collect::<String>(),
            "Test Description",
        );
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, error.kind);
        assert_eq!(
            "Name too long: Project name must be between 1 and 256 characters",
            error.message
        );
    }

    #[test]
    fn test_create_project_write_to_file_error() {
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
        let result = create_project(&mock_app, "Test Project", "Test Description");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::TauriError, err.kind);
        assert_eq!("tauri path error", err.message);
    }

    #[test]
    #[serial]
    fn test_read_project_info() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        // Test data
        let test_project = create_project(app, "Test Project", "Test Description")
            .expect("Failed to create test project");
        let file_name: String = (&test_project.id)
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect();
        let result = read_project_info(app, &file_name);
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(test_project.name, project.name);
        assert_eq!(test_project.description, project.description);
        assert_eq!(test_project.id, project.id);
        assert_eq!(0, project.boards.len());
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(file_name);
        assert!(fs::exists(&project_path).expect("Failed to check exists"));
        fs::remove_file(project_path).expect("Failed to remove file");
    }

    #[test]
    fn test_read_project_info_invalid_project_id() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let project_id = "36c2747ba8e5431aa1f247f7b711d8101a";
        let result = read_project_info(app, project_id);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, err.kind);
        assert_eq!("Invalid project ID", err.message);
    }

    #[test]
    fn test_read_project_info_invalid_version() {
        // tauri env
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let mut bw = BinaryWriter::new();
        let id = Uuid::new_v4();
        write_project_header(&mut bw, &id, "Test Name", "Test Description");
        let bytes = bw.as_bytes();
        let mut bw = BinaryWriter::new();
        bw.write_byte(0x01);
        bw.write_bytes(&bytes[1..]);
        write_project_to_file(app, &bw).expect("Failed to create project");
        let file_name: String = id.as_bytes().iter().map(|b| format!("{:02X}", b)).collect();
        let result = read_project_info(app, &file_name);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::ProjectError, err.kind);
        assert_eq!("Project version not supported", err.message);
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(file_name);
        assert!(fs::exists(&project_path).expect("Failed to check exists"));
        fs::remove_file(project_path).expect("Failed to remove file");
    }

    #[test]
    #[serial]
    fn test_get_all_projects_info() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let project_1 = create_project(app, "Test Project 1", "Test Description 1")
            .expect("Failed to create test project");
        let project_2 = create_project(app, "Test Project 2", "Test Description 2")
            .expect("Failed to create test project");
        let project_3 = create_project(app, "Test Project 3", "Test Description 3")
            .expect("Failed to create test project");
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH);
        let file_path = project_path.join("invalid_id");
        fs::write(file_path, Vec::<u8>::new()).expect("Failed failed to create test project");
        let result = get_all_projects_info(app);
        assert!(result.is_ok());
        let projects = result.unwrap();
        assert_eq!(3, projects.len());
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
    fn test_get_all_projects_info_app_data_dir_error() {
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
        let result = get_all_projects_info(&mock_app);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::TauriError, err.kind);
    }

    #[test]
    #[serial]
    fn test_get_all_projects_info_dir_not_found() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH);
        if fs::exists(&project_path).expect("Unable to check whether path exists") {
            fs::remove_dir(&project_path).expect("Failed to remove dir");
        }
        let result = get_all_projects_info(app);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(KanbanErrorKind::IoError, err.kind);
    }

    #[test]
    #[serial]
    fn test_save_project() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let test_list_1 = BoardList {
            title: "Test Board 1".to_string(),
            color: [0xFF, 0xFF, 0xFF],
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let test_list_2 = BoardList {
            title: "Test Board 2".to_string(),
            color: [0x00, 0x00, 0x00],
            items: ["Item 1", "Item 2"].map(|s| s.to_string()).to_vec(),
        };
        let test_list_3 = BoardList {
            title: "Test Board 3".to_string(),
            color: [0xaa, 0xaa, 0xaa],
            items: ["Item 1"].map(|s| s.to_string()).to_vec(),
        };
        let test_board = board::Board {
            name: "Test Board".to_string(),
            lists: [test_list_1, test_list_2, test_list_3].to_vec(),
        };
        let test_project = Project {
            name: "Test Project 1".to_string(),
            description: "Description for Test Project 1".to_string(),
            boards: [test_board.clone(), test_board.clone()].to_vec(),
            id: Uuid::new_v4().into_bytes(),
        };
        let result = save_project(app, &test_project);
        assert!(result.is_ok());
        let file_name: String = (&test_project.id)
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(file_name);
        let bytes = fs::read(&project_path).expect("Failed to read file");
        let mut expected_bytes: Vec<u8> = Vec::new();
        expected_bytes.push(0x00);
        expected_bytes.extend_from_slice(&test_project.id);
        expected_bytes.extend_from_slice(&[
            0x0E, 0x54, 0x65, 0x73, 0x74, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
            0x31, 0x1E, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x20,
            0x66, 0x6F, 0x72, 0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65,
            0x63, 0x74, 0x20, 0x31,
        ]);
        expected_bytes.extend_from_slice(&[
            2, 10, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x03, 0x0C, 0x54,
            0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x31, 0xFF, 0xFF, 0xFF,
            0x03, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20,
            0x32, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x33, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20,
            0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x32, 0x00, 0x00, 0x00, 0x02, 0x06, 0x49, 0x74,
            0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x0C, 0x54, 0x65,
            0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x33, 0xaa, 0xaa, 0xaa, 0x01,
            0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 10, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F,
            0x61, 0x72, 0x64, 0x03, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72,
            0x64, 0x20, 0x31, 0xFF, 0xFF, 0xFF, 0x03, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31,
            0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x32, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x33,
            0x0C, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64, 0x20, 0x32, 0x00,
            0x00, 0x00, 0x02, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31, 0x06, 0x49, 0x74, 0x65,
            0x6D, 0x20, 0x32, 0x0C, 0x54, 0x65, 0x73, 0x74, 0x20, 0x42, 0x6F, 0x61, 0x72, 0x64,
            0x20, 0x33, 0xaa, 0xaa, 0xaa, 0x01, 0x06, 0x49, 0x74, 0x65, 0x6D, 0x20, 0x31,
        ]);
        assert_eq!(expected_bytes, bytes);
        if fs::exists(&project_path).expect("Failed to check whether file exists") {
            fs::remove_file(&project_path).expect("Failed to remove file");
        }
    }

    #[test]
    #[serial]
    fn test_read_project() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let test_list_1 = BoardList {
            title: "Test Board 1".to_string(),
            color: [0xFF, 0xFF, 0xFF],
            items: ["Item 1", "Item 2", "Item 3"]
                .map(|s| s.to_string())
                .to_vec(),
        };
        let test_list_2 = BoardList {
            title: "Test Board 2".to_string(),
            color: [0x00, 0x00, 0x00],
            items: ["Item 1", "Item 2"].map(|s| s.to_string()).to_vec(),
        };
        let test_list_3 = BoardList {
            title: "Test Board 3".to_string(),
            color: [0xaa, 0xaa, 0xaa],
            items: ["Item 1"].map(|s| s.to_string()).to_vec(),
        };
        let test_board = board::Board {
            name: "Test Board".to_string(),
            lists: [test_list_1, test_list_2, test_list_3].to_vec(),
        };
        let test_project = Project {
            name: "Test Project 1".to_string(),
            description: "Description for Test Project 1".to_string(),
            boards: [test_board.clone(), test_board.clone()].to_vec(),
            id: Uuid::new_v4().into_bytes(),
        };
        let expected_project = save_project(app, &test_project).expect("Failed to save project");
        let project = read_project(app, &expected_project.id);
        assert!(project.is_ok());
        assert_eq!(expected_project, project.unwrap());
        let file_name: String = (&expected_project.id)
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(file_name);
        if fs::exists(&project_path).expect("Failed to check whether file exists") {
            fs::remove_file(&project_path).expect("Failed to remove file");
        }
    }

    #[test]
    fn test_delete_project() {
        let mock = tauri::test::mock_app();
        let app = mock.app_handle();
        let project = create_project(app, "Test Project", "Test Description").unwrap();
        let result = delete_project(app, &project.id);
        assert!(result.is_ok());
        let project_id: String = (&project.id).iter().map(|b| format!("{:02X}", b)).collect();
        let project_path = Manager::path(app)
            .app_data_dir()
            .expect("Failed to get data path")
            .join(PROJECT_PATH)
            .join(&project_id);
        assert!(!fs::exists(project_path).unwrap());
    }
}
