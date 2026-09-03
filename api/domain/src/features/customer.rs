use chrono::NaiveDate;

use crate::{
    features::error::{CustomerCreationErrorKind, CustomerErrorKind},
    shared::{Source, error::DomainError},
};

/// Represents a Customer.
pub struct Customer {
    /// The ID of this Customer.
    id: Option<String>,
    /// The first name of this Customer.
    first_name: Option<String>,
    /// The last name of this Customer.
    last_name: Option<String>,
    /// The nickname of this Customer.
    nickname: Option<String>,
    /// The email address of this Customer.
    email_address: Option<String>,
    /// The date of birth of this Customer.
    date_of_birth: Option<NaiveDate>,
    /// The origin of this Customer.
    source: Option<Source>,
    /// The origin ID of this Customer.
    source_id: Option<String>,
}

impl Customer {
    /// Creates a new [`Customer`].
    pub fn new(
        first_name: Option<String>,
        last_name: Option<String>,
        nickname: Option<String>,
        email_address: Option<String>,
        date_of_birth: Option<NaiveDate>,
        source: Option<Source>,
        source_id: Option<String>,
    ) -> Result<Self, DomainError<CustomerErrorKind>> {
        if source.is_some() && source_id.is_none() {
            return Err(DomainError::from(CustomerErrorKind::New(
                CustomerCreationErrorKind::InvalidSource,
            )));
        }

        Ok(Self {
            id: None,
            first_name,
            last_name,
            nickname,
            email_address,
            date_of_birth,
            source,
            source_id,
        })
    }
}
