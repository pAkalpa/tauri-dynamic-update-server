use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UpdateContent {
    pub(crate) version: String,
    pub(crate) pub_date: String,
    pub(crate) url: String,
    pub(crate) signature: String,
    pub(crate) notes: String
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub github_token: String,
    pub github_repo_owner: String,
    pub github_repo_name: String,
    #[serde(default="default_github_api_url")]
    pub github_api_url: String,
    #[serde(default="default_home_redirect_url")]
    pub home_redirect_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub name: String,
    pub browser_download_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestReleaseResponse {
    pub tag_name: String,
    pub published_at: String,
    pub assets: Vec<Assets>,
    pub body: String
}

fn default_github_api_url() -> String {
    String::from("https://api.github.com")
}

fn default_home_redirect_url() -> String {
    String::from("https://github.com/pAkalpa")
}