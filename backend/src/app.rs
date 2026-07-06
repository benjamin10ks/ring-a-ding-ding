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
        App {
            cfg,
            db: Database::new("my_database.db"),
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

        println!("Listening on {}", listener.local_addr().unwrap());

        loop {
            let (socket, _) = listener.accept().await.unwrap();

            println!("Accepted connection on {}", socket.peer_addr().unwrap());
            tokio::spawn(async move {});
        }
    }
}
