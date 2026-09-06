use std::fmt::{Display, Formatter};

use axum::http::StatusCode;

use crate::shared::error::{DomainError, DomainErrorKind};

pub mod error;
pub mod openapi;

/// Represents the source of the domain element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Source {
    /// Square is the origin of this element.
    Square,
}

impl Source {
    /// Returns the matching [Source] for the provided value.
    pub fn from(value: &String) -> Result<Self, DomainError<SourceErrorKind>> {
        if value == &Source::Square.to_string() {
            Ok(Source::Square)
        } else {
            Err(DomainError::from(SourceErrorKind::From))
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Source::Square => write!(f, "Square"),
        }
    }
}

/// Represents a Source domain error.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceErrorKind {
    /// Failed to find a matching Source.
    From,
}

impl DomainErrorKind for SourceErrorKind {
    fn code(&self) -> String {
        match self {
            SourceErrorKind::From => "error.domain.source.from".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            SourceErrorKind::From => "Failed to find a matching Source.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl Display for SourceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::Source;

    #[test]
    fn test_string_format() {
        // When + then
        assert_eq!(Source::Square.to_string(), "Square");
    }
}
