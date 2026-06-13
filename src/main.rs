mod api;
mod check;
mod display;
mod npm;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fleet-status", about = "Monitor all SuperInstance fleet services")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Show overview of all services (default)
    #[command(visible_alias("overview"))]
    Check { service: Option<String> },
    /// Show fleet-vector-api stats
    Api,
    /// List published crates
    Crates,
    /// Continuous monitoring (polls every 30s)
    Watch,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        None | Some(Commands::Check { service: None }) => {
            display::overview().await;
        }
        Some(Commands::Check { service: Some(svc) }) => {
            display::detailed_check(&svc).await;
        }
        Some(Commands::Api) => {
            display::api_stats().await;
        }
        Some(Commands::Crates) => {
            display::crates_list().await;
        }
        Some(Commands::Watch) => {
            display::watch_loop().await;
        }
    }
}
