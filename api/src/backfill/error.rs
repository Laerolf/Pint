use axum::http::StatusCode;
use std::fmt::Display;

use domain::shared::error::DomainErrorKind;

/// Represents a Customer domain error.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackfillErrorKind {
    /// Failed to run the backfill.
    Run,
}

impl DomainErrorKind for BackfillErrorKind {
    fn code(&self) -> String {
        match self {
            BackfillErrorKind::Run => "error.backfill.run".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            BackfillErrorKind::Run => "Failed to run the backfill.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl Display for BackfillErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

#[cfg(test)]
mod tests {
    use crate::backfill::error::BackfillErrorKind;

    #[test]
    fn test_string_format() {
        // When + then
        assert_eq!(
            BackfillErrorKind::Run.to_string(),
            "[error.backfill.run] Failed to run the backfill."
        );
    }
}
