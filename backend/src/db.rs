use rusqlite::Connection;

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new(name: &str) -> rusqlite::Result<Self> {
        Ok(Database {
            connection: Connection::open(name)?,
        })
    }
}
