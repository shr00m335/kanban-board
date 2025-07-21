use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum KanbanErrorKind {
    IoError,
    TauriError,
    ProjectError,
    TextError,
}

#[derive(Debug)]
pub struct KanbanError {
    pub kind: KanbanErrorKind,
    pub message: String,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl KanbanError {
    pub fn new<M: Into<String>>(kind: KanbanErrorKind, message: M) -> Self {
        KanbanError {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source<E: Error + 'static + Send + Sync>(kind: KanbanErrorKind, source: E) -> Self {
        KanbanError {
            kind,
            message: source.to_string(),
            source: Some(Box::new(source)),
        }
    }

    pub fn from_box_source(kind: KanbanErrorKind, source: Box<dyn Error + Send + Sync>) -> Self {
        KanbanError {
            kind,
            message: source.to_string(),
            source: Some(source),
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
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let error = KanbanError::new(KanbanErrorKind::ProjectError, "Project Test Error");
        assert_eq!(KanbanErrorKind::ProjectError, error.kind);
        assert_eq!("Project Test Error", error.message);
        assert!(error.source.is_none());
    }

    #[test]
    fn test_from_source() {
        let source_error = std::io::Error::new(std::io::ErrorKind::NotFound, "Not Found Error");
        let error = KanbanError::from_source(KanbanErrorKind::IoError, source_error);
        assert_eq!(KanbanErrorKind::IoError, error.kind);
        assert_eq!("Not Found Error", &error.message);
        assert!(error.source.is_some());
        let inner = error.source.unwrap();
        let io_error = inner
            .downcast_ref::<std::io::Error>()
            .expect("Should be std::io::Error");
        assert_eq!(std::io::ErrorKind::NotFound, io_error.kind());
    }

    #[test]
    fn test_display_fmt() {
        let error = KanbanError::new(KanbanErrorKind::ProjectError, "Project Error Test");
        let formatted = format!("{}", error);
        assert_eq!("[ProjectError] Project Error Test", &formatted);
    }

    #[test]
    fn test_source_function() {
        let source_error = std::io::Error::new(std::io::ErrorKind::Other, "inner test error");
        let error = KanbanError::from_source(KanbanErrorKind::IoError, source_error);
        let source = error.source();
        assert!(source.is_some());
        let source = source.unwrap();
        assert_eq!("inner test error", &source.to_string());
    }
}
