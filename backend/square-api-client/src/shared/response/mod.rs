use serde::Deserialize;

/// A Square API Error category.
/// * [Square API: ErrorCategory](https://developer.squareup.com/reference/square/objects/ErrorCategory)
#[derive(Deserialize, Clone)]
pub enum SquareApiErrorCategory {
    /// An error occurred with the Connect API itself.
    #[serde(rename = "API_ERROR")]
    ApiError,

    /// An authentication error occurred.
    #[serde(rename = "AUTHENTICATION_ERROR")]
    AuthenticationError,

    /// The request was invalid.
    #[serde(rename = "INVALID_REQUEST_ERROR")]
    InvalidRequestError,

    /// Your application reached the Square API rate limit.
    #[serde(rename = "RATE_LIMIT_ERROR")]
    RateLimitError,

    /// An error occurred while processing a payment method.
    #[serde(rename = "PAYMENT_METHOD_ERROR")]
    PaymentMethodError,

    /// An error occurred while attempting to process a refund.
    #[serde(rename = "REFUND_ERROR")]
    RefundError,

    /// An error occurred when checking a merchant subscription status.
    #[serde(rename = "MERCHANT_SUBSCRIPTION_ERROR")]
    MerchantSubscriptionError,

    /// An error that is returned from an external vendor's API.
    #[serde(rename = "EXTERNAL_VENDOR_ERROR")]
    ExternalVendorError,
}

/// A Square API Error.
/// * [Square API: Error object properties](https://developer.squareup.com/docs/build-basics/general-considerations/handling-errors#error-object-properties)
#[derive(Deserialize, Clone)]
pub struct SquareApiResponseError {
    /// A high-level classification of the error.
    pub category: SquareApiErrorCategory,
    /// A specific identifier for the error.
    /// * [Square API: ErrorCode](https://developer.squareup.com/reference/square/objects/ErrorCode)
    pub code: String,
    /// A human-readable error description for developers, not intended to be customer facing.
    pub detail: Option<String>,
    /// The name of the field in the original request (if applicable) that the error pertains to.
    pub field: Option<String>,
}
