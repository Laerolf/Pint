use chrono::Utc;

use entity::venues;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::{
    features::venue::{
        error::{VenueErrorKind, VenueMappingErrorKind},
        model::Venue,
    },
    shared::{Source, error::DomainError},
};

/// Represents a Mapper for [`Venues`][Venue]
pub struct VenueMapper;

impl VenueMapper {
    /// Maps a [Venue model][venues::Model] to an [Venue].
    pub fn to_domain_model(model: &venues::Model) -> Result<Venue, DomainError<VenueErrorKind>> {
        let source = model
            .source
            .as_ref()
            .map(|source| {
                Source::from(source).map_err(|error| {
                    DomainError::from(VenueErrorKind::Map(
                        VenueMappingErrorKind::FromSquareToActiveModel,
                    ))
                    .with_cause(error)
                })
            })
            .transpose()?;

        Venue::restore(
            model.id,
            model.name.clone(),
            model.country.clone(),
            model.language_code.clone(),
            model.timezone.clone(),
            model.currency.clone(),
            source,
            model.source_id.clone(),
            model.created_at,
            model.last_updated_at,
        )
    }

    /// Maps a [Venue] to an [active model][venues::ActiveModel] to insert.
    pub fn to_insert_active_model(venue: &Venue) -> venues::ActiveModel {
        venues::ActiveModel {
            id: NotSet,
            name: Set(venue.name().cloned()),
            country: Set(venue.country().cloned()),
            language_code: Set(venue.language_code().cloned()),
            timezone: Set(venue.timezone().cloned()),
            currency: Set(venue.currency().cloned()),
            source: Set(venue.source().map(|source| source.to_string())),
            source_id: Set(venue.source_id().cloned()),
            created_at: Set(Utc::now().naive_utc()),
            last_updated_at: NotSet,
        }
    }

    /// Maps a [Venue] to an [active model][venues::ActiveModel] to update.
    pub fn to_update_active_model(
        venue: &Venue,
    ) -> Result<venues::ActiveModel, DomainError<VenueErrorKind>> {
        Ok(venues::ActiveModel {
            id: Unchanged(venue.id().cloned().ok_or_else(|| {
                DomainError::from(VenueErrorKind::Map(VenueMappingErrorKind::NotPersisted))
            })?),
            name: Set(venue.name().cloned()),
            country: Set(venue.country().cloned()),
            language_code: Set(venue.language_code().cloned()),
            timezone: Set(venue.timezone().cloned()),
            currency: Set(venue.currency().cloned()),
            source: Set(venue.source().map(|source| source.to_string())),
            source_id: Set(venue.source_id().cloned()),
            created_at: Set(Utc::now().naive_utc()),
            last_updated_at: NotSet,
        })
    }
}
