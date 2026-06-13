use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NpmPackage {
    #[allow(dead_code)]
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: Option<DistTags>,
    #[allow(dead_code)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DistTags {
    pub latest: Option<String>,
}

pub async fn get_package_version(client: &Client, name: &str) -> Result<String, String> {
    let url = format!("https://registry.npmjs.org/{}", name);
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        let pkg: NpmPackage = resp.json().await.map_err(|e| e.to_string())?;
        pkg.dist_tags
            .and_then(|t| t.latest)
            .ok_or_else(|| "no latest version".to_string())
    } else {
        Err(format!("HTTP {} — package not found", resp.status()))
    }
}

pub static NPM_PACKAGES: &[&str] = &[
    "@superinstance/tminus-client",
    "@superinstance/tminus-dispatcher",
];
