/// An [`ApiResponse`].
pub mod response;

pub use self::response::ApiResponse;
pub use self::response::IndexEntry;
pub use self::response::OkResponse;
pub use self::response::OkResponseHeader;
pub use self::response::ResultEntry;
use std::collections::HashMap;

/// The response header
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponseHeader<P> {
    /// The status code
    pub status: i32,

    /// The header payload
    ///
    /// This contains specialized fields that are not sent for every api request.
    #[serde(flatten)]
    pub payload: P,
}

/// An API Error
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiError {
    /// The response header
    pub header: ApiResponseHeader<ApiErrorHeaderPayload>,

    /// Extra
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (status={})",
            self.header.payload.message, self.header.status
        )
    }
}

impl std::error::Error for ApiError {}

/// The payload of an api error header
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiErrorHeaderPayload {
    /// The message
    pub message: Box<str>,

    /// Extra
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}
