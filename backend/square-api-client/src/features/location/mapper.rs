use chrono::Utc;
use domain::{
    features::venue::error::VenueErrorKind,
    shared::{Source, error::DomainError},
};
use entity::venues;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::features::location::model::Location;

/// Represents a Mapper for [`Square Location`][Location]
pub struct SquareLocationMapper;

impl SquareLocationMapper {
    /// Maps a [Square Location][Location] to an [active model][venues::ActiveModel] to insert.
    pub fn to_insert_active_model(
        location: &Location,
    ) -> Result<venues::ActiveModel, DomainError<VenueErrorKind>> {
        Ok(venues::ActiveModel {
            id: NotSet,
            name: Set(location.business_name.clone()),
            country: Set(Some(location.country.clone())),
            language_code: Set(location.language_code.clone()),
            timezone: Set(location.timezone.clone()),
            currency: Set(location.currency.clone()),
            source: Set(Some(Source::Square.to_string())),
            source_id: Set(Some(location.id.clone())),
            created_at: Set(Utc::now().naive_utc()),
            last_updated_at: NotSet,
        })
    }
}
