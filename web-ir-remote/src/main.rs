use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    extract::{Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use clap::Parser;
use serde::Deserialize;
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::watch,
};

mod config;
use config::{HBox, Node, VBox};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long, env)]
    #[clap(default_value = "0.0.0.0:3000")]
    listen: SocketAddr,

    #[clap(long, env)]
    #[clap(default_value = "255.255.255.255:6464")]
    target: SocketAddr,

    #[clap(long, env)]
    #[clap(default_value = "/etc/ir-remote.xml")]
    config: PathBuf,
}

#[derive(Clone)]
struct App {
    allowed_actions: Arc<HashSet<PathBuf>>,
    tx: watch::Sender<PathBuf>,
    config: &'static str,
}

#[derive(Clone, Deserialize)]
struct Action {
    action: PathBuf,
}

async fn act_handler(State(app): State<App>, action: Query<Action>) -> impl IntoResponse {
    if !app.allowed_actions.contains(&action.action) {
        return (StatusCode::BAD_REQUEST, "bad request");
    }

    app.tx.send(action.action.clone()).unwrap();

    (StatusCode::OK, "scheduled")
}

async fn xml_handler(State(app): State<App>) -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/xml")], app.config)
}

async fn root_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_bytes!("../assets/index.html"),
    )
}

fn find_allowed_actions_from_node(set: &mut HashSet<PathBuf>, node: &config::Node) {
    match node {
        Node::HBox(HBox { children }) | Node::VBox(VBox { children }) => {
            for child in children {
                find_allowed_actions_from_node(set, child);
            }
        }
        Node::Button(button) => {
            set.insert(button.action.clone());
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = std::fs::read_to_string(&cli.config)
        .expect("Failed to open config file")
        .leak();

    let config_parsed: config::Controllers =
        serde_xml_rs::from_str(config).expect("Failed to parse config file");

    let allowed_actions: Arc<HashSet<PathBuf>> = {
        let mut set = HashSet::new();

        for controller in &config_parsed.controllers {
            find_allowed_actions_from_node(&mut set, &controller.root_node);
        }

        Arc::new(set)
    };

    let base_path = {
        let mut path = cli.config.clone();
        path.pop();
        path
    };

    let (tx, mut rx) = watch::channel(PathBuf::new());

    tokio::spawn(async move {
        let mut cache: HashMap<PathBuf, Vec<u8>> = HashMap::new();

        let sock = match &cli.target {
            SocketAddr::V4(_) => UdpSocket::bind("0.0.0.0:0").await,
            SocketAddr::V6(_) => UdpSocket::bind("[::]:0").await,
        }
        .unwrap();

        loop {
            rx.changed().await.expect("Broken pipe");

            let path = rx.borrow().clone();

            let binary_ref = if cache.contains_key(&path) {
                cache.get(&path).unwrap()
            } else {
                let abs_path = if path.is_absolute() {
                    path.clone()
                } else {
                    base_path.join(&path)
                };

                let Ok(json) = std::fs::read_to_string(&abs_path) else {
                    eprintln!("Failed to read file {:?}", abs_path);
                    continue;
                };

                let Ok(packet) = serde_json::from_str::<model::Packet>(&json) else {
                    eprintln!("Illegal format {:?}", abs_path);
                    continue;
                };

                cache.insert(path.clone(), packet.encode());

                cache.get(&path).unwrap()
            };

            if let Err(e) = sock.send_to(binary_ref, &cli.target).await {
                eprintln!("Failed to send packet: {e:?}");
            };

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/xml", get(xml_handler))
        .route("/api/act", post(act_handler))
        .with_state(App {
            allowed_actions,
            tx,
            config,
        });

    let listener = TcpListener::bind(cli.listen).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
