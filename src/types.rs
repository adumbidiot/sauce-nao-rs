/// [`SearchJson`] types
pub mod search_json;

pub use self::search_json::SearchJson;
use std::collections::HashMap;

/// An API Error
#[derive(Debug, serde::Deserialize)]
pub struct ApiError {
    /// Header?
    pub header: ApiErrorHeader,

    /// Extra
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// The API Error header
#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorHeader {
    /// The message
    pub message: Box<str>,

    /// The status
    pub status: i32,

    /// Extra
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
