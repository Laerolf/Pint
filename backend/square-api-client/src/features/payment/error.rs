use std::fmt::Display;

use crate::shared::client::error::SquareClientErrorKind;

#[derive(Debug)]
pub enum PaymentsEndpointErrorKind {
    ListPayments,
}

impl SquareClientErrorKind for PaymentsEndpointErrorKind {
    fn code(&self) -> String {
        match self {
            Self::ListPayments => "error.client.endpoint.listPayments".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::ListPayments => {
                "Failed to get a list of Payments from the Square API endpoint.".to_string()
            }
        }
    }
}

impl Display for PaymentsEndpointErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

#[cfg(test)]
mod tests {
    mod payments_endpoint_error_kind {
        use crate::features::payment::error::PaymentsEndpointErrorKind;

        #[test]
        fn test_string_format() {
            // Given
            let expected_list_payments_error_kind_message = "[error.client.endpoint.listPayments] Failed to get a list of Payments from the Square API endpoint.";

            // When
            let list_payments_error_kind = PaymentsEndpointErrorKind::ListPayments;

            // Then
            assert_eq!(
                list_payments_error_kind.to_string(),
                expected_list_payments_error_kind_message
            );
        }
    }
}
