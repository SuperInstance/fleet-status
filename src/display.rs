use crate::api;
use crate::check::{self, HealthResult};
use crate::npm;
use colored::*;
use std::time::Duration;

struct ServiceDef {
    name: &'static str,
    url: &'static str,
}

static HTTP_SERVICES: &[ServiceDef] = &[
    ServiceDef { name: "fleet-vector-api", url: "https://fleet-vector-api.casey-digennaro.workers.dev/stats" },
    ServiceDef { name: "fleet-edge", url: "https://fleet-edge.casey-digennaro.workers.dev/" },
    ServiceDef { name: "superinstance-assets", url: "https://superinstance-assets.casey-digennaro.workers.dev/favicon.ico" },
    ServiceDef { name: "superinstance.ai", url: "https://superinstance.ai/" },
];

fn status_icon(ok: bool) -> String {
    if ok { "✅".to_string() } else { "❌".to_string() }
}

fn format_health(r: &HealthResult) -> String {
    let icon = status_icon(r.ok);
    let status_str = match r.status {
        Some(s) => format!("{} OK", s),
        None => "ERR".to_string(),
    };
    let latency = format!("({}ms)", r.latency_ms).dimmed();
    let err = match &r.error {
        Some(e) => format!(" — {}", e.chars().take(80).collect::<String>()).red().to_string(),
        None => String::new(),
    };
    format!(
        "{} {:<25} .... {} {}{}",
        icon, r.name, status_str, latency, err
    )
}

pub async fn overview() {
    let client = check::build_client();

    println!("{}", "SuperInstance Fleet Status".bold().cyan());
    println!("{}", "==========================".cyan());

    // HTTP checks
    let mut http_results: Vec<HealthResult> = Vec::new();
    for svc in HTTP_SERVICES {
        let r = check::check_endpoint(&client, svc.name, svc.url).await;
        println!("{}", format_health(&r));
        http_results.push(r);
    }

    // API extras
    let stats = check::check_api_stats(&client).await.ok();
    if let Some(s) = &stats {
        let vc = s.vector_count.map(|v| v.to_string()).unwrap_or_else(|| "?".into());
        let dim = s.dimensions.map(|d| d.to_string()).unwrap_or_else(|| "?".into());
        println!(
            "   vectors: {}, dimensions: {}",
            vc.green(),
            dim.green()
        );
    }

    println!();

    // npm packages
    for pkg in npm::NPM_PACKAGES {
        match npm::get_package_version(&client, pkg).await {
            Ok(ver) => println!("✅ {:<35} .... {} (npm)", pkg, ver.green()),
            Err(e) => println!("❌ {:<35} .... {} (npm)", pkg, e.red()),
        }
    }

    println!();
    print_conservation();
    println!();
    println!("Bottles in transit: {}", "0".dimmed());
}

pub async fn detailed_check(service: &str) {
    let client = check::build_client();

    let matches: Vec<&ServiceDef> = HTTP_SERVICES.iter().filter(|s| s.name.contains(service)).collect();
    if matches.is_empty() {
        println!("{} No service matching '{}'", "⚠".yellow(), service);
        return;
    }

    for svc in matches {
        println!("{}", format!("Checking {} ...", svc.name).bold());
        let r = check::check_endpoint(&client, svc.name, svc.url).await;
        println!("  URL:     {}", svc.url);
        println!("  Status:  {}", if r.ok { format!("{}", r.status.unwrap_or(0)).green() } else { format!("{:?}", r.status).red() });
        println!("  Latency: {}ms", r.latency_ms);
        if let Some(e) = &r.error {
            println!("  Error:   {}", e.red());
        }
        if let Some(d) = &r.detail {
            println!("  Body:    {}...", &d[..d.len().min(200)]);
        }
        println!();
    }
}

pub async fn api_stats() {
    let client = check::build_client();
    match api::get_stats(&client).await {
        Ok(stats) => {
            println!("{}", "fleet-vector-api Stats".bold().cyan());
            println!("{}", "=====================".cyan());
            println!("Index:      {}", stats.index_name.unwrap_or_else(|| "unknown".into()));
            println!("Vectors:    {}", stats.vector_count.unwrap_or(0));
            println!("Dimensions: {}", stats.dimensions.unwrap_or(0));
        }
        Err(e) => println!("{} Failed to get stats: {}", "❌".red(), e),
    }

    println!();
    println!("Running sample search...");
    match api::search(&client, "time client", 3).await {
        Ok(results) => println!("{}", serde_json::to_string_pretty(&results).unwrap_or_else(|_| "parse error".into())),
        Err(e) => println!("{} Search failed: {}", "❌".red(), e),
    }
}

pub async fn crates_list() {
    let client = check::build_client();
    let url = "https://crates.io/api/v1/crates?q=superinstance&per_page=100";
    println!("{}", "SuperInstance Crates".bold().cyan());
    println!("{}", "====================".cyan());

    match client.get(url).header("User-Agent", "fleet-status/0.1").send().await {
        Ok(resp) if resp.status().is_success() => {
            let body: serde_json::Value = resp.json().await.unwrap_or_default();
            if let Some(crates) = body["crates"].as_array() {
                for c in crates {
                    let name = c["name"].as_str().unwrap_or("?");
                    let ver = c["max_version"].as_str().unwrap_or("?");
                    let dl = c["downloads"].as_i64().unwrap_or(0);
                    println!("  {} {} ({} downloads)", name.green(), ver.dimmed(), dl);
                }
                println!("\nTotal: {} crates", crates.len());
            } else {
                println!("No crates found or unexpected response format.");
            }
        }
        Ok(resp) => println!("HTTP {}", resp.status()),
        Err(e) => println!("{} {}", "❌".red(), e),
    }
}

pub async fn watch_loop() {
    println!("{}", "fleet-status watch — polling every 30s (Ctrl+C to stop)".bold());
    println!();
    loop {
        let now = chrono::Local::now().format("%H:%M:%S");
        println!("{} [{}]", "---".dimmed(), now);
        overview().await;
        println!();
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

fn print_conservation() {
    // SuperInstance physics — conservation law display
    println!(
        "Conservation: {} {} {} {}",
        "γ=0.72".green(),
        "η=0.28".blue(),
        "C=1.00".yellow(),
        "drift=0.00".dimmed()
    );
}
