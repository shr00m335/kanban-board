use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum KanbanErrorKind {
    IoError,
    TauriError,
    ProjectError,
}

#[derive(Debug)]
pub struct KanbanError {
    pub kind: KanbanErrorKind,
    pub message: String,
    pub source: Option<Box<dyn Error>>,
}

impl KanbanError {
    pub fn new<M: Into<String>>(kind: KanbanErrorKind, message: M) -> Self {
        KanbanError {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source<E: Error + 'static>(kind: KanbanErrorKind, source: E) -> Self {
        KanbanError {
            kind,
            message: source.to_string(),
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for KanbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl Error for KanbanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
