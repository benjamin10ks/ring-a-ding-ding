use crate::db::{MetadataStore, SqliteStore};
use crate::event::Event;
use axum::{extract::State, routing::get, Json, Router};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ApiState {
    pub metadata: Arc<Mutex<SqliteStore>>,
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/events", get(list_events))
        .with_state(state)
}

async fn list_events(State(state): State<ApiState>) -> Json<Vec<Event>> {
    let metadata = state.metadata.lock().unwrap();
    Json(metadata.list_events().unwrap_or_default())
}
