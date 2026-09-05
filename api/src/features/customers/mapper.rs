use chrono::{NaiveDate, Utc};
use domain::{
    features::customer::error::{CustomerErrorKind, CustomerMappingErrorKind},
    shared::{Source, error::DomainError},
};
use entity::customers;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use square_api_client::features::customer::model::Customer;

/// Represents a Mapper for [`Square Customers`][Customer]
pub struct SquareCustomerMapper;

impl SquareCustomerMapper {
    /// Maps a [Square Customer][Customer] to an [active model][customers::ActiveModel] to insert.
    pub fn to_insert_active_model(
        customer: Customer,
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
            first_name: Set(customer.given_name),
            last_name: Set(customer.family_name),
            nickname: Set(customer.nickname),
            email_address: Set(customer.email_address),
            date_of_birth: Set(date_of_birth),
            source: Set(Some(Source::Square.to_string())),
            source_id: Set(Some(customer.id)),
            created_at: Set(Utc::now().date_naive()),
            last_updated_at: NotSet,
        })
    }

    /// Maps a [Square Customer][Customer] to an [active model][customers::ActiveModel] to update.
    pub fn to_update_active_model(
        customer: Customer,
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
            first_name: Set(customer.given_name),
            last_name: Set(customer.family_name),
            nickname: Set(customer.nickname),
            email_address: Set(customer.email_address),
            date_of_birth: Set(date_of_birth),
            source: Unchanged(Some(Source::Square.to_string())),
            source_id: Unchanged(Some(customer.id)),
            created_at: NotSet,
            last_updated_at: Set(Some(Utc::now().date_naive())),
        })
    }
}
