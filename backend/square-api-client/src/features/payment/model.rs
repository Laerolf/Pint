use serde::Deserialize;

use crate::shared::response::objects::Money;

/// Represents a Payment in the Square API.
/// * [Square API Reference](https://developer.squareup.com/reference/square_2026-08-19/objects/Payment)
#[derive(Deserialize, Clone)]
pub struct Payment {
    /// A unique ID for the payment.
    pub id: String,
    /// The timestamp of when the payment was created, in RFC 3339 format.
    pub created_at: String,
    /// The timestamp of when the payment was last updated, in RFC 3339 format.
    pub updated_at: String,
    /// The total amount for the payment, including amount_money and tip_money.
    pub total_money: Money,
    /// Indicates whether the payment is APPROVED, PENDING, COMPLETED, CANCELED, or FAILED.
    pub status: String,
    /// The source type for this payment.
    pub source_type: String,
    /// The ID of the location associated with the payment.
    pub location_id: String,
    /// The ID of the order associated with the payment.
    pub order_id: Option<String>,
    /// An optional ID that associates the payment with an entity in another system.
    pub reference_id: Option<String>,
    /// The ID of the customer associated with the payment.
    pub customer_id: Option<String>,
}
