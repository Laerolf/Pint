use serde::Deserialize;

/// Represents the status of a [Location].
#[derive(Deserialize, Clone)]
pub enum Status {
    /// A location that is active for business.
    #[serde(rename = "ACTIVE")]
    Active,
    /// A location that is not active for business.
    #[serde(rename = "INACTIVE")]
    Inactive,
}

/// Represents a Location in the Square API.
/// * [Square API Reference](https://developer.squareup.com/reference/square_2026-08-19/objects/Location)
#[derive(Deserialize, Clone)]
pub struct Location {
    /// A short generated string of letters and numbers that uniquely identifies this location instance.
    pub id: String,
    /// The name of the location's overall business.
    pub business_name: Option<String>,
    /// The country of the location, in the two-letter format of ISO 3166.
    pub country: String,
    /// The language associated with the location, in BCP 47 format.
    pub language_code: Option<String>,
    /// The currency used for all transactions at this location, in ISO 4217 format.
    pub currency: Option<String>,
    /// The status of the location.
    pub status: Option<Status>,
    /// The ID of the merchant that owns the location.
    pub merchant_id: Option<String>,
    /// The IANA time zone identifier for the time zone of the location.
    pub timezone: Option<String>,
    /// The time when the location was created, in RFC 3339 format.
    pub created_at: String,
}
