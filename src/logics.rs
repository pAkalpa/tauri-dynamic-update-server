use reqwest::Error;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::serde::json::Json;
use crate::models::{Arch, Configs, GetLatestReleaseResponse, LatestJSONContent, OSType, TargetPlatform, UpdateContent};
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

async fn extract_data_from_latest_json(response: &GetLatestReleaseResponse, platform: TargetPlatform) -> Option<Json<UpdateContent>> {
    let latest_json_url: Vec<String> = response.assets.iter().filter(|asset| {
        asset.name.eq_ignore_ascii_case("latest.json")
    }).map(|asset| asset.browser_download_url.clone()).collect();

    if &latest_json_url.len() > &0 {
        let config = envy::from_env::<Configs>().unwrap();
        let client = reqwest::Client::new();
        let response = client.get(&latest_json_url[0]).header(USER_AGENT, config.github_repo_owner).send().await.ok();
        let res = response?.json::<LatestJSONContent>().await;
        match res {
            Ok(re) => {
                if let Some(platform_info) = re.platforms.get(&platform.to_string()) {
                    Some(Json(UpdateContent {
                        version: re.version,
                        pub_date: re.pub_date,
                        url: platform_info.url.clone(),
                        signature: platform_info.signature.clone(),
                        notes: re.notes,
                    }))
                } else {
                    None
                }
            },
            Err(_e) => None
        }
    } else { None }
}

async fn get_signature(url: String) -> Result<String, Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

pub async fn get_release_data(version: &str, arch: &str, target: &str) -> Option<Json<UpdateContent>> {
    let response = get_latest_release_data().await;
    match response {
        Ok(res) => {
            if !is_older_version(version, &res.tag_name) {
                return None;
            }

            let target_os = target.parse::<OSType>().ok()?;
            let architecture = arch.parse::<Arch>().ok()?;

            match (target_os, architecture) {
                (OSType::Windows, Arch::X86_64) => extract_data_from_latest_json(&res, TargetPlatform::WindowsX86_64).await,
                (OSType::Linux, Arch::X86_64) => extract_data_from_latest_json(&res, TargetPlatform::LinuxX86_64).await,
                (OSType::Darwin, Arch::X86_64) => extract_data_from_latest_json(&res, TargetPlatform::DarwinX86_64).await,
                (OSType::Darwin, Arch::Aarch64) => extract_data_from_latest_json(&res, TargetPlatform::DarwinAarch64).await,
                _ => None,
            }
        },
        Err(_) => { None }
    }
}