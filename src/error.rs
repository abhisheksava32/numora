//! Errors returned by every part of numora.

use std::fmt;

/// Everything that can go wrong in numora.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Shapes that do not fit together, for example adding a 2x3 matrix to a 3x2 one.
    Dimension(String),
    /// A matrix that cannot be inverted or used to solve a system.
    Singular,
    /// A math operation outside its domain, for example the mean of nothing.
    Domain(String),
    /// Input that could not be read as numora syntax.
    Parse(String),
    /// A well-formed expression that could not be evaluated, for example an unknown variable.
    Eval(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dimension(msg) => write!(f, "dimension mismatch: {msg}"),
            Error::Singular => write!(f, "matrix is singular"),
            Error::Domain(msg) => write!(f, "domain error: {msg}"),
            Error::Parse(msg) => write!(f, "parse error: {msg}"),
            Error::Eval(msg) => write!(f, "error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// Shorthand for results that use [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_are_readable() {
        assert_eq!(Error::Singular.to_string(), "matrix is singular");
        assert_eq!(
            Error::Dimension("2x3 vs 3x2".into()).to_string(),
            "dimension mismatch: 2x3 vs 3x2"
        );
        assert_eq!(
            Error::Domain("mean of an empty list".into()).to_string(),
            "domain error: mean of an empty list"
        );
        assert_eq!(
            Error::Parse("unexpected ')'".into()).to_string(),
            "parse error: unexpected ')'"
        );
        assert_eq!(
            Error::Eval("unknown variable x".into()).to_string(),
            "error: unknown variable x"
        );
    }
}
