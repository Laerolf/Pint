use std::{collections::HashMap, fmt::Display};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::shared::error::{DomainError, DomainErrorKind};
use serde::Serialize;
use utoipa::ToSchema;

/// Represents an API [Error][`std::error::Error`].
#[derive(Debug)]
pub enum StartupError {
    MissingHost,
    MissingPort,
    MissingDbUrl,
    InvalidDbUrl,
    DatabaseMigration,
    CreateSquareApiClient,
    RunSquareApiBackfill,
    Serve,
}

impl StartupError {
    /// Gets the locale code of this [`ApiError`].
    fn code(&self) -> &'static str {
        match self {
            Self::MissingHost => "error.api.missingHost",
            Self::MissingPort => "error.api.missingPort",
            Self::MissingDbUrl => "error.api.missingDbUrl",
            Self::InvalidDbUrl => "error.api.invalidDbUrl",
            Self::DatabaseMigration => "error.api.databaseMigration",
            Self::CreateSquareApiClient => "error.api.createSquareApiClient",
            Self::RunSquareApiBackfill => "error.api.runSquareApiBackfill",
            Self::Serve => "error.api.serve",
        }
    }

    /// Gets the locale code of this [`ApiError`].
    fn message(&self) -> &'static str {
        match self {
            Self::MissingHost => "A host is required.",
            Self::MissingPort => "A port is required.",
            Self::MissingDbUrl => "A database connection url is required.",
            Self::InvalidDbUrl => "Failed to create a database connection.",
            Self::DatabaseMigration => "Failed to run the database migration.",
            Self::CreateSquareApiClient => "Failed to create a Square API client.",
            Self::RunSquareApiBackfill => "Failed to run the Square API backfill.",
            Self::Serve => "Failed to serve the API routes.",
        }
    }
}

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message(), self.code())
    }
}

#[derive(Serialize, ToSchema)]
pub struct AppError {
    /// The message of the error.
    #[schema(example = "Ozzy is in heaven.")]
    pub message: String,
    /// The message code of the error.
    #[schema(example = "error.ozzy.in_heaven")]
    pub code: String,
    /// The context of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = json!({"since": "2025-07-22T00:00:00Z"}))]
    pub context: Option<HashMap<String, String>>,

    #[serde(skip)]
    status_code: StatusCode,
}

impl<K> From<DomainError<K>> for AppError
where
    K: DomainErrorKind,
{
    fn from(value: DomainError<K>) -> Self {
        Self {
            code: value.kind().code(),
            message: value.kind().message(),
            context: value.context().clone(),
            status_code: value.kind().http_status(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}
