use std::collections::HashMap;

/// A error
#[derive(Debug)]
pub struct ErrorResponse {
    /// The header
    pub header: ErrorResponseHeader,

    /// Extra K/Vs
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// A client side error header
#[derive(Debug, serde::Deserialize)]
pub struct ErrorResponseHeader {
    /// The status code
    pub status: i64,

    /// The message
    pub message: Box<str>,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = &self.header.message;
        let status = self.header.status;

        write!(f, "{message} (status={status})",)
    }
}

impl std::error::Error for ErrorResponse {}
