use crate::db::{MetadataStore, SqliteStore};
use crate::event::Event;
use axum::{Json, Router, extract::State, routing::get};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ApiState {
    pub metadata: Arc<Mutex<SqliteStore>>,
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/events", get(list_events).post(insert_event))
        .with_state(state)
}

async fn insert_event(State(state): State<ApiState>, Json(event): Json<Event>) -> Json<Event> {
    let metadata = state.metadata.lock().unwrap();
    metadata.insert_event(&event).unwrap();
    Json(event)
}

async fn list_events(State(state): State<ApiState>) -> Json<Vec<Event>> {
    let metadata = state.metadata.lock().unwrap();
    Json(metadata.list_events().unwrap_or_default())
}
