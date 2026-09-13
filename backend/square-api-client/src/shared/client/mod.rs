use std::{collections::HashMap, str::FromStr, time::Duration};

use reqwest::{
    Client, Url,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue},
};
use serde::de::DeserializeOwned;

use crate::shared::client::error::{ClientErrorKind, SquareClientError};

pub mod error;

/// Represents the Square API setup arguments.
pub struct SquareApiClientOptions {
    /// The Square API base URL.
    pub base_url: Url,
    /// The Square API token used for authentication.
    pub square_api_token: String,
    /// The Square version to use as a header.
    pub square_api_version: String,
}

/// Represents a Square API client.
pub struct SquareApiClient {
    base_url: Url,
    client: Client,
}

impl SquareApiClient {
    /// Returns the default headers for a [`SquareApiClient`](./SquareApiClient).
    fn default_headers(
        square_api_token: String,
        square_api_version: String,
    ) -> Result<HeaderMap, SquareClientError<ClientErrorKind>> {
        let auth_header_value = HeaderValue::from_str(&["Bearer", &square_api_token].join(" "))
            .map_err(|error| SquareClientError::from(ClientErrorKind::Setup).with_cause(error))?;
        let version_header_value = HeaderValue::from_str(&square_api_version)
            .map_err(|error| SquareClientError::from(ClientErrorKind::Setup).with_cause(error))?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, auth_header_value);
        headers.insert(
            HeaderName::from_str("Square-Version").unwrap(),
            version_header_value,
        );
        Ok(headers)
    }

    /// Creates a new [`SquareApiClient`](./SquareApiClient).
    pub fn new(
        options: SquareApiClientOptions,
    ) -> Result<Self, SquareClientError<ClientErrorKind>> {
        let default_headers =
            SquareApiClient::default_headers(options.square_api_token, options.square_api_version)?;

        Ok(Self {
            base_url: options.base_url,
            client: Client::builder()
                .default_headers(default_headers)
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(|error| {
                    SquareClientError::from(ClientErrorKind::Setup).with_cause(error)
                })?,
        })
    }

    /// Executes a GET request to the resolved URL of the base URL and the provided URL.
    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<T, SquareClientError<ClientErrorKind>> {
        let mut request_builder = self
            .client
            .get(self.base_url.join(path.trim_start_matches('/')).unwrap());

        if query.is_some() {
            request_builder = request_builder.query(&query);
        }

        let request = request_builder.build().map_err(|error| {
            SquareClientError::from(ClientErrorKind::GetRequest).with_cause(error)
        })?;

        // TODO: Replace with a debug-level log
        println!("GET {}", request.url());

        let response = self.client.execute(request).await.map_err(|error| {
            SquareClientError::from(ClientErrorKind::GetRequest).with_cause(error)
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SquareClientError::from(ClientErrorKind::GetRequest)
                .with_context("status", status.as_u16().to_string())
                .with_context("body", body));
        }

        response
            .json::<T>()
            .await
            .map_err(|error| SquareClientError::from(ClientErrorKind::GetRequest).with_cause(error))
    }
}

#[cfg(test)]
mod tests {
    mod square_api_client {
        mod new {
            use reqwest::Url;

            use crate::shared::client::{SquareApiClient, SquareApiClientOptions};

            #[test]
            fn test_can_be_created() {
                // Given
                let base_url = "https://armageddon.hell";
                let square_api_token = "test-test".to_string();
                let square_api_version = "2026-08-19".to_string();

                // When
                let client = SquareApiClient::new(SquareApiClientOptions {
                    base_url: Url::parse(base_url).unwrap(),
                    square_api_token: square_api_token.to_string(),
                    square_api_version: square_api_version.to_string(),
                });

                // Then
                assert!(client.is_ok());
            }
        }
    }
}
