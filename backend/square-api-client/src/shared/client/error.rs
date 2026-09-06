use std::{collections::HashMap, fmt::Display};

/// Represents a Square API Client error.
pub struct SquareClientError<K> {
    kind: K,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    context: Option<HashMap<String, String>>,
}

impl<K> SquareClientError<K>
where
    K: SquareClientErrorKind,
{
    pub fn from(kind: K) -> Self {
        Self {
            kind,
            cause: None,
            context: None,
        }
    }

    pub fn with_cause(mut self, cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }

    pub fn cause(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.cause.as_deref()
    }

    pub fn context(&self) -> &Option<HashMap<String, String>> {
        &self.context
    }
}

impl<K> std::error::Error for SquareClientError<K> where K: SquareClientErrorKind + Send + Sync {}

impl<K> std::fmt::Debug for SquareClientError<K>
where
    K: SquareClientErrorKind,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("DomainError");
        debug.field("code", &self.kind.code());
        debug.field("message", &self.kind.message());
        debug.field("cause", &self.cause);
        debug.field("context", &self.context);
        debug.finish()
    }
}

impl<K> Display for SquareClientError<K>
where
    K: SquareClientErrorKind + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.kind().code(), self.kind().message())
    }
}

/// Represents the kind of a [`SquareClientError`].
pub trait SquareClientErrorKind {
    /// Gets the locale code of this [`SquareClientErrorKind`].
    fn code(&self) -> String;

    /// Gets the message of this [`SquareClientErrorKind`].
    fn message(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClientErrorKind {
    Setup,
    GetRequest,
}

impl SquareClientErrorKind for ClientErrorKind {
    fn code(&self) -> String {
        match self {
            Self::Setup => "error.client.setup".to_string(),
            Self::GetRequest => "error.client.getRequest".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Setup => "Failed to setup the Square API client.".to_string(),
            Self::GetRequest => "Something went wrong during a Square API GET request.".to_string(),
        }
    }
}

impl Display for ClientErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

#[cfg(test)]
mod tests {
    mod square_client_error {
        mod from {
            use std::io::{Error, ErrorKind};

            use crate::shared::client::error::{ClientErrorKind, SquareClientError};

            #[test]
            fn test_it_can_be_created() {
                // Given
                let expected_error_kind = ClientErrorKind::Setup;
                let expected_cause = Error::new(ErrorKind::Other, "Test");
                let expected_context_message = "Beeeeeeeeeeeeep!";

                // When
                let error = SquareClientError::from(expected_error_kind)
                    .with_cause(expected_cause)
                    .with_context("message", expected_context_message);

                // Then
                assert_eq!(*error.kind(), expected_error_kind);

                let cause = error.cause().unwrap();
                let io_cause = cause.downcast_ref::<std::io::Error>().unwrap();
                assert_eq!(io_cause.kind(), ErrorKind::Other);

                assert_eq!(
                    error.context().as_ref().unwrap().get("message"),
                    Some(&expected_context_message.to_string())
                );
            }

            #[test]
            fn test_string_format() {
                // Given
                let expected_setup_error_message =
                    "[error.client.setup] Failed to setup the Square API client.";
                let expected_get_request_error_message = "[error.client.getRequest] Something went wrong during a Square API GET request.";

                // When
                let setup_error = SquareClientError::from(ClientErrorKind::Setup);
                let get_request_error = SquareClientError::from(ClientErrorKind::GetRequest);

                // Then
                assert_eq!(setup_error.to_string(), expected_setup_error_message);
                assert_eq!(
                    get_request_error.to_string(),
                    expected_get_request_error_message
                );
            }

            #[test]
            fn test_debug_format() {
                // Given
                let cause = Error::new(std::io::ErrorKind::Other, "boom");
                let error = SquareClientError::from(ClientErrorKind::GetRequest)
                    .with_cause(cause)
                    .with_context("status", "429");

                // When
                let debug_output = format!("{:?}", error);

                // Then
                assert!(debug_output.contains("error.client.getRequest"));
                assert!(debug_output.contains("boom"));
                assert!(debug_output.contains("\"status\""));
                assert!(debug_output.contains("\"429\""));
            }
        }
    }

    mod client_error_kind {
        use crate::shared::client::error::ClientErrorKind;

        #[test]
        fn test_string_format() {
            // Given
            let expected_setup_error_kind_message =
                "[error.client.setup] Failed to setup the Square API client.";
            let expected_get_request_error_kind_message =
                "[error.client.getRequest] Something went wrong during a Square API GET request.";

            // When
            let setup_error_kind = ClientErrorKind::Setup;
            let get_request_error_kind = ClientErrorKind::GetRequest;

            // Then
            assert_eq!(
                setup_error_kind.to_string(),
                expected_setup_error_kind_message
            );
            assert_eq!(
                get_request_error_kind.to_string(),
                expected_get_request_error_kind_message
            );
        }
    }
}
