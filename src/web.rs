use std::sync::Arc;
use tokio::net::ToSocketAddrs;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::sync::Mutex;

use crate::provider::docker::DockerProvider;
use crate::target::traefik::TraefikConfig;

pub struct App {
    shutdown_signal: tokio::sync::oneshot::Sender<()>,
}

struct AppState {
    docker_client: Arc<Mutex<DockerProvider>>,
}

async fn get_traefik_config(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let traefik_cfg = state.docker_client.lock().await.get_exposable_containers_info().await.unwrap();
    let cfg = serde_json::to_string::<TraefikConfig>(&traefik_cfg.into()).unwrap();

    ([("Content-Type", "application/json")], cfg)
}

impl App {
    pub fn new<A: ToSocketAddrs + 'static + Send>(addr: A, docker_client: Arc<Mutex<DockerProvider>>) -> anyhow::Result<Self> {
        let state = Arc::new(AppState {
            docker_client,
        });

        let router = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/traefik", get(get_traefik_config))
            .with_state(state.clone());

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router)
                .with_graceful_shutdown(async move { shutdown_rx.await.unwrap_or_default() })
                .await?;

            Ok(()) as anyhow::Result<()>
        });

        Ok(App {
            shutdown_signal: shutdown_tx,
        })
    }

    pub fn shutdown(self) {
        let _ = self.shutdown_signal.send(());
    }
}
