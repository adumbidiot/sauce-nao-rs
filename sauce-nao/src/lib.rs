/// Client type
mod client;
/// Image type
mod image;
/// Api types
mod types;

pub use self::client::Client;
pub use self::image::Image;
pub use self::types::ApiResponse;
pub use self::types::Creator;
pub use self::types::ErrorResponse;
pub use self::types::OkResponse;
pub use self::types::ResultEntry;

/// The error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A reqwest error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// A URL parse error
    #[error("invalid url")]
    Url(#[from] url::ParseError),

    /// An API Error Occured
    #[error("api error")]
    Api(#[from] self::ErrorResponse),
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMAGE_PATH: &str = "./test_data/oZjCxGo.jpg";

    fn get_api_key() -> String {
        // Try env first
        match std::env::var_os("SAUCE_NAO_API_KEY") {
            Some(var) => var
                .into_string()
                .expect("`SAUCE_NAO_API_KEY` is not valid UTF-8"),
            None => {
                // Otherwise, try to load from the api_key.txt file.
                // The file should contain only the api key as UTF-8.
                std::fs::read_to_string("api_key.txt").expect("failed to load `api_key.txt`")
            }
        }
    }

    #[tokio::test]
    async fn search_url_works() {
        let client = Client::new(&get_api_key());

        let urls = [
            "https://i.imgur.com/oZjCxGo.jpg",
            "https://konachan.com/image/5982d8946ae503351e960f097f84cd90/Konachan.com%20-%20330136%20animal%20nobody%20original%20signed%20yutaka_kana.jpg",
        ];
        for url in urls {
            let results = client
                .search(url)
                .await
                .unwrap_or_else(|error| panic!("failed to search for `{url}`: {error:?}"));
            dbg!(results);
        }
    }

    #[tokio::test]
    async fn search_file_works() {
        let image = Image::from_path(IMAGE_PATH.as_ref())
            .await
            .expect("failed to open image");
        let client = Client::new(&get_api_key());
        let results = client.search(image).await.expect("failed to search");
        dbg!(results);
    }

    #[tokio::test]
    async fn invalid_client_fails() {
        let image = Image::from_path(IMAGE_PATH.as_ref())
            .await
            .expect("failed to open image");
        let client = Client::new("");
        let err = client
            .search(image)
            .await
            .expect_err("anonymous searching should fail");
        assert!(matches!(err, Error::Api(_)));
    }
}
