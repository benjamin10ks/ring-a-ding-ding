use crate::config::Config;
use crate::db::Database;
use tokio::net::TcpListener;

pub struct App {
    cfg: Config,
    db: Database,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        println!(
            "Initializing application with config: port={}, dbname={}",
            cfg.port, cfg.dbname
        );
        let db = Database::new(&cfg.dbname).expect("Failed to initialize database");
        App { cfg, db }
    }

    pub async fn run(&self) {
        let addr = format!("127.0.0.1:{}", self.cfg.port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        println!("Listening on {}", listener.local_addr().unwrap());

        loop {
            let (socket, _) = listener.accept().await.unwrap();

            println!("Accepted connection on {}", socket.peer_addr().unwrap());
            tokio::spawn(async move {});
        }
    }
}
