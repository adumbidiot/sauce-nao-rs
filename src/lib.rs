/// Client type
pub mod client;
/// Image type
pub mod image;
/// Api types
pub mod types;

pub use self::{
    client::Client,
    image::Image,
    types::SearchJson,
};

/// The error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A reqwest error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// A URL parse error
    #[error("invalid url")]
    Url(#[from] url::ParseError),
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
        let results = client
            .search("https://i.imgur.com/oZjCxGo.jpg")
            .await
            .expect("failed to search");
        dbg!(results);
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
}
