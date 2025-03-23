use bollard::container::ListContainersOptions;
use bollard::secret::EventMessage;
use bollard::system::EventsOptions;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc::Receiver;
use tokio::task;

use crate::target::traefik::{
    HttpConfig, LoadBalancerConfig, RouterConfig, ServerConfig, ServiceConfig, TraefikConfig,
};

const LABEL_PREFIX: &str = "kasama.traefik-exposer.";

fn label_key(name: &str) -> String {
    format!("{}{}", LABEL_PREFIX, name)
}

pub struct DockerProvider {
    client: bollard::Docker,
    memory: Option<Vec<ContainerInfo>>,
    dirty: AtomicBool,
}

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    name: String,
    ip: String,
    labels: HashMap<String, String>,
}

impl From<Vec<ContainerInfo>> for TraefikConfig {
    fn from(container_infos: Vec<ContainerInfo>) -> Self {
        let mut routers = std::collections::HashMap::new();
        let mut services = std::collections::HashMap::new();
        let middlewares = std::collections::HashMap::new();

        for container in container_infos {
            println!("Processing container '{:?}'", container.name);
            if let Some(enabled) = container.labels.get(&label_key("enabled")) {
                if enabled == "true" {
                    let service_name = format!("{}-service", container.name);
                    let router_name = format!("{}-router", container.name);

                    let service = ServiceConfig::LoadBalancer(LoadBalancerConfig {
                        sticky: None,
                        servers: vec![ServerConfig {
                            url: format!(
                                "http://{}:{}",
                                container.ip,
                                container
                                    .labels
                                    .get(&label_key("port"))
                                    .unwrap_or(&"80".to_string())
                            ),
                            weight: 1,
                            preserve_path: true,
                        }],
                        health_check: None,
                        pass_host_header: None,
                        response_forwarding: None,
                        servers_transport: None,
                    });

                    let router_rule = container
                        .labels
                        .get(&label_key("rule"))
                        .unwrap_or(&"".to_string())
                        .clone();

                    if router_rule.is_empty() {
                        println!("Rule is empty for container '{:?}'. Please specify a rule with the label '{}'", container.name, label_key("rule"));
                        continue;
                    }

                    let router = RouterConfig {
                        entry_points: container
                            .labels
                            .get(&label_key("entrypoints"))
                            .map(|s| s.split(',').map(String::from).collect())
                            .unwrap_or_else(|| vec!["http".to_string()]),
                        middlewares: vec![],
                        service: service_name.clone(),
                        rule: router_rule,
                        rule_syntax: None,
                        priority: None,
                        tls: None,
                        observability: None,
                    };

                    services.insert(service_name, service);
                    routers.insert(router_name, router);
                } else {
                    println!(
                        "Container '{:?}' skipped because it is not enabled '{} = true'",
                        container.name,
                        label_key("enabled")
                    )
                }
            } else {
                println!(
                    "Container '{:?}' skipped because it is missing the label '{} = true'",
                    container.name,
                    label_key("enabled")
                )
            }
        }

        TraefikConfig {
            http: Some(HttpConfig {
                routers,
                services,
                middlewares,
            }),
        }
    }
}

impl DockerProvider {
    pub fn new() -> anyhow::Result<Self> {
        let client = bollard::Docker::connect_with_local_defaults()?;
        Ok(DockerProvider {
            client,
            memory: None,
            dirty: AtomicBool::new(false),
        })
    }

    pub fn mark_dirty(&self) {
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn get_exposable_containers_info(&mut self) -> anyhow::Result<Vec<ContainerInfo>> {
        if self.dirty.load(std::sync::atomic::Ordering::Relaxed) {
            self.memory = None;
        }
        if let Some(ref memory) = self.memory {
            println!("responding from memory");
            return Ok(memory.to_vec());
        }

        let options = Some(ListContainersOptions::<String> {
            all: false,
            ..Default::default()
        });

        let containers = self.client.list_containers(options).await?;

        let mut container_info_list = Vec::new();

        println!("Found {} containers", containers.len());
        for container in containers {
            let labels: HashMap<String, String> = container
                .labels
                .unwrap_or_default()
                .into_iter()
                .filter(|(k, _v)| k.starts_with(LABEL_PREFIX))
                .collect();
            let name = container
                .names
                .unwrap_or_default()
                .first()
                .cloned()
                .unwrap_or_default();
            let network_settings = container.network_settings.unwrap_or_default();
            let networks = network_settings.networks.unwrap_or_default();
            let ip = networks
                .values()
                .next()
                .map(|n| n.ip_address.clone())
                .unwrap_or_default();

            container_info_list.push(ContainerInfo {
                name,
                ip: ip.unwrap_or_default(),
                labels,
            });
        }

        self.memory = Some(container_info_list.clone());
        self.dirty
            .store(false, std::sync::atomic::Ordering::Relaxed);

        Ok(container_info_list)
    }

    pub fn watch_container_events(&self, actions: Vec<String>) -> Receiver<EventMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        let client = self.client.clone();
        task::spawn(async move {
            let mut events_stream = client.events::<String>(Some(EventsOptions {
                since: Some(chrono::Utc::now()),
                until: None,
                filters: Default::default(),
            }));

            while let Some(event_result) = events_stream.next().await {
                match event_result {
                    Ok(event) => {
                        if let Some(ref action) = event.action {
                            if actions.contains(action) && tx.send(event).await.is_err() {
                                eprintln!("Receiver dropped");
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving event: {:?}", e);
                    }
                }
            }
        });

        rx
    }
}
