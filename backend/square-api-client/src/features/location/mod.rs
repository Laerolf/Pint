use serde::Deserialize;

use crate::{
    features::location::{error::LocationEndpointErrorKind, model::Location},
    shared::{
        client::{SquareApiClient, error::SquareClientError},
        response::SquareApiResponseError,
    },
};

pub mod error;
pub mod mapper;
pub mod model;

/// Response envelope for Square's location retrieval endpoint.
#[derive(Deserialize, Clone)]
pub struct RetrieveLocationResponse {
    /// The errors in the Square API response if an error occurred.
    pub errors: Option<Vec<SquareApiResponseError>>,
    /// The Square API Location if no error occurred.
    pub location: Option<Location>,
}

/// Response envelope for Square's list locations endpoint.
#[derive(Deserialize, Clone)]
pub struct ListLocationsResponse {
    /// The errors in the Square API response if an error occurred.
    pub errors: Option<Vec<SquareApiResponseError>>,
    /// The Square API Location list if no error occurred.
    pub locations: Option<Vec<Location>>,
}

/// Represents the available Square locations API endpoints.
pub struct LocationEndpoint {
    client: SquareApiClient,
}

impl LocationEndpoint {
    /// Creates a new [`LocationEndpoint`].
    pub fn new(client: SquareApiClient) -> Self {
        Self { client }
    }

    /// Retrieves the a Location with the provided ID in the Square API.
    /// * [Square API reference](https://developer.squareup.com/reference/square/locations-api/retrieve-location)
    pub async fn retrieve_location(
        &self,
        location_id: &str,
    ) -> Result<RetrieveLocationResponse, SquareClientError<LocationEndpointErrorKind>> {
        self.client
            .get::<RetrieveLocationResponse>(
                &format!("/locations/{}", location_id).to_string(),
                None,
            )
            .await
            .map_err(|error| {
                SquareClientError::from(LocationEndpointErrorKind::RetrieveLocation)
                    .with_cause(error)
            })
    }
}

#[cfg(test)]
mod tests {
    mod retrieve_location {
        use std::fs;
        use wiremock::{
            Mock, MockServer, ResponseTemplate,
            matchers::{method, path},
        };

        use crate::{
            features::location::LocationEndpoint,
            shared::client::{SquareApiClient, SquareApiClientOptions},
        };

        #[tokio::test]
        async fn test_retrieve_location_returns_a_location() {
            // Given
            let mock_server = MockServer::start().await;
            let list_fixture =
                fs::read_to_string("tests/fixtures/location/retrieve_location.json").unwrap();
            let expected_location_id = "666666";

            Mock::given(method("GET"))
                .and(path(
                    format!("/locations/{}", expected_location_id).to_string(),
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_raw(list_fixture, "application/json"),
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
            let response = LocationEndpoint::new(client)
                .retrieve_location(expected_location_id)
                .await
                .expect("Expected the Location to be retrieved successfully.");

            // Then
            assert!(response.location.is_some());
        }

        #[tokio::test]
        async fn test_retrieve_location_throws_an_error() {
            // Given
            let mock_server = MockServer::start().await;
            let errors_fixture = fs::read_to_string("tests/fixtures/errors.json").unwrap();
            let expected_location_id = "666666";

            Mock::given(method("GET"))
                .and(path(
                    format!("/locations/{}", expected_location_id).to_string(),
                ))
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
            let response = LocationEndpoint::new(client)
                .retrieve_location(expected_location_id)
                .await;

            // Then
            assert!(response.is_err());
        }
    }
}
