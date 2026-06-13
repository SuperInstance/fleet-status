use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StatsResponse {
    pub vector_count: Option<i64>,
    pub dimensions: Option<i32>,
    pub index_name: Option<String>,
}

pub async fn get_stats(client: &Client) -> Result<StatsResponse, String> {
    let url = "https://fleet-vector-api.casey-digennaro.workers.dev/stats";
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        resp.json::<StatsResponse>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}", resp.status()))
    }
}

pub async fn search(client: &Client, query: &str, top_k: usize) -> Result<serde_json::Value, String> {
    let url = "https://fleet-vector-api.casey-digennaro.workers.dev/search";
    let body = serde_json::json!({ "query": query, "topK": top_k });
    let resp = client.post(url).json(&body).send().await.map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        resp.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}", resp.status()))
    }
}
