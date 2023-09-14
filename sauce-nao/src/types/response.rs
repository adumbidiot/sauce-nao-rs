mod error;
mod ok;

pub use self::error::ErrorResponse;
pub use self::error::ErrorResponseHeader;
pub use self::ok::Creator;
pub use self::ok::IndexEntry;
pub use self::ok::OkResponse;
pub use self::ok::OkResponseHeader;
pub use self::ok::ResultEntry;
use std::collections::HashMap;

/// An Api Response
#[derive(Debug)]
pub enum ApiResponse {
    /// The normal response
    Ok(OkResponse),

    /// There was a client-side error
    Error(ErrorResponse),
}

impl<'de> serde::Deserialize<'de> for ApiResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::Unexpected;
        use serde_json::Value;

        let mut value = HashMap::<Box<str>, serde_json::Value>::deserialize(deserializer)?;

        let header = value
            .remove("header")
            .ok_or(D::Error::missing_field("header"))?;
        let status = header
            .get("status")
            .ok_or(D::Error::missing_field("status"))?;

        let expected = &"an integer";
        let status = match status {
            Value::Null => {
                return Err(D::Error::invalid_type(Unexpected::Option, expected));
            }
            Value::Bool(value) => {
                return Err(D::Error::invalid_type(Unexpected::Bool(*value), expected));
            }
            Value::String(value) => {
                return Err(D::Error::invalid_type(Unexpected::Str(value), expected));
            }
            Value::Array(_value) => {
                return Err(D::Error::invalid_type(Unexpected::Seq, expected));
            }
            Value::Object(_value) => {
                return Err(D::Error::invalid_type(Unexpected::Map, expected));
            }
            Value::Number(value) => value,
        };

        let status = status.as_i64().ok_or_else(|| {
            D::Error::invalid_type(
                serde::de::Unexpected::Other("json value type"),
                &"an integer",
            )
        })?;

        if status == 0 {
            let results = value
                .remove("results")
                .ok_or(D::Error::missing_field("results"))?
                .take();

            Ok(Self::Ok(OkResponse {
                header: serde_json::from_value(header).map_err(D::Error::custom)?,
                results: serde_json::from_value(results).map_err(D::Error::custom)?,
                extra: value,
            }))
        } else {
            Ok(Self::Error(ErrorResponse {
                header: serde_json::from_value(header).map_err(D::Error::custom)?,
                extra: value,
            }))
        }
    }
}

impl ApiResponse {
    /// Transform this into a `Result`.
    pub fn into_result(self) -> Result<OkResponse, ErrorResponse> {
        match self {
            Self::Ok(response) => Ok(response),
            Self::Error(response) => Err(response),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("../../test_data/sample.json");
    const IMGUR: &str = include_str!("../../test_data/imgur.json");

    #[test]
    fn parse_search_json() {
        let response: ApiResponse = serde_json::from_str(SAMPLE).expect("failed to parse");
        let response = response.into_result().expect("response is not ok");
        for result in response.results.iter() {
            for extra in result.data.extra.iter() {
                panic!("unknown data: {extra:#?}");
            }
        }
    }

    #[test]
    fn parse_imgur_json() {
        let response: ApiResponse = serde_json::from_str(IMGUR).expect("failed to parse");
        let response = response.into_result().expect("response is not ok");
        for result in response.results.iter() {
            for extra in result.data.extra.iter() {
                panic!("unknown data: {extra:#?}");
            }
        }
    }
}
