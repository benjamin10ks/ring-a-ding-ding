mod api;
mod app;
mod config;
mod db;
mod event;
mod video_store;

use app::App;
use config::Config;

#[tokio::main]
async fn main() {
    let cfg = Config::load();
    let app = App::new(cfg);
    app.run().await;
}
