use std::fmt::Display;

use crate::shared::client::error::SquareClientErrorKind;

#[derive(Debug)]
pub enum LocationEndpointErrorKind {
    /// Failed to get a list of Locations from the Square API endpoint.
    ListLocations,
    /// Failed to get a Location with the provided ID from the Square API endpoint.
    RetrieveLocation,
}

impl SquareClientErrorKind for LocationEndpointErrorKind {
    fn code(&self) -> String {
        match self {
            Self::ListLocations => "error.client.endpoint.listLocations".to_string(),
            Self::RetrieveLocation => "error.client.endpoint.retrieveLocation".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::ListLocations => {
                "Failed to get a list of Locations from the Square API endpoint.".to_string()
            }
            Self::RetrieveLocation => {
                "Failed to get a Location with the provided ID from the Square API endpoint."
                    .to_string()
            }
        }
    }
}

impl Display for LocationEndpointErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

#[cfg(test)]
mod tests {
    mod location_endpoint_error_kind {
        use crate::features::location::error::LocationEndpointErrorKind;

        #[test]
        fn test_string_format() {
            // Given
            let expected_error_kind_message = "[error.client.endpoint.listLocations] Failed to get a list of Locations from the Square API endpoint.";

            // When
            let error_kind = LocationEndpointErrorKind::ListLocations;

            // Then
            assert_eq!(error_kind.to_string(), expected_error_kind_message);
        }
    }
}
