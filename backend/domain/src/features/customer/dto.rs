use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::customer::{
        domain::Customer,
        error::{CustomerErrorKind, CustomerMappingErrorKind},
    },
    shared::{
        error::DomainError,
        openapi::{example_date_string, example_id, example_source},
    },
};

/// Represents a Customer DTO.
#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct CustomerDto {
    /// The ID of this Customer.
    #[schema(example = example_id)]
    pub id: i32,
    /// The first name of this Customer.
    #[schema(example = "Henry")]
    pub first_name: Option<String>,
    /// The last name of this Customer.
    #[schema(example = "Jekyll")]
    pub last_name: Option<String>,
    /// The nickname of this Customer.
    #[schema(example = "The Good Doctor")]
    pub nickname: Option<String>,
    /// The email address of this Customer.
    #[schema(example = "henry.jekyll@good.doctor")]
    pub email_address: Option<String>,
    /// The date of birth of this Customer.
    #[schema(example = "1948-12-03")]
    pub date_of_birth: Option<String>,
    /// The origin of this Customer.
    #[schema(example = example_source)]
    pub source: Option<String>,
    /// The origin ID of this Customer.
    #[schema(example = example_id)]
    pub source_id: Option<String>,
    /// The date of the creation date of this Customer.
    #[schema(example = example_date_string)]
    pub created_at: Option<String>,
    /// The date of the last update of this Customer.
    pub last_updated_at: Option<String>,
}

impl CustomerDto {
    /// Creates a [`CustomerDto`] based on a [Customer]
    pub fn from(customer: &Customer) -> Result<Self, DomainError<CustomerErrorKind>> {
        let id = customer.id().cloned().ok_or_else(|| {
            DomainError::from(CustomerErrorKind::Map(
                CustomerMappingErrorKind::NotPersisted,
            ))
        })?;

        Ok(Self {
            id,
            first_name: customer.first_name().cloned(),
            last_name: customer.last_name().cloned(),
            nickname: customer.nickname().cloned(),
            email_address: customer.email_address().cloned(),
            date_of_birth: customer
                .date_of_birth()
                .map(|date_of_birth| date_of_birth.to_string()),
            source: customer.source().map(|source| source.to_string()),
            source_id: customer.source_id().cloned(),
            created_at: customer
                .created_at()
                .map(|created_at| created_at.and_utc().to_rfc3339()),
            last_updated_at: customer
                .last_updated_at()
                .map(|last_updated_at| last_updated_at.and_utc().to_rfc3339()),
        })
    }
}

#[cfg(test)]
mod tests {
    mod from {
        use chrono::{NaiveDate, Utc};

        use crate::{
            features::customer::{domain::Customer, dto::CustomerDto},
            shared::Source,
        };

        #[test]
        fn test_a_customer_dto_can_be_created_with_a_persisted_customer() {
            // Given
            let id = 666666;
            let first_name = Some(String::from("John"));
            let last_name = Some(String::from("Osbourne"));
            let nickname = Some(String::from("Ozzy"));
            let email_address = Some(String::from("ozzy@in.heaven"));
            let date_of_birth = Some(
                NaiveDate::parse_from_str("1948-12-03", "%Y-%m-%d")
                    .expect("Expected the test date of birth to resolve successfully."),
            );
            let source = Some(Source::Square);
            let source_id = Some(String::from("666666"));
            let created_at = Utc::now().naive_utc();

            let customer = Customer::restore(
                id,
                first_name,
                last_name,
                nickname,
                email_address,
                date_of_birth,
                source,
                source_id,
                created_at,
                None,
            )
            .expect("Expected the test Customer user to be restored.");

            // When
            let dto = CustomerDto::from(&customer)
                .expect("Expected the test Customer DTO to resolve successfully.");

            // Then
            assert_eq!(Some(&dto.id), customer.id());
            assert_eq!(dto.first_name.as_ref(), customer.first_name());
            assert_eq!(dto.last_name.as_ref(), customer.last_name());
            assert_eq!(dto.nickname.as_ref(), customer.nickname());
            assert_eq!(dto.email_address.as_ref(), customer.email_address());
            assert_eq!(
                dto.date_of_birth,
                customer
                    .date_of_birth()
                    .map(|date_of_birth| date_of_birth.to_string())
            );
            assert_eq!(
                dto.source,
                customer.source().map(|source| source.to_string())
            );
            assert_eq!(dto.source_id.as_ref(), customer.source_id());
            assert_eq!(
                dto.created_at,
                customer
                    .created_at()
                    .map(|created_at| created_at.and_utc().to_rfc3339())
            );
            assert_eq!(
                dto.last_updated_at,
                customer
                    .last_updated_at()
                    .map(|last_updated_at| last_updated_at.and_utc().to_rfc3339())
            );
        }
    }
}
