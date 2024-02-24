use reqwest::Error;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::serde::json::Json;
use crate::models::{Configs, GetLatestReleaseResponse, UpdateContent};
use version_compare::{Version};

fn is_older_version(request_version: &str, release_version: &str) -> bool {
    let request_version = Version::from(request_version).unwrap();
    let release_version = Version::from(release_version).unwrap();
    return if request_version < release_version { true } else { false }
}

async fn get_latest_release_data() -> Result<GetLatestReleaseResponse, Error> {
    let config = envy::from_env::<Configs>().unwrap();
    let url = format!("{}/repos/{}/{}/releases/latest", config.github_api_url, config.github_repo_owner, config.github_repo_name);
    let auth_token = format!("Bearer {}", config.github_token);
    let client = reqwest::Client::new();
    let response = client.get(url).header(AUTHORIZATION, auth_token).header(ACCEPT, "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28").header(USER_AGENT, config.github_repo_owner).send().await?;
    let res = response.json().await?;
    Ok(res)
}

fn extract_url(response: &GetLatestReleaseResponse, category: &str) -> Vec<String> {
    response.assets.iter().filter(|asset| {
        asset.name.ends_with(category)
    }).map(|asset| asset.browser_download_url.clone()).collect()
}

async fn get_signature(url: String) -> Result<String, Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

pub async fn windows_logic(version: &str) -> Option<Json<UpdateContent>> {
    let response = get_latest_release_data().await.unwrap();
    if is_older_version(version, response.tag_name.as_str()) {
        let zip_url = extract_url(&response, ".msi.zip")[0].to_owned();
        let sig_url = extract_url(&response, ".msi.zip.sig")[0].to_owned();
        let signature = get_signature(sig_url).await.unwrap();
        Some(Json(UpdateContent {
            version: response.tag_name,
            pub_date: response.published_at,
            url: zip_url,
            signature,
            notes: response.body
        }))
    } else {
        None
    }
}