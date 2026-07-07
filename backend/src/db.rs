use crate::event::Event;
use rusqlite::Connection;

pub trait MetadataStore {
    fn insert_event(&self, event: &Event) -> rusqlite::Result<()>;
    fn list_events(&self) -> rusqlite::Result<Vec<Event>>;
}

pub struct SqliteStore {
    connection: Connection,
}

impl SqliteStore {
    pub fn new(name: &str) -> rusqlite::Result<Self> {
        Ok(SqliteStore {
            connection: Connection::open(name)?,
        })
    }
}

impl MetadataStore for SqliteStore {
    fn insert_event(&self, event: &Event) -> rusqlite::Result<()> {
        self.connection.execute("", [])?;
        Ok(())
    }
    fn list_events(&self) -> rusqlite::Result<Vec<Event>> {
        Ok(vec![])
    }
}
