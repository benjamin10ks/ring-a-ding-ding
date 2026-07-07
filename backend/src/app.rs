use crate::api::{self, ApiState};
use crate::config::Config;
use crate::db::SqliteStore;
use crate::video_store::DiskVideoStore;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub struct App {
    cfg: Config,
    metadata: Arc<Mutex<SqliteStore>>,
    video: Arc<DiskVideoStore>,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        println!(
            "Initializing application with config: pi_port={}, api_port={}, dbname={}",
            cfg.pi_port, cfg.api_port, cfg.dbname
        );
        let metadata = SqliteStore::new(&cfg.dbname).expect("Failed to initialize database");
        let video = DiskVideoStore::new(&cfg.video_path).expect("Failed to initialize video store");

        App {
            cfg,
            metadata: Arc::new(Mutex::new(metadata)),
            video: Arc::new(video),
        }
    }

    pub async fn run(&self) {
        tokio::join!(self.run_pi_ingestor(), self.run_http_server());
    }

    /// Raw TCP stream from the Pi camera node — not HTTP, this is the video ingest path.
    async fn run_pi_ingestor(&self) {
        let addr = format!("127.0.0.1:{}", self.cfg.pi_port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        println!(
            "Pi stream ingestor listening on {}",
            listener.local_addr().unwrap()
        );

        loop {
            let (socket, _) = listener.accept().await.unwrap();

            println!(
                "Accepted Pi connection from {}",
                socket.peer_addr().unwrap()
            );
            tokio::spawn(async move {});
        }
    }

    /// HTTP API for the client application (event feed, live feed, archive, push).
    async fn run_http_server(&self) {
        let state = ApiState {
            metadata: self.metadata.clone(),
        };
        let router = api::router(state);

        let addr = format!("127.0.0.1:{}", self.cfg.api_port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        println!("HTTP API listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, router).await.unwrap();
    }
}
