use chrono::{NaiveDate, Utc};

use domain::{
    features::customer::error::{CustomerErrorKind, CustomerMappingErrorKind},
    shared::{Source, error::DomainError},
};
use entity::customers;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::customer::model::Customer;

/// Represents a Mapper for [`Square Customers`][Customer]
pub struct SquareCustomerMapper;

impl SquareCustomerMapper {
    /// Maps a [Square Customer][Customer] to an [active model][customers::ActiveModel] to insert.
    pub fn to_insert_active_model(
        customer: &Customer,
    ) -> Result<customers::ActiveModel, DomainError<CustomerErrorKind>> {
        let date_of_birth = customer
            .birthday
            .as_ref()
            .map(|birthday| {
                NaiveDate::parse_from_str(birthday, "%Y-%m-%d").map_err(|error| {
                    DomainError::from(CustomerErrorKind::Map(
                        CustomerMappingErrorKind::FromSquareToActiveModel,
                    ))
                    .with_cause(error)
                })
            })
            .transpose()?;

        Ok(customers::ActiveModel {
            id: NotSet,
            first_name: Set(customer.given_name.clone()),
            last_name: Set(customer.family_name.clone()),
            nickname: Set(customer.nickname.clone()),
            email_address: Set(customer.email_address.clone()),
            date_of_birth: Set(date_of_birth),
            source: Set(Some(Source::Square.to_string())),
            source_id: Set(Some(customer.id.clone())),
            created_at: Set(Utc::now().naive_utc()),
            last_updated_at: NotSet,
        })
    }
}
