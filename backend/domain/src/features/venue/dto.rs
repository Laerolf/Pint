use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    features::venue::{
        error::{VenueErrorKind, VenueMappingErrorKind},
        model::Venue,
    },
    shared::{
        error::DomainError,
        openapi::{example_date_string, example_id, example_source},
    },
};

/// Represents a Venue DTO.
#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct VenueDto {
    /// The ID of this Venue.
    #[schema(example = example_id)]
    pub id: i32,
    /// The name of this Venue.
    #[schema(example = "Bar Armegeddon")]
    pub name: Option<String>,
    /// The country code of this Venue.
    #[schema(example = "hell")]
    pub country: Option<String>,
    /// The language code of this Venue.
    #[schema(example = "la-HE")]
    pub language_code: Option<String>,
    /// The timezone of this Venue.
    #[schema(example = "Europe/Oslo")]
    pub timezone: Option<String>,
    /// The currency code of this Venue.
    #[schema(example = "SLS")]
    pub currency: Option<String>,
    /// The origin of this Venue.
    #[schema(example = example_source)]
    pub source: Option<String>,
    /// The origin ID of this Venue.
    #[schema(example = example_id)]
    pub source_id: Option<String>,
    /// The date of the creation date of this Venue.
    #[schema(example = example_date_string)]
    pub created_at: Option<String>,
    /// The date of the last update of this Venue.
    pub last_updated_at: Option<String>,
}

impl VenueDto {
    /// Creates a [`VenueDto`] based on a [Venue]
    pub fn from(venue: &Venue) -> Result<Self, DomainError<VenueErrorKind>> {
        let id = venue.id().cloned().ok_or_else(|| {
            DomainError::from(VenueErrorKind::Map(VenueMappingErrorKind::NotPersisted))
        })?;

        Ok(Self {
            id,
            name: venue.name().cloned(),
            country: venue.country().cloned(),
            language_code: venue.language_code().cloned(),
            timezone: venue.timezone().cloned(),
            currency: venue.currency().cloned(),
            source: venue.source().map(|source| source.to_string()),
            source_id: venue.source_id().cloned(),
            created_at: venue
                .created_at()
                .map(|created_at| created_at.and_utc().to_rfc3339()),
            last_updated_at: venue
                .last_updated_at()
                .map(|last_updated_at| last_updated_at.and_utc().to_rfc3339()),
        })
    }
}

#[cfg(test)]
mod tests {
    mod from {
        use chrono::Utc;

        use crate::{
            features::venue::{dto::VenueDto, model::Venue},
            shared::Source,
        };

        #[test]
        fn test_a_venue_dto_can_be_created_with_a_persisted_venue() {
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

            // When
            let dto = VenueDto::from(&venue)
                .expect("Expected the test Customer DTO to resolve successfully.");

            // Then
            assert_eq!(Some(&dto.id), venue.id());
            assert_eq!(dto.name.as_ref(), venue.name());
            assert_eq!(dto.country.as_ref(), venue.country());
            assert_eq!(dto.language_code.as_ref(), venue.language_code());
            assert_eq!(dto.timezone.as_ref(), venue.timezone());
            assert_eq!(dto.currency.as_ref(), venue.currency());
            assert_eq!(dto.source, venue.source().map(|source| source.to_string()));
            assert_eq!(dto.source_id.as_ref(), venue.source_id());
            assert_eq!(
                dto.created_at,
                venue
                    .created_at()
                    .map(|created_at| created_at.and_utc().to_rfc3339())
            );
            assert_eq!(
                dto.last_updated_at,
                venue
                    .last_updated_at()
                    .map(|last_updated_at| last_updated_at.and_utc().to_rfc3339())
            );
        }
    }
}
