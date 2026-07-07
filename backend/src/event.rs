#[derive(serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: i32,
    pub timestamp: String,
    pub event_type: String,
    pub description: String,
}

impl Event {
    pub fn new(id: i32, timestamp: String, event_type: String, description: String) -> Self {
        Event {
            id,
            timestamp,
            event_type,
            description,
        }
    }
}
