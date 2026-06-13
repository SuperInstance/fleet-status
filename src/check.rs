use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HealthResult {
    pub name: String,
    #[allow(dead_code)]
    pub url: String,
    pub status: Option<u16>,
    pub ok: bool,
    pub latency_ms: u64,
    pub detail: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiStatsResponse {
    pub vector_count: Option<i64>,
    pub dimensions: Option<i32>,
    #[allow(dead_code)]
    pub index_name: Option<String>,
}

pub async fn check_endpoint(client: &Client, name: &str, url: &str) -> HealthResult {
    let start = std::time::Instant::now();
    match client.get(url).timeout(Duration::from_secs(10)).send().await {
        Ok(resp) => {
            let latency = start.elapsed().as_millis() as u64;
            let status = resp.status().as_u16();
            let ok = resp.status().is_success();
            let body = resp.text().await.unwrap_or_default();
            HealthResult {
                name: name.to_string(),
                url: url.to_string(),
                status: Some(status),
                ok,
                latency_ms: latency,
                detail: if ok { None } else { Some(body.chars().take(200).collect()) },
                error: None,
            }
        }
        Err(e) => HealthResult {
            name: name.to_string(),
            url: url.to_string(),
            status: None,
            ok: false,
            latency_ms: start.elapsed().as_millis() as u64,
            detail: None,
            error: Some(e.to_string()),
        },
    }
}

pub async fn check_api_stats(client: &Client) -> Result<ApiStatsResponse, String> {
    let url = "https://fleet-vector-api.casey-digennaro.workers.dev/stats";
    let resp = client.get(url).timeout(Duration::from_secs(10)).send().await;
    match resp {
        Ok(r) if r.status().is_success() => r.json::<ApiStatsResponse>().await.map_err(|e| e.to_string()),
        Ok(r) => Err(format!("HTTP {}", r.status())),
        Err(e) => Err(e.to_string()),
    }
}

pub fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .expect("failed to build HTTP client")
}
