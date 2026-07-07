pub struct Config {
    pub pi_port: u16,
    pub api_port: u16,
    pub dbname: String,
    pub video_path: String,
}

impl Config {
    pub fn load() -> Self {
        // Load configuration from environment variables
        // TODO: or config file
        let pi_port = std::env::var("PI_PORT")
            .unwrap_or_else(|_| "6379".to_string())
            .parse::<u16>()
            .unwrap_or(6379);
        let api_port = std::env::var("API_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);
        let dbname = std::env::var("DB_NAME").unwrap_or_else(|_| "my_database.db".to_string());
        let video_path = std::env::var("VIDEO_PATH").unwrap_or_else(|_| "./videos".to_string());
        Config {
            pi_port,
            api_port,
            dbname,
            video_path,
        }
    }
}
