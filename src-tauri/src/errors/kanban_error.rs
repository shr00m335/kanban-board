use std::{error::Error, fmt};

#[derive(Debug)]
pub enum KanbanError {
    IoError(std::io::Error),
    ProjectError(String),
}

impl fmt::Display for KanbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KanbanError::IoError(err) => write!(f, "IO Error: {}", err),
            KanbanError::ProjectError(msg) => write!(f, "Project Error: {}", msg),
        }
    }
}

impl Error for KanbanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            KanbanError::IoError(err) => Some(err),
            _ => None,
        }
    }
}
