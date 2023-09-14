/// [`ResultEntry`] types
pub mod result_entry;

pub use self::result_entry::ResultEntry;
use crate::types::ApiResponseHeader;
use std::{collections::HashMap, marker::PhantomData, str::FromStr};

/// A JSON search result
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchJson {
    /// Header
    pub header: (),

    /// Results
    pub results: Vec<ResultEntry>,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}
