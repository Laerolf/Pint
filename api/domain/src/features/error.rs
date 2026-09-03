use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CustomerCreationErrorKind {
    InvalidSource,
}

/// Represents a Customer domain error.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CustomerErrorKind {
    /// Failed to create a new Customer.
    New(CustomerCreationErrorKind),
}

impl DomainErrorKind for CustomerErrorKind {
    fn code(&self) -> String {
        match self {
            CustomerErrorKind::New(_error) => "error.domain.customer.new.invalidSource".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            CustomerErrorKind::New(_error) => "The provided Source is invalid.".to_string(),
        }
    }
}

impl Display for CustomerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}
