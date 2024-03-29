use anyhow::bail;
use anyhow::Context;
use reqwest::Url;
use sauce_nao::Creator;
use sauce_nao::Image;
use std::str::FromStr;

/// The output format
#[derive(Debug, Default)]
pub enum OutputFormat {
    #[default]
    Human,

    Json,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "human" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            input => bail!("unknown output format \"{input}\""),
        }
    }
}

/// User config
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// The api key
    pub api_key: Option<String>,
}

impl Config {
    /// Make an empty config
    pub fn new() -> Self {
        Self { api_key: None }
    }

    /// Load the config from the data dir, or return an empty copy if it does not exist
    pub fn load() -> anyhow::Result<Self> {
        let config_dir = dirs_next::config_dir().context("missing config dir")?;
        let config_file_path = config_dir.join("sauce_nao/config.toml");
        let file = match std::fs::read_to_string(config_file_path) {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::new()),
            Err(e) => return Err(e).context("failed to read config"),
        };
        let config: Self = toml::from_str(&file).context("failed to parse config")?;
        Ok(config)
    }

    /// Call `Self::load` using the tokio threadpool
    pub async fn load_async() -> anyhow::Result<Self> {
        tokio::task::spawn_blocking(Self::load)
            .await
            .context("failed to join task")?
    }

    /// Save the config
    pub fn save(&self) -> anyhow::Result<()> {
        let data = toml::to_string(self).context("failed to serialize config")?;
        let config_dir = dirs_next::config_dir().context("missing config dir")?;
        let sauce_nao_config_dir = config_dir.join("sauce_nao");
        std::fs::create_dir_all(&sauce_nao_config_dir)
            .context("failed to create sauce_nao config dir")?;
        let config_file_path = sauce_nao_config_dir.join("config.toml");
        std::fs::write(config_file_path, data).context("failed to save to file")?;
        Ok(())
    }

    /// Call `Self::save` using the tokio threadpool
    pub async fn save_async(&self) -> anyhow::Result<()> {
        let self_clone = self.clone();
        tokio::task::spawn_blocking(move || self_clone.save())
            .await
            .context("failed to join task")?
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(argh::FromArgs)]
#[argh(description = "a tool to look up images on sauce nao")]
pub struct Options {
    #[argh(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
pub enum SubCommand {
    Login(LoginOptions),
    Search(SearchOptions),
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "login", description = "login to sauce nao")]
pub struct LoginOptions {
    #[argh(option, description = "the api key", long = "api-key", short = 'k')]
    pub api_key: Option<String>,
}

#[derive(argh::FromArgs, Debug)]
#[argh(
    subcommand,
    name = "search",
    description = "search for an image on sauce nao"
)]
pub struct SearchOptions {
    #[argh(positional, description = "the image url or path")]
    pub url: String,

    #[argh(
        option,
        long = "output-format",
        description = "the output format",
        default = "Default::default()"
    )]
    pub output_format: OutputFormat,

    #[argh(
        switch,
        long = "proxy-file",
        description = "whether to download the given url and upload as a file"
    )]
    pub proxy_file: bool,
}

/// The entry point
fn main() -> anyhow::Result<()> {
    // This must be called first as if it fails, it will exit the program without running destructors.
    let options: Options = argh::from_env();
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("failed to start tokio runtime")?;
    tokio_rt.block_on(async_main(options))?;
    Ok(())
}

/// The async entry point
async fn async_main(options: Options) -> anyhow::Result<()> {
    let mut config = Config::load_async()
        .await
        .context("failed to load config")?;

    match options.subcommand {
        SubCommand::Login(options) => {
            if options.api_key.is_some() {
                config.api_key = options.api_key;
            }

            config.save_async().await.context("failed to save")?;

            eprintln!("Updated config.");
        }
        SubCommand::Search(options) => {
            let api_key = config.api_key.as_deref().context("missing api_key")?;
            let client = sauce_nao::Client::new(api_key);

            let image = if options.url.starts_with("http") {
                if !options.proxy_file {
                    Image::from(options.url.as_str())
                } else {
                    let maybe_url = Url::parse(&options.url);
                    let file_name = maybe_url
                        .as_ref()
                        .ok()
                        .and_then(|url| url.path_segments()?.next_back()?.split('.').next())
                        .unwrap_or("file.png");

                    eprintln!("Proxying image...");
                    let response = async {
                        client
                            .client
                            .get(options.url.as_str())
                            .send()
                            .await?
                            .error_for_status()
                    }
                    .await
                    .with_context(|| format!("failed to send request to \"{}\"", options.url))?;
                    let body = reqwest::Body::wrap_stream(response.bytes_stream());

                    Image::File {
                        name: file_name.into(),
                        body,
                    }
                }
            } else {
                eprintln!("Loading image...");
                Image::from_path(options.url.as_ref())
                    .await
                    .context("failed to load image")?
            };

            eprintln!("Searching...");
            let results = client.search(image).await.context("failed to search")?;

            match options.output_format {
                OutputFormat::Human => {
                    println!();
                    println!("Results for search: ");
                    println!();

                    for (i, result) in results.results.iter().enumerate() {
                        println!("{})", i + 1);
                        println!("Similarity: {}", result.header.similarity);
                        println!("Thumbnail: {}", result.header.thumbnail.as_str());
                        println!("Index name: {}", result.header.index_name);
                        if !result.data.ext_urls.is_empty() {
                            println!("Ext Urls:");

                            for url in result.data.ext_urls.iter() {
                                println!("    {}", url.as_str());
                            }
                        }
                        if let Some(author_name) = result.data.author_name.as_deref() {
                            println!("Author Name: {}", author_name);
                        }
                        if let Some(creator) = result.data.creator.as_ref() {
                            match creator {
                                Creator::Single(creator) => {
                                    println!("Creator: {}", creator);
                                }
                                Creator::Multiple(creators) => {
                                    println!("Creators: ");
                                    for creator in creators.iter() {
                                        println!("    {}", creator);
                                    }
                                }
                            }
                        }
                        if let Some(member_name) = result.data.member_name.as_deref() {
                            println!("Member Name: {}", member_name);
                        }
                        println!();
                    }
                }
                OutputFormat::Json => {
                    let stdout = std::io::stdout();
                    let stdout = stdout.lock();

                    serde_json::to_writer(stdout, &results)?;
                }
            }
        }
    }

    Ok(())
}
