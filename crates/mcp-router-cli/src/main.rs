use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use config::{Config, File};
use mcp_router_transport::client::manager::ClientManager;

mod app_config;
use app_config::{AppConfig, DownstreamConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "router.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let settings = Config::builder()
        .add_source(File::from(args.config))
        .build()?;

    let app_config: AppConfig = settings.try_deserialize()?;
    let client_manager = ClientManager::new();

    // Iterate over all the downstreams and start them
    for (id, downstream) in app_config.downstreams {
        match downstream {
            DownstreamConfig::Stdio { command, args, .. } => {
                tracing::info!("Connecting to downstream '{}': {} {:?}", id, command, args);
                client_manager.spawn_client(&command, &args).await?;

                break;
            }
        }
    }

    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down");

    Ok(())
}
