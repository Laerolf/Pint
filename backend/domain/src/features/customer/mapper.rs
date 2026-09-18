use chrono::Utc;

use entity::customers;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::{
    features::customer::{
        error::{CustomerErrorKind, CustomerMappingErrorKind},
        model::Customer,
    },
    shared::{Source, error::DomainError},
};

/// Represents a Mapper for [`Customers`][Customer]
pub struct CustomerMapper;

impl CustomerMapper {
    /// Maps a [Customer model][customers::Model] to an [Customer].
    pub fn to_domain_model(
        model: &customers::Model,
    ) -> Result<Customer, DomainError<CustomerErrorKind>> {
        let source = model
            .source
            .as_ref()
            .map(|source| {
                Source::from(source).map_err(|error| {
                    DomainError::from(CustomerErrorKind::Map(
                        CustomerMappingErrorKind::FromSquareToActiveModel,
                    ))
                    .with_cause(error)
                })
            })
            .transpose()?;

        Customer::restore(
            model.id,
            model.first_name.clone(),
            model.last_name.clone(),
            model.nickname.clone(),
            model.email_address.clone(),
            model.date_of_birth,
            source,
            model.source_id.clone(),
            model.created_at,
            model.last_updated_at,
        )
    }

    /// Maps a [Customer][Customer] to an [active model][customers::ActiveModel] to insert.
    pub fn to_insert_active_model(customer: &Customer) -> customers::ActiveModel {
        customers::ActiveModel {
            id: NotSet,
            first_name: Set(customer.first_name().cloned()),
            last_name: Set(customer.last_name().cloned()),
            nickname: Set(customer.nickname().cloned()),
            email_address: Set(customer.email_address().cloned()),
            date_of_birth: Set(customer.date_of_birth().cloned()),
            source: Set(customer.source().map(|source| source.to_string())),
            source_id: Set(customer.source_id().cloned()),
            created_at: Set(Utc::now().naive_utc()),
            last_updated_at: NotSet,
        }
    }

    /// Maps a [Customer][Customer] to an [active model][customers::ActiveModel] to update.
    pub fn to_update_active_model(
        customer: &Customer,
    ) -> Result<customers::ActiveModel, DomainError<CustomerErrorKind>> {
        Ok(customers::ActiveModel {
            id: Unchanged(customer.id().cloned().ok_or_else(|| {
                DomainError::from(CustomerErrorKind::Map(
                    CustomerMappingErrorKind::NotPersisted,
                ))
            })?),
            first_name: Set(customer.first_name().cloned()),
            last_name: Set(customer.last_name().cloned()),
            nickname: Set(customer.nickname().cloned()),
            email_address: Set(customer.email_address().cloned()),
            date_of_birth: Set(customer.date_of_birth().cloned()),
            source: Set(customer.source().map(|source| source.to_string())),
            source_id: Set(customer.source_id().cloned()),
            created_at: Set(Utc::now().naive_utc()),
            last_updated_at: NotSet,
        })
    }
}
