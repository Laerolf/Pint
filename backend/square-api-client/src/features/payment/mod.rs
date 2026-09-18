use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    features::payment::{error::PaymentsEndpointErrorKind, model::Payment},
    shared::{
        client::{SquareApiClient, error::SquareClientError},
        response::SquareApiResponseError,
    },
};

pub mod error;
pub mod model;

/// Response envelope for Square's list payments endpoint.
#[derive(Deserialize, Clone)]
pub struct ListPaymentsResponse {
    /// The errors in the Square API response if an error occurred.
    pub errors: Option<Vec<SquareApiResponseError>>,
    /// The Square API Payment list if no error occurred.
    pub payments: Option<Vec<Payment>>,
    /// The cursor string in case there are still Payments to load.
    pub cursor: Option<String>,
}

/// Represents the available Square Payments API endpoints.
pub struct PaymentEndpoint {
    client: SquareApiClient,
}

impl PaymentEndpoint {
    /// Creates a new [`CustomerEndpoint`].
    pub fn new(client: SquareApiClient) -> Self {
        Self { client }
    }

    /// Retrieves a list of all existing Payments in the Square API.
    /// * [Square API reference](https://developer.squareup.com/reference/square/payments-api/list-payments)
    pub async fn list_payments(
        &self,
        cursor: Option<String>,
        location_id: Option<String>,
    ) -> Result<ListPaymentsResponse, SquareClientError<PaymentsEndpointErrorKind>> {
        let mut query: HashMap<String, String> =
            HashMap::from([(String::from("count"), true.to_string())]);

        if let Some(cursor) = cursor {
            query.insert(String::from("cursor"), cursor);
        }

        if let Some(location_id) = location_id {
            query.insert(String::from("location_id"), location_id);
        }

        self.client
            .get::<ListPaymentsResponse>("/payments", Some(query))
            .await
            .map_err(|error| {
                SquareClientError::from(PaymentsEndpointErrorKind::ListPayments).with_cause(error)
            })
    }
}

#[cfg(test)]
mod tests {
    mod list_payments {
        use std::fs;
        use wiremock::{
            Mock, MockServer, ResponseTemplate,
            matchers::{method, path},
        };

        use crate::{
            features::payment::PaymentEndpoint,
            shared::client::{SquareApiClient, SquareApiClientOptions},
        };

        #[tokio::test]
        async fn test_list_payments_returns_a_payments_list() {
            // Given
            let mock_server = MockServer::start().await;
            let payments_list_fixture =
                fs::read_to_string("tests/fixtures/payment/payments_list.json").unwrap();

            Mock::given(method("GET"))
                .and(path("/payments"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(payments_list_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = PaymentEndpoint::new(client)
                .list_payments(None, None)
                .await
                .expect("Expected the Payments list to be retrieved successfully.");

            // Then
            assert!(!response.payments.unwrap_or_default().is_empty());
            assert!(response.cursor.is_none());
        }

        #[tokio::test]
        async fn test_list_payments_returns_a_payments_list_with_a_cursor() {
            // Given
            let mock_server = MockServer::start().await;
            let payments_list_fixture =
                fs::read_to_string("tests/fixtures/payment/payments_list_with_cursor.json")
                    .unwrap();

            Mock::given(method("GET"))
                .and(path("/payments"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(payments_list_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = PaymentEndpoint::new(client)
                .list_payments(None, None)
                .await
                .expect("Expected the Payments list to be retrieved successfully.");

            // Then
            assert!(!response.payments.unwrap_or_default().is_empty());
            assert!(response.cursor.is_some());
        }

        #[tokio::test]
        async fn test_list_payments_with_a_location_id_returns_a_payments_list() {
            // Given
            let mock_server = MockServer::start().await;
            let payments_list_fixture =
                fs::read_to_string("tests/fixtures/payment/payments_list_with_location_id.json")
                    .unwrap();
            let location_id = String::from("666666");

            Mock::given(method("GET"))
                .and(path("/payments"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(payments_list_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = PaymentEndpoint::new(client)
                .list_payments(None, Some(location_id))
                .await
                .expect("Expected the Payments list to be retrieved successfully.");

            // Then
            assert!(!response.payments.unwrap_or_default().is_empty());
            assert!(response.cursor.is_none());
        }

        #[tokio::test]
        async fn test_list_payments_throws_an_error() {
            // Given
            let mock_server = MockServer::start().await;
            let errors_fixture = fs::read_to_string("tests/fixtures/errors.json").unwrap();

            Mock::given(method("GET"))
                .and(path("/payments"))
                .respond_with(
                    ResponseTemplate::new(429).set_body_raw(errors_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = PaymentEndpoint::new(client).list_payments(None, None).await;

            // Then
            assert!(response.is_err());
        }
    }
}
