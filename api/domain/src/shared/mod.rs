use std::fmt::{Display, Formatter};

pub mod error;
pub mod openapi;

/// Represents the source of the domain element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Source {
    /// Square is the origin of this element.
    Square,
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Source::Square => write!(f, "Square"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::Source;

    #[test]
    fn test_string_format() {
        // When + then
        assert_eq!(Source::Square.to_string(), "Square");
    }
}
