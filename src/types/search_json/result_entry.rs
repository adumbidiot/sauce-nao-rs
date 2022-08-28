use std::collections::HashMap;
use url::Url;

/// A result entry
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultEntry {
    /// Header
    pub header: Header,
    /// Data
    pub data: Data,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// A result entry header
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Header {
    /// Result similarity
    pub similarity: Box<str>,
    /// Image thumbnail
    pub thumbnail: Url,
    /// index id?
    pub index_id: u64,
    /// The index name
    pub index_name: Box<str>,
    /// the # of dupes
    pub dupes: u64,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// Result data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Data {
    /// ?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ext_urls: Vec<Url>,
    /// title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Deviantart ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub da_id: Option<Box<str>>,
    /// Author name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<Box<str>>,
    /// Author URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<Url>,
    /// Pixiv id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixiv_id: Option<u64>,
    /// member name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<Box<str>>,
    /// member id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<u64>,
    /// source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<str>>,
    /// ?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anidb_aid: Option<u64>,
    ///?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<Box<str>>,
    /// anime year?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Box<str>>,
    /// ?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_time: Option<Box<str>>,
    /// fur affinity id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fa_id: Option<u64>,
    /// twitter tweet creation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Box<str>>,
    /// tweet_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_id: Option<Box<str>>,
    /// twitter_user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_user_id: Option<Box<str>>,
    /// twitter_user_handle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_user_handle: Option<Box<str>>,
    /// creator?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// eng name?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eng_name: Option<Box<str>>,
    /// jp name?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_name: Option<Box<str>>,
    /// bcy_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcy_id: Option<u64>,
    /// member_link_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_link_id: Option<u64>,
    /// bcy_type?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcy_type: Option<Box<str>>,
    /// fn_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_id: Option<u64>,
    /// fn_type?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_type: Option<Box<str>>,
    /// pawoo_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pawoo_id: Option<u64>,
    /// pawoo_user_acct?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pawoo_user_acct: Option<Box<str>>,
    /// pawoo_user_username?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pawoo_user_username: Option<Box<str>>,
    /// pawoo_user_display_name?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pawoo_user_display_name: Option<Box<str>>,
    /// seiga_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seiga_id: Option<u64>,
    /// danbooru_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danbooru_id: Option<u64>,
    /// material?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Box<str>>,
    /// characters?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Box<str>>,
    /// konachan_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konachan_id: Option<u64>,
    /// drawr_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawr_id: Option<u64>,
    /// gelbooru_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gelbooru_id: Option<u64>,
    /// sankaku_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankaku_id: Option<u64>,
    /// artist?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<Box<str>>,
    /// author?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<str>>,
    /// md_id?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_id: Option<Box<str>>,

    /// Extra K/Vs
    #[serde(flatten)]
    pub extra: HashMap<Box<str>, serde_json::Value>,
}

/// The creator field of [`Data`]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Creator {
    /// a single creator
    Single(Box<str>),

    /// multiple creators
    Multiple(Vec<Box<str>>),
}
