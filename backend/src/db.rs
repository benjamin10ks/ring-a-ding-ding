use rusqlite::Connection;

pub struct Database {
    pub connection: Option<Connection>,
}

impl Database {
    pub fn new(name: &str) -> Self {
        Database {
            connection: Database::connect(name),
        }
    }

    fn connect(name: &str) -> Option<Connection> {
        match Connection::open(name) {
            Ok(conn) => Some(conn),
            Err(e) => {
                eprintln!("Failed to connect to database {}: {}", name, e);
                None
            }
        }
    }
}
