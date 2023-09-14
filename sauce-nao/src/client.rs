use crate::ApiResponse;
use crate::Error;
use crate::Image;
use crate::OkResponse;
use std::sync::Arc;
use url::Url;

const DEFAULT_USER_AGENT_STR: &str =
    concat!(env!("CARGO_CRATE_NAME"), "/", env!("CARGO_PKG_VERSION"));
const SEARCH_URL: &str = "https://saucenao.com/search.php";

/// The sauce nao client
#[derive(Debug, Clone)]
pub struct Client {
    /// The inner http client.
    ///
    /// This generally should not be used directly.
    pub client: reqwest::Client,

    api_key: Arc<str>,
}

impl Client {
    /// Create a new [`Client`].
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(DEFAULT_USER_AGENT_STR)
                .build()
                .expect("failed to build saucenao client"),
            api_key: Arc::from(api_key),
        }
    }

    /// Look up an image
    pub async fn search(&self, image: impl Into<Image>) -> Result<OkResponse, Error> {
        let image = image.into();
        let mut url = Url::parse_with_params(
            SEARCH_URL,
            &[("output_type", "2"), ("api_key", &*self.api_key)],
        )?;

        let mut part = None;
        match image {
            Image::Url(image_url) => {
                url.query_pairs_mut().append_pair("url", &image_url);
            }
            Image::File { name, body } => {
                part = Some(reqwest::multipart::Part::stream(body).file_name(name));
            }
        }

        let mut request = self.client.post(url.as_str());
        if let Some(part) = part {
            let form = reqwest::multipart::Form::new().part("file", part);
            request = request.multipart(form);
        }
        let response = request.send().await?;

        // Don't check for status,
        // we trust the internal api response status code more.

        let response: ApiResponse = response.json().await?;

        Ok(response.into_result()?)
    }
}
