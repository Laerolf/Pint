use std::{collections::HashMap, fmt::Display};

/// Represents a domain error.
pub struct DomainError<K> {
    kind: K,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    context: Option<HashMap<String, String>>,
}

impl<K> DomainError<K>
where
    K: DomainErrorKind,
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

impl<K> std::error::Error for DomainError<K> where K: DomainErrorKind + Send + Sync {}

impl<K> std::fmt::Debug for DomainError<K>
where
    K: DomainErrorKind,
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

impl<K> Display for DomainError<K>
where
    K: DomainErrorKind + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.kind().code(), self.kind().message())
    }
}

/// Represents the kind of a [`SquareClientError`].
pub trait DomainErrorKind {
    /// Gets the locale code of this [`SquareClientErrorKind`].
    fn code(&self) -> String;

    /// Gets the message of this [`SquareClientErrorKind`].
    fn message(&self) -> String;
}

#[cfg(test)]
mod tests {
    mod domain_error {
        mod from {
            use std::io::{Error, ErrorKind};

            use crate::{
                features::error::{CustomerCreationErrorKind, CustomerErrorKind},
                shared::error::DomainError,
            };

            #[test]
            fn test_it_can_be_created() {
                // Given
                let expected_error_kind =
                    CustomerErrorKind::New(CustomerCreationErrorKind::InvalidSource);
                let expected_cause = Error::new(ErrorKind::Other, "Test");
                let expected_context_message = "Beeeeeeeeeeeeep!";

                // When
                let error = DomainError::from(expected_error_kind)
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
                    "[error.domain.customer.new] Failed to create a new Customer.";

                // When
                let setup_error = DomainError::from(CustomerErrorKind::New(
                    CustomerCreationErrorKind::InvalidSource,
                ));

                // Then
                assert_eq!(setup_error.to_string(), expected_setup_error_message);
            }

            #[test]
            fn test_debug_format() {
                // Given
                let cause = Error::new(std::io::ErrorKind::Other, "boom");
                let error = DomainError::from(CustomerErrorKind::New(
                    CustomerCreationErrorKind::InvalidSource,
                ))
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
}
