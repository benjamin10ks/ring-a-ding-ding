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
    pub fn validate(&self) -> Result<(), String> {
        if self.event_type.is_empty() {
            return Err("Event type cannot be empty".to_string());
        }
        if self.description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        Ok(())
    }
}
