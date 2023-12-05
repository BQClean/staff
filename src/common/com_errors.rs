use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ServiceError(pub String);

impl From<&str> for ServiceError {
    fn from(value: &str) -> Self {
        return Self(value.to_string());
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

impl std::error::Error for ServiceError {}