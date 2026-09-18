use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenueCreationErrorKind {
    /// The provided Source is invalid.
    InvalidSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenueRestoreErrorKind {
    /// The provided Source is invalid.
    InvalidSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenueMappingErrorKind {
    /// Failed to map a Square Venue to an active model.
    FromSquareToActiveModel,
    /// The Venue has not been persisted yet.
    NotPersisted,
}

/// Represents a Customer domain error.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenueErrorKind {
    /// Failed to create a new Venue.
    New(VenueCreationErrorKind),
    /// Failed to restore a Venue.
    Restore(VenueRestoreErrorKind),
    /// Failed to find a Venue with the provided ID.
    FindById,
    /// Failed to find a Venue with the provided source and source ID.
    FindBySourceAndSourceId,
    /// Failed to map a Venue.
    Map(VenueMappingErrorKind),
    /// Failed to insert a new Venue.
    Insert,
}

impl DomainErrorKind for VenueErrorKind {
    fn code(&self) -> String {
        match self {
            VenueErrorKind::New(error) => match error {
                VenueCreationErrorKind::InvalidSource => {
                    "error.domain.venue.new.invalidSource".to_string()
                }
            },
            VenueErrorKind::Restore(error) => match error {
                VenueRestoreErrorKind::InvalidSource => {
                    "error.domain.venue.restore.invalidSource".to_string()
                }
            },
            VenueErrorKind::Map(error) => match error {
                VenueMappingErrorKind::FromSquareToActiveModel => {
                    "error.domain.venue.map.fromSquareToActiveModel".to_string()
                }
                VenueMappingErrorKind::NotPersisted => {
                    "error.domain.venue.map.notPersisted".to_string()
                }
            },
            VenueErrorKind::FindById => "error.domain.venue.findById".to_string(),
            VenueErrorKind::FindBySourceAndSourceId => {
                "error.domain.venue.findBySourceAndSourceId".to_string()
            }
            Self::Insert => "error.domain.venue.insert".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            VenueErrorKind::New(error) => match error {
                VenueCreationErrorKind::InvalidSource => {
                    "The provided Source is invalid.".to_string()
                }
            },
            VenueErrorKind::Restore(error) => match error {
                VenueRestoreErrorKind::InvalidSource => {
                    "The provided Source is invalid.".to_string()
                }
            },
            VenueErrorKind::Map(error) => match error {
                VenueMappingErrorKind::FromSquareToActiveModel => {
                    "Failed to map a Square Venue to an active model.".to_string()
                }
                VenueMappingErrorKind::NotPersisted => {
                    "The Venue has not been persisted yet.".to_string()
                }
            },
            VenueErrorKind::FindById => "Failed to find a Venue with the provided ID.".to_string(),
            VenueErrorKind::FindBySourceAndSourceId => {
                "Failed to find a Venue with the provided source and source ID.".to_string()
            }
            VenueErrorKind::Insert => "Failed to insert a new Venue.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl Display for VenueErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
