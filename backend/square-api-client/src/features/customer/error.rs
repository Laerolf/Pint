use std::fmt::Display;

use crate::shared::client::error::SquareClientErrorKind;

#[derive(Debug)]
pub enum CustomerEndpointErrorKind {
    ListCustomer,
}

impl SquareClientErrorKind for CustomerEndpointErrorKind {
    fn code(&self) -> String {
        match self {
            Self::ListCustomer => "error.client.endpoint.listCustomer".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::ListCustomer => {
                "Failed to get a list of Customer from the Square API endpoint.".to_string()
            }
        }
    }
}

impl Display for CustomerEndpointErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

#[cfg(test)]
mod tests {
    mod customer_endpoint_error_kind {
        use crate::features::customer::error::CustomerEndpointErrorKind;

        #[test]
        fn test_string_format() {
            // Given
            let expected_list_customer_error_kind_message = "[error.client.endpoint.listCustomer] Failed to get a list of Customer from the Square API endpoint.";

            // When
            let list_customer_error_kind = CustomerEndpointErrorKind::ListCustomer;

            // Then
            assert_eq!(
                list_customer_error_kind.to_string(),
                expected_list_customer_error_kind_message
            );
        }
    }
}
