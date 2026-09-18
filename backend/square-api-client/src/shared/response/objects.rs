use serde::Deserialize;

/// Represents the Money unit in the Square API.
/// * [Square API Reference](https://developer.squareup.com/reference/square_2026-08-19/objects/Money)
#[derive(Deserialize, Clone)]
pub struct Money {
    /// The amount of money, in the smallest denomination of the currency indicated by currency.
    pub amount: i64,
    /// The type of currency, in ISO 4217 format.
    pub currency: String,
}
