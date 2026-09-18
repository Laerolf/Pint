use chrono::NaiveDateTime;

use crate::{
    features::venue::error::{VenueCreationErrorKind, VenueErrorKind, VenueRestoreErrorKind},
    shared::{error::DomainError, Source},
};

/// Represents a Venue.
#[derive(Debug, Clone)]
pub struct Venue {
    /// The ID of this Venue.
    id: Option<i32>,
    /// The name of this Venue.
    name: Option<String>,
    /// The country code of this Venue.
    country: Option<String>,
    /// The language code of this Venue.
    language_code: Option<String>,
    /// The timezone of this Venue.
    timezone: Option<String>,
    /// The currency of this Venue.
    currency: Option<String>,
    /// The origin of this Venue.
    source: Option<Source>,
    /// The origin ID of this Venue.
    source_id: Option<String>,
    /// The date of the creation date of this Venue.
    created_at: Option<NaiveDateTime>,
    /// The date of the last update of this Venue.
    last_updated_at: Option<NaiveDateTime>,
}

impl Venue {
    /// Creates a new [`Venue`].
    pub fn new(
        name: Option<String>,
        country: Option<String>,
        language_code: Option<String>,
        timezone: Option<String>,
        currency: Option<String>,
        source: Option<Source>,
        source_id: Option<String>,
    ) -> Result<Self, DomainError<VenueErrorKind>> {
        if source.is_some() && source_id.is_none() {
            return Err(DomainError::from(VenueErrorKind::New(
                VenueCreationErrorKind::InvalidSource,
            )));
        }

        Ok(Self {
            id: None,
            name,
            country,
            language_code,
            timezone,
            currency,
            source,
            source_id,
            created_at: None,
            last_updated_at: None,
        })
    }

    /// Restores a [`Venue`].
    pub fn restore(
        id: i32,
        name: Option<String>,
        country: Option<String>,
        language_code: Option<String>,
        timezone: Option<String>,
        currency: Option<String>,
        source: Option<Source>,
        source_id: Option<String>,
        created_at: NaiveDateTime,
        last_updated_at: Option<NaiveDateTime>,
    ) -> Result<Self, DomainError<VenueErrorKind>> {
        if source.is_some() && source_id.is_none() {
            return Err(DomainError::from(VenueErrorKind::Restore(
                VenueRestoreErrorKind::InvalidSource,
            )));
        }

        Ok(Self {
            id: Some(id),
            name,
            country,
            language_code,
            timezone,
            currency,
            source,
            source_id,
            created_at: Some(created_at),
            last_updated_at,
        })
    }

    /// Returns the ID of this [`Venue`].
    pub fn id(&self) -> Option<&i32> {
        self.id.as_ref()
    }

    /// Returns the name of this [`Venue`].
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Returns the country code of this [`Venue`].
    pub fn country(&self) -> Option<&String> {
        self.country.as_ref()
    }

    /// Returns the language code of this [`Venue`].
    pub fn language_code(&self) -> Option<&String> {
        self.language_code.as_ref()
    }

    /// Returns the timezone of this [`Venue`].
    pub fn timezone(&self) -> Option<&String> {
        self.timezone.as_ref()
    }

    /// Returns the currency of this [`Venue`].
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }

    /// Returns the [Source] of this [`Venue`].
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Returns the Source ID of this [`Venue`].
    pub fn source_id(&self) -> Option<&String> {
        self.source_id.as_ref()
    }

    /// Returns the creation date of this [`Venue`].
    pub fn created_at(&self) -> Option<&NaiveDateTime> {
        self.created_at.as_ref()
    }

    /// Returns the last update date of this [`Venue`].
    pub fn last_updated_at(&self) -> Option<&NaiveDateTime> {
        self.last_updated_at.as_ref()
    }
}

#[cfg(test)]
mod tests {
    mod new {
        use crate::{
            features::venue::{
                error::{VenueCreationErrorKind, VenueErrorKind},
                model::Venue,
            },
            shared::Source,
        };

        #[test]
        fn test_it_should_be_possible_to_create_a_venue() {
            // Given
            let expected_name = Some(String::from("Bar Armegeddon"));
            let expected_country = Some(String::from("hell"));
            let expected_language_code = Some(String::from("la-HE"));
            let expected_timezone = Some(String::from("test"));
            let expected_currency = Some(String::from("test"));
            let expected_source = Some(Source::Square);
            let expected_source_id = Some(String::from("666666"));

            // When
            let venue = Venue::new(
                expected_name.clone(),
                expected_country.clone(),
                expected_language_code.clone(),
                expected_timezone.clone(),
                expected_currency.clone(),
                expected_source.clone(),
                expected_source_id.clone(),
            )
            .expect("Expected the test Venue to resolve successfully.");

            // Then
            assert!(venue.id().is_none());
            assert_eq!(venue.name(), expected_name.as_ref());
            assert_eq!(venue.country(), expected_country.as_ref());
            assert_eq!(venue.language_code(), expected_language_code.as_ref());
            assert_eq!(venue.timezone(), expected_timezone.as_ref());
            assert_eq!(venue.currency(), expected_currency.as_ref());
            assert_eq!(venue.source(), expected_source.as_ref());
            assert_eq!(venue.source_id(), expected_source_id.as_ref());
            assert!(venue.created_at().is_none());
            assert!(venue.last_updated_at().is_none());
        }

        #[test]
        fn test_a_venue_should_have_a_source_id_when_it_has_a_source() {
            // Given
            let expected_name = Some(String::from("Bar Armegeddon"));
            let expected_country = Some(String::from("hell"));
            let expected_language_code = Some(String::from("la-HE"));
            let expected_timezone = Some(String::from("Europe/Oslo"));
            let expected_currency = Some(String::from("SLS"));
            let expected_source = Some(Source::Square);

            let expected_error_kind = VenueErrorKind::New(VenueCreationErrorKind::InvalidSource);

            // When
            let customer = Venue::new(
                expected_name,
                expected_country,
                expected_language_code,
                expected_timezone,
                expected_currency,
                expected_source,
                None,
            );

            // Then
            let error = customer.expect_err("Expected venue creation to fail without a source_id");
            assert_eq!(error.kind(), &expected_error_kind);
        }
    }

    mod restore {
        use chrono::Utc;

        use crate::{
            features::venue::{
                error::{VenueErrorKind, VenueRestoreErrorKind},
                model::Venue,
            },
            shared::Source,
        };

        #[test]
        fn test_it_should_be_possible_to_restore_a_venue() {
            // Given
            let expected_id = 555555;
            let expected_name = Some(String::from("Bar Armegeddon"));
            let expected_country = Some(String::from("hell"));
            let expected_language_code = Some(String::from("test"));
            let expected_timezone = Some(String::from("test"));
            let expected_currency = Some(String::from("test"));
            let expected_source = Some(Source::Square);
            let expected_source_id = Some(String::from("666666"));
            let expected_created_at = Utc::now().naive_utc();
            let expected_last_updated_at = Some(Utc::now().naive_utc());

            // When
            let venue = Venue::restore(
                expected_id,
                expected_name.clone(),
                expected_country.clone(),
                expected_language_code.clone(),
                expected_timezone.clone(),
                expected_currency.clone(),
                expected_source.clone(),
                expected_source_id.clone(),
                expected_created_at,
                expected_last_updated_at,
            )
            .expect("Expected the test Venue to resolve successfully.");

            // Then
            assert_eq!(venue.id(), Some(expected_id).as_ref());
            assert_eq!(venue.name(), expected_name.as_ref());
            assert_eq!(venue.country(), expected_country.as_ref());
            assert_eq!(venue.language_code(), expected_language_code.as_ref());
            assert_eq!(venue.timezone(), expected_timezone.as_ref());
            assert_eq!(venue.currency(), expected_currency.as_ref());
            assert_eq!(venue.source(), expected_source.as_ref());
            assert_eq!(venue.source_id(), expected_source_id.as_ref());
            assert_eq!(venue.created_at(), Some(expected_created_at).as_ref());
            assert_eq!(venue.last_updated_at(), expected_last_updated_at.as_ref());
        }

        #[test]
        fn test_a_venue_should_have_a_source_id_when_it_has_a_source() {
            // Given
            let expected_id = 555555;
            let expected_name = Some(String::from("Bar Armegeddon"));
            let expected_country = Some(String::from("hell"));
            let expected_language_code = Some(String::from("test"));
            let expected_timezone = Some(String::from("test"));
            let expected_currency = Some(String::from("test"));
            let expected_source = Some(Source::Square);
            let expected_created_at = Utc::now().naive_utc();
            let expected_last_updated_at = Some(Utc::now().naive_utc());

            let expected_error_kind = VenueErrorKind::Restore(VenueRestoreErrorKind::InvalidSource);

            // When
            let venue = Venue::restore(
                expected_id,
                expected_name.clone(),
                expected_country.clone(),
                expected_language_code.clone(),
                expected_timezone.clone(),
                expected_currency.clone(),
                expected_source.clone(),
                None,
                expected_created_at,
                expected_last_updated_at,
            );

            // Then
            let error = venue.expect_err("Expected venue restoration to fail without a source_id");
            assert_eq!(error.kind(), &expected_error_kind);
        }
    }
}
