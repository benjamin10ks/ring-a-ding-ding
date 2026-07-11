use crate::event::Event;
use rusqlite::{Connection, params};
use std::time::SystemTime;

pub trait MetadataStore {
    fn insert_event(&self, event: &Event) -> rusqlite::Result<()>;
    fn list_events(&self) -> rusqlite::Result<Vec<Event>>;
}

pub struct SqliteStore {
    connection: Connection,
}

impl SqliteStore {
    pub fn new(name: &str) -> rusqlite::Result<Self> {
        let connection = Connection::open(name)?;
        let store = Self { connection };
        store.run_migrations()?;
        Ok(store)
    }

    // Very basic just add name and sql to migrations arr
    fn run_migrations(&self) -> rusqlite::Result<()> {
        self.connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            applied_at INTEGER NOT NULL
        )",
        )?;

        let migrations: &[(&str, &str)] = &[(
            "0001_create_events",
            "CREATE TABLE events (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             camera_id TEXT NOT NULL,
             timestamp TEXT NOT NULL,
             clip_path TEXT NOT NULL,
             )",
        )];

        for (name, sql) in migrations {
            let already_applied: bool = self.connection.query_row(
                "SELECT COUNT(*) FROM _migrations WHERE name = ?1",
                params![name],
                |row| row.get::<_, i64>(0),
            )? > 0;

            if already_applied {
                continue;
            }

            self.connection
                .execute_batch(&format!("BEGIN; {} COMMIT;", sql))?;
            self.connection.execute(
                "INSERT INTO _migrations (name, applied_at) VALUES (?1, strftime('%s', 'now'))",
                params![
                    name,
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64
                ],
            )?;
        }

        Ok(())
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
