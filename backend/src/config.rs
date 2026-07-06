pub struct Config {
    pub port: u16,
    pub dbname: String,
}

impl Config {
    pub fn load() -> Self {
        // Load configuration from environment variables
        // TODO: or config file
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "6379".to_string())
            .parse::<u16>()
            .unwrap_or(6379);
        let dbname = std::env::var("DB_NAME").unwrap_or_else(|_| "my_database.db".to_string());
        Config { port, dbname }
    }
}
