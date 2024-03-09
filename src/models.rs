use serde::{Deserialize, Serialize};
use std::{fmt::{Formatter, Display}, collections::HashMap, str::FromStr};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformInfo {
    pub signature: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestJSONContent {
    pub version: String,
    pub notes: String,
    pub pub_date: String,
    pub platforms: HashMap<String, PlatformInfo>,
}

#[derive(Debug, PartialEq)]
pub enum TargetPlatform {
    DarwinX86_64,
    DarwinAarch64,
    LinuxX86_64,
    WindowsX86_64
}

#[derive(Debug, PartialEq)]
pub enum Arch {
    X86_64,
    Aarch64,
    I686,
    Armv7
}

#[derive(Debug, PartialEq)]
pub enum OSType {
    Windows,
    Linux,
    Darwin
}

impl FromStr for OSType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "windows" => Ok(OSType::Windows),
            "linux" => Ok(OSType::Linux),
            "darwin" => Ok(OSType::Darwin),
            _ => Err(())
        }
    }
}

impl FromStr for Arch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86_64" => Ok(Arch::X86_64),
            "aarch64" => Ok(Arch::Aarch64),
            "i686" => Ok(Arch::I686),
            "armv7" => Ok(Arch::Armv7),
            _ => Err(())
        }
    }
}

impl Display for TargetPlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            TargetPlatform::DarwinX86_64 => write!(f, "darwin-x86_64"),
            TargetPlatform::DarwinAarch64 => write!(f, "darwin-aarch64"),
            TargetPlatform::LinuxX86_64 => write!(f, "linux-x86_64"),
            TargetPlatform::WindowsX86_64 => write!(f, "windows-x86_64")
        }
    }
}

fn default_github_api_url() -> String {
    String::from("https://api.github.com")
}

fn default_home_redirect_url() -> String {
    String::from("https://github.com/pAkalpa")
}