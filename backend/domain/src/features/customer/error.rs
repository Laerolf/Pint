use std::fmt::Display;

use axum::http::StatusCode;

use crate::shared::error::DomainErrorKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CustomerCreationErrorKind {
    /// The provided Source is invalid.
    InvalidSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CustomerRestoreErrorKind {
    /// The provided Source is invalid.
    InvalidSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CustomerMappingErrorKind {
    /// Failed to map a Square Customer to an active model.
    FromSquareToActiveModel,
    /// The Customer has not been persisted yet.
    NotPersisted,
}

/// Represents a Customer domain error.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CustomerErrorKind {
    /// Failed to create a new Customer.
    New(CustomerCreationErrorKind),
    /// Failed to restore a Customer.
    Restore(CustomerRestoreErrorKind),
    /// Failed to find a Customer with the provided ID.
    FindById,
    /// Failed to get all Customers.
    GetAll,
    /// Failed to get all Customers for the provided Source.
    GetAllBySource,
    /// Failed to insert many Customers.
    InsertMany,
    /// Failed to update a Customers.
    Update,
    /// Failed to map a Customer.
    Map(CustomerMappingErrorKind),
}

impl DomainErrorKind for CustomerErrorKind {
    fn code(&self) -> String {
        match self {
            CustomerErrorKind::New(error) => match error {
                CustomerCreationErrorKind::InvalidSource => {
                    "error.domain.customer.new.invalidSource".to_string()
                }
            },
            CustomerErrorKind::Restore(error) => match error {
                CustomerRestoreErrorKind::InvalidSource => {
                    "error.domain.customer.restore.invalidSource".to_string()
                }
            },
            CustomerErrorKind::FindById => "error.domain.customer.findById".to_string(),
            CustomerErrorKind::GetAll => "error.domain.customer.getAll".to_string(),
            CustomerErrorKind::GetAllBySource => "error.domain.customer.getAllBySource".to_string(),
            CustomerErrorKind::InsertMany => "error.domain.customer.insertMany".to_string(),
            CustomerErrorKind::Update => "error.domain.customer.update".to_string(),
            CustomerErrorKind::Map(error) => match error {
                CustomerMappingErrorKind::FromSquareToActiveModel => {
                    "error.domain.customer.map.fromSquareToActiveModel".to_string()
                }
                CustomerMappingErrorKind::NotPersisted => {
                    "error.domain.customer.map.notPersisted".to_string()
                }
            },
        }
    }

    fn message(&self) -> String {
        match self {
            CustomerErrorKind::New(error) => match error {
                CustomerCreationErrorKind::InvalidSource => {
                    "The provided Source is invalid.".to_string()
                }
            },
            CustomerErrorKind::Restore(error) => match error {
                CustomerRestoreErrorKind::InvalidSource => {
                    "The provided Source is invalid.".to_string()
                }
            },
            CustomerErrorKind::FindById => {
                "Failed to find a Customer with the provided ID.".to_string()
            }
            CustomerErrorKind::GetAll => "Failed to get all Customers.".to_string(),
            CustomerErrorKind::GetAllBySource => {
                "Failed to get all Customers for the provided Source.".to_string()
            }
            CustomerErrorKind::InsertMany => "Failed to insert many Customers.".to_string(),
            CustomerErrorKind::Update => "Failed to update a Customer.".to_string(),
            CustomerErrorKind::Map(error) => match error {
                CustomerMappingErrorKind::FromSquareToActiveModel => {
                    "Failed to map a Square Customer to an active model.".to_string()
                }
                CustomerMappingErrorKind::NotPersisted => {
                    "The Customer has not been persisted yet.".to_string()
                }
            },
        }
    }

    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl Display for CustomerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
