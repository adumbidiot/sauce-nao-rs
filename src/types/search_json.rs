/// [`ResultEntry`] types
pub mod result_entry;

pub use self::result_entry::ResultEntry;
use crate::types::ApiResponseHeader;
use std::{collections::HashMap, marker::PhantomData, str::FromStr};

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

    /// The number of requests that can be made in the short limit.
    ///
    /// This is currently 4 for free accounts.
    /// The short limit is currently 30 seconds.
    #[serde(deserialize_with = "de_from_str", serialize_with = "ser_int_to_str")]
    pub short_limit: u8,

    /// The number of requests that can be made in the long limit.
    ///
    /// This is currently 100 for free accounts.
    /// The long limit is currently 24 hours.
    #[serde(deserialize_with = "de_from_str", serialize_with = "ser_int_to_str")]
    pub long_limit: u8,

    /// The number of request remaining for the long period.
    ///
    /// The long period is currently 24 hours.
    pub long_remaining: u64,

    /// The minimum of the number of remaining requests for the short and long periods.
    ///
    /// The short period is currently 30 seconds.
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

fn de_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    struct FromStrVisitor<T>(PhantomData<T>);

    impl<'de, T> serde::de::Visitor<'de> for FromStrVisitor<T>
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                formatter,
                "a string that can be parsed into a {}",
                std::any::type_name::<T>()
            )
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Self::Value::from_str(value).map_err(serde::de::Error::custom)
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Self::Value::from_str(&value).map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_str(FromStrVisitor(PhantomData))
}

fn ser_int_to_str<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: itoa::Integer,
    S: serde::Serializer,
{
    let mut buffer = itoa::Buffer::new();
    let value = buffer.format(*value);
    serializer.serialize_str(value)
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
