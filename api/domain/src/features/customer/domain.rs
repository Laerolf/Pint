use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    features::customer::error::{CustomerCreationErrorKind, CustomerErrorKind},
    shared::{Source, error::DomainError},
};

/// Represents a Customer.
#[derive(Debug, Clone)]
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
    /// The date of the creation date of this Customer.
    created_at: Option<NaiveDateTime>,
    /// The date of the last update of this Customer.
    last_updated_at: Option<NaiveDateTime>,
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
            created_at: None,
            last_updated_at: None,
        })
    }

    /// Restores a [`Customer`].
    pub fn restore(
        id: String,
        first_name: Option<String>,
        last_name: Option<String>,
        nickname: Option<String>,
        email_address: Option<String>,
        date_of_birth: Option<NaiveDate>,
        source: Option<Source>,
        source_id: Option<String>,
        created_at: NaiveDateTime,
        last_updated_at: Option<NaiveDateTime>,
    ) -> Result<Self, DomainError<CustomerErrorKind>> {
        if source.is_some() && source_id.is_none() {
            return Err(DomainError::from(CustomerErrorKind::New(
                CustomerCreationErrorKind::InvalidSource,
            )));
        }

        Ok(Self {
            id: Some(id),
            first_name,
            last_name,
            nickname,
            email_address,
            date_of_birth,
            source,
            source_id,
            created_at: Some(created_at),
            last_updated_at,
        })
    }

    /// Returns the ID of this [`Customer`].
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Returns the first name of this [`Customer`].
    pub fn first_name(&self) -> Option<&String> {
        self.first_name.as_ref()
    }

    /// Returns the last name of this [`Customer`].
    pub fn last_name(&self) -> Option<&String> {
        self.last_name.as_ref()
    }

    /// Returns the nickname of this [`Customer`].
    pub fn nickname(&self) -> Option<&String> {
        self.nickname.as_ref()
    }

    /// Returns the email address of this [`Customer`].
    pub fn email_address(&self) -> Option<&String> {
        self.email_address.as_ref()
    }

    /// Returns the date of birth of this [`Customer`].
    pub fn date_of_birth(&self) -> Option<&NaiveDate> {
        self.date_of_birth.as_ref()
    }

    /// Returns the [Source] of this [`Customer`].
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Returns the Source ID of this [`Customer`].
    pub fn source_id(&self) -> Option<&String> {
        self.source_id.as_ref()
    }

    /// Returns the creation date of this [`Customer`].
    pub fn created_at(&self) -> Option<&NaiveDateTime> {
        self.created_at.as_ref()
    }

    /// Returns the last update date of this [`Customer`].
    pub fn last_updated_at(&self) -> Option<&NaiveDateTime> {
        self.last_updated_at.as_ref()
    }
}

#[cfg(test)]
mod tests {
    mod new {
        use chrono::NaiveDate;

        use crate::{
            features::customer::{
                domain::Customer,
                error::{CustomerCreationErrorKind, CustomerErrorKind},
            },
            shared::Source,
        };

        #[test]
        fn test_it_should_be_possible_to_create_a_customer() {
            // Given
            let expected_first_name = Some(String::from("John"));
            let expected_last_name = Some(String::from("Osbourne"));
            let expected_nickname = Some(String::from("Ozzy"));
            let expected_email_address = Some(String::from("ozzy@in.heaven"));
            let expected_date_of_birth = Some(
                NaiveDate::parse_from_str("1948-12-03", "%Y-%m-%d")
                    .expect("Expected the test date of birth to resolve successfully."),
            );
            let expected_source = Some(Source::Square);
            let expected_source_id = Some(String::from("666666"));

            // When
            let customer = Customer::new(
                expected_first_name.clone(),
                expected_last_name.clone(),
                expected_nickname.clone(),
                expected_email_address.clone(),
                expected_date_of_birth.clone(),
                expected_source.clone(),
                expected_source_id.clone(),
            )
            .expect("Expected the test Customer to resolve successfully.");

            // Then
            assert!(customer.id().is_none());
            assert_eq!(customer.first_name(), expected_first_name.as_ref());
            assert_eq!(customer.last_name(), expected_last_name.as_ref());
            assert_eq!(customer.nickname(), expected_nickname.as_ref());
            assert_eq!(customer.email_address(), expected_email_address.as_ref());
            assert_eq!(customer.date_of_birth(), expected_date_of_birth.as_ref());
            assert_eq!(customer.source(), expected_source.as_ref());
            assert_eq!(customer.source_id(), expected_source_id.as_ref());
            assert!(customer.created_at().is_none());
            assert!(customer.last_updated_at().is_none());
        }

        #[test]
        fn test_a_customer_should_have_a_source_id_when_it_has_a_source() {
            // Given
            let expected_first_name = Some(String::from("John"));
            let expected_last_name = Some(String::from("Osbourne"));
            let expected_nickname = Some(String::from("Ozzy"));
            let expected_email_address = Some(String::from("ozzy@in.heaven"));
            let expected_date_of_birth = Some(
                NaiveDate::parse_from_str("1948-12-03", "%Y-%m-%d")
                    .expect("Expected the test date of birth to resolve successfully."),
            );
            let expected_source = Some(Source::Square);

            let expected_error_kind =
                CustomerErrorKind::New(CustomerCreationErrorKind::InvalidSource);

            // When
            let customer = Customer::new(
                expected_first_name,
                expected_last_name,
                expected_nickname,
                expected_email_address,
                expected_date_of_birth,
                expected_source,
                None,
            );

            // Then
            let error =
                customer.expect_err("Expected customer creation to fail without a source_id");
            assert_eq!(error.kind(), &expected_error_kind);
        }
    }

    mod restore {
        use chrono::{NaiveDate, Utc};

        use crate::{
            features::customer::{
                domain::Customer,
                error::{CustomerCreationErrorKind, CustomerErrorKind},
            },
            shared::Source,
        };

        #[test]
        fn test_it_should_be_possible_to_restore_a_customer() {
            // Given
            let expected_id = String::from("555555");
            let expected_first_name = Some(String::from("John"));
            let expected_last_name = Some(String::from("Osbourne"));
            let expected_nickname = Some(String::from("Ozzy"));
            let expected_email_address = Some(String::from("ozzy@in.heaven"));
            let expected_date_of_birth = Some(
                NaiveDate::parse_from_str("1948-12-03", "%Y-%m-%d")
                    .expect("Expected the test date of birth to resolve successfully."),
            );
            let expected_source = Some(Source::Square);
            let expected_source_id = Some(String::from("666666"));
            let expected_created_at = Utc::now().naive_utc();
            let expected_last_updated_at = Some(Utc::now().naive_utc());

            // When
            let customer = Customer::restore(
                expected_id.clone(),
                expected_first_name.clone(),
                expected_last_name.clone(),
                expected_nickname.clone(),
                expected_email_address.clone(),
                expected_date_of_birth.clone(),
                expected_source.clone(),
                expected_source_id.clone(),
                expected_created_at.clone(),
                expected_last_updated_at.clone(),
            )
            .expect("Expected the test Customer to resolve successfully.");

            // Then
            assert_eq!(customer.id(), Some(expected_id).as_ref());
            assert_eq!(customer.first_name(), expected_first_name.as_ref());
            assert_eq!(customer.last_name(), expected_last_name.as_ref());
            assert_eq!(customer.nickname(), expected_nickname.as_ref());
            assert_eq!(customer.email_address(), expected_email_address.as_ref());
            assert_eq!(customer.date_of_birth(), expected_date_of_birth.as_ref());
            assert_eq!(customer.source(), expected_source.as_ref());
            assert_eq!(customer.source_id(), expected_source_id.as_ref());
            assert_eq!(customer.created_at(), Some(expected_created_at).as_ref());
            assert_eq!(
                customer.last_updated_at(),
                expected_last_updated_at.as_ref()
            );
        }

        #[test]
        fn test_a_customer_should_have_a_source_id_when_it_has_a_source() {
            // Given
            let expected_id = String::from("555555");
            let expected_first_name = Some(String::from("John"));
            let expected_last_name = Some(String::from("Osbourne"));
            let expected_nickname = Some(String::from("Ozzy"));
            let expected_email_address = Some(String::from("ozzy@in.heaven"));
            let expected_date_of_birth = Some(
                NaiveDate::parse_from_str("1948-12-03", "%Y-%m-%d")
                    .expect("Expected the test date of birth to resolve successfully."),
            );
            let expected_source = Some(Source::Square);
            let expected_created_at = Utc::now().naive_utc();
            let expected_last_updated_at = Some(Utc::now().naive_utc());

            let expected_error_kind =
                CustomerErrorKind::New(CustomerCreationErrorKind::InvalidSource);

            // When
            let customer = Customer::restore(
                expected_id,
                expected_first_name,
                expected_last_name,
                expected_nickname,
                expected_email_address,
                expected_date_of_birth,
                expected_source,
                None,
                expected_created_at,
                expected_last_updated_at,
            );

            // Then
            let error =
                customer.expect_err("Expected customer restoration to fail without a source_id");
            assert_eq!(error.kind(), &expected_error_kind);
        }
    }
}
