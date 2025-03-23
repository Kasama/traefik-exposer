use std::sync::Arc;

use clap::Parser;
use tokio::sync::Mutex;

mod provider;
mod target;
mod web;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long, default_value = "0.0.0.0:3716", env = "EXPOSER_ADDR")]
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    let docker_client = Arc::new(Mutex::new(provider::docker::DockerProvider::new()?));
    let mut receiver = {
        docker_client.lock().await.watch_container_events(vec!["create".to_string(), "update".to_string(), "delete".to_string()])
    };

    let app = web::App::new(cli.addr, docker_client.clone())?;

    while let Some(_event) = receiver.recv().await {
        docker_client.lock().await.mark_dirty();
    }

    app.shutdown();

    Ok(())
}
