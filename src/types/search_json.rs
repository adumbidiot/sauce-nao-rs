/// [`ResultEntry`] types
pub mod result_entry;

pub use self::result_entry::ResultEntry;
use crate::types::ApiResponseHeader;
use std::collections::HashMap;

/// A JSON search result
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchJson {
    /// Header
    pub header: ApiResponseHeader<HeaderPayload>,

    /// Results
    pub results: Vec<ResultEntry>,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// Search json result header
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HeaderPayload {
    /// user id?
    pub user_id: Box<str>,

    /// account type?
    pub account_type: Box<str>,

    /// Short limit?
    pub short_limit: Box<str>,

    /// Long limit?
    pub long_limit: Box<str>,

    /// long remaining?
    pub long_remaining: u64,

    /// short remaining
    pub short_remaining: u64,

    /// results requested?
    ///
    /// This may be a string or a number
    pub results_requested: serde_json::Value,
    /// index?
    pub index: HashMap<Box<str>, IndexEntry>,
    /// search depth?
    pub search_depth: Box<str>,
    /// minimum similarity?
    pub minimum_similarity: f64,
    /// a path to the image maybe?
    pub query_image_display: Box<str>,
    /// the query image name
    pub query_image: Box<str>,

    /// The number of results returned
    pub results_returned: u64,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// An entry in the header index
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IndexEntry {
    /// status?
    pub status: u64,
    /// parent id?
    pub parent_id: u64,
    /// id?
    pub id: u64,
    /// results?
    pub results: u64,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("../../test_data/sample.json");
    const IMGUR: &str = include_str!("../../test_data/imgur.json");

    #[test]
    fn parse_search_json() {
        let res: SearchJson = serde_json::from_str(SAMPLE).expect("failed to parse");
        dbg!(&res);

        for result in res.results.iter() {
            for extra in result.data.extra.iter() {
                panic!("unknown data: {:#?}", extra);
            }
        }
    }

    #[test]
    fn parse_imgur_json() {
        let res: SearchJson = serde_json::from_str(IMGUR).expect("failed to parse");
        dbg!(&res);

        for result in res.results.iter() {
            for extra in result.data.extra.iter() {
                panic!("unknown data: {:#?}", extra);
            }
        }
    }
}
