use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Seed {
    /// Readable Name
    #[serde(rename = "name")]
    pub name: String,

    /// Description
    #[serde(rename = "description")]
    pub description: String,

    /// Homepage or Website URL
    #[serde(rename = "homepage")]
    pub homepage: String,

    // The '.app' name
    #[serde(rename = "app_name")]
    pub app_name: String,

    /// Application Version
    #[serde(rename = "app_version")]
    pub app_version: String,

    /// Minimum macOS Version
    #[serde(rename = "min_os")]
    pub min_os: String,

    /// Download URLs
    #[serde(rename = "downloads")]
    pub downloads: Downloads,

    /// Files to remove on uninstall
    #[serde(rename = "uninstall")]
    pub uninstall: Uninstall,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Downloads {
    /// Architecture arm64 (M1, M2, Etc)
    #[serde(rename = "arm64")]
    pub arm64: String,

    /// Architecture amd64 (Intel, AMD (if hackintosh), etc)
    #[serde(rename = "amd64")]
    pub amd64: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Uninstall {
    /// Files to remove
    #[serde(rename = "files")]
    pub files: Vec<String>,

    /// Directories to remove
    #[serde(rename = "directories")]
    pub directories: Vec<String>,

    /// Launchctl entries to remove
    #[serde(rename = "launchctl")]
    pub launchctl: Vec<String>,

    /// Other files/cache/etc to remove
    #[serde(rename = "other")]
    pub other: Vec<String>,
}

pub fn load_seed(seed: &str) -> Seed {
    let seed_decode: Seed = serde_json::from_str(seed).expect("Invalid seed file.");

    return seed_decode;
}
