use serde::Deserialize;

/// Represents a Customer in the Square API.
/// * [Square API Reference](https://developer.squareup.com/reference/square_2026-08-19/objects/Customer)
#[derive(Deserialize, Clone)]
pub struct Customer {
    ///A unique Square-assigned ID for the customer profile.
    pub id: String,
    /// The timestamp when the customer profile was created, in RFC 3339 format.
    pub created_at: String,
    /// The timestamp when the customer profile was last updated, in RFC 3339 format.
    pub updated_at: String,
    /// The given name (that is, the first name) associated with the customer profile.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the customer profile.
    pub family_name: Option<String>,
    /// A nickname for the customer profile.
    pub nickname: Option<String>,
    /// A business name associated with the customer profile.
    pub company_name: Option<String>,
    /// The email address associated with the customer profile.
    pub email_address: Option<String>,
    /// The phone number associated with the customer profile.
    pub phone_number: Option<String>,
    /// The birthday associated with the customer profile, in YYYY-MM-DD format.
    pub birthday: Option<String>,
    /// An optional second ID used to associate the customer profile with an entity in another system.
    pub reference_id: Option<String>,
    /// A custom note associated with the customer profile.
    pub note: Option<String>,
    /// The Square-assigned version number of the customer profile.
    pub version: i64,
}
