# ring-a-ding-ding

A self-hosted smart doorbell system built on a Raspberry Pi, a home server, and a mobile client.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          RASPBERRY PI  (Camera Node)                            │
│                                                                                 │
│   ┌──────────┐   motion    ┌─────────────────────────────────────────────────┐  │
│   │   PIR    │────event───▶│            Camera Pipeline                      │  │
│   │ Sensor   │             │                                                  │  │
│   └──────────┘             │  idle: low-res preview                          │  │
│        │                   │  triggered: switch to high-res + H.264 encode   │  │
│        │ polls             └───────────────────┬─────────────────────────────┘  │
│        │                                       │ RTSP / raw TCP stream          │
└────────┼───────────────────────────────────────┼────────────────────────────────┘
         │                                       │
         │                              video stream + event metadata
         │                                       │
         │                                       ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         SELF-HOSTED SERVER  (Backend)                           │
│                                                                                 │
│   ┌─────────────────────┐   ┌─────────────────────┐   ┌──────────────────────┐ │
│   │   Stream Ingestor   │──▶│  Motion Validator    │──▶│   Video Store        │ │
│   │                     │   │                      │   │                      │ │
│   │  accepts TCP conn   │   │  discard false       │   │  persists confirmed  │ │
│   │  from Pi camera     │   │  positives           │   │  event clips         │ │
│   └─────────────────────┘   └──────────┬───────────┘   └──────────────────────┘ │
│                                        │ confirmed event                        │
│                                        ▼                                        │
│   ┌─────────────────────────────────────────────────────────────────────────┐   │
│   │                          API  /  Event Bus                              │   │
│   │                                                                         │   │
│   │   • event feed  — ordered list of confirmed motion events               │   │
│   │   • live feed   — on-demand proxy of the Pi's current camera stream     │   │
│   │   • archive     — serves stored video clips for past events             │   │
│   │   • push        — forwards confirmed-event notifications to client      │   │
│   └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└──────────────────────────────────────┬──────────────────────────────────────────┘
                                       │
                          push notification + stream / archive API
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          CLIENT APPLICATION  (Mobile / Web)                     │
│                                                                                 │
│   ┌──────────────────────┐   ┌────────────────────────┐   ┌──────────────────┐ │
│   │   Push Notification  │   │     Live Feed Viewer   │   │  Event Archive   │ │
│   │                      │   │                        │   │                  │ │
│   │  receives alert on   │──▶│  taps in to real-time  │   │  browses & plays │ │
│   │  confirmed motion    │   │  camera stream via     │   │  past event clips│ │
│   │  event               │   │  server proxy          │   │  from server     │ │
│   └──────────────────────┘   └────────────────────────┘   └──────────────────┘ │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Data flows

| Flow                  | Path                           | Notes                                                                          |
| --------------------- | ------------------------------ | ------------------------------------------------------------------------------ |
| Motion trigger        | PIR → Camera Pipeline → Server | Pi polls PIR; on trigger, ramps camera to high resolution and begins encoding  |
| Video stream          | Pi → Server Stream Ingestor    | Continuous TCP stream; server buffers and stores on confirmed events           |
| False-positive filter | Server Motion Validator        | Server-side secondary check before persisting a clip or pushing a notification |
| Live feed             | Client → Server → Pi           | Client requests live view; server proxies the Pi's current stream on demand    |
| Event archive         | Client → Server Video Store    | Client retrieves stored clips from server for past events                      |
| Push notification     | Server → Client                | Server pushes an alert to the client app on each confirmed motion event        |

---

## Components

### Pi (`/pi`) — scaffolded, unimplemented

- Language: Rust
- Crates: `rust_gpiozero` (GPIO / PIR), `ctrlc` (graceful shutdown), `rusqlite` (bundled)
- Responsibilities: PIR polling, dynamic camera resolution switching, H.264 encoding, TCP stream to server
- Status: `Cargo.toml` declares the intended dependencies but `main.rs` is still the default "Hello, world!" stub — none of the above is implemented yet

### Backend (`/backend`) — in progress

- Language: Rust
- Crates: `tokio` (async runtime), `axum` (HTTP API), `rusqlite` (bundled, metadata storage), `serde` (JSON serialization)
- Responsibilities: TCP listener for the Pi stream, stream ingestion, motion validation, video storage, event/live-feed/archive HTTP API, push notification dispatch
- Status:
  - `Config::load` reads `PI_PORT`, `API_PORT`, `DB_NAME`, `VIDEO_PATH` from env vars with defaults; no config file support yet
  - `App::run_pi_ingestor` binds the Pi TCP listener and accepts connections, but the accepted socket is dropped immediately — no stream reading, decoding, or motion validation yet
  - `App::run_http_server` serves the axum router with a single `GET /events` route; no live-feed proxy or archive endpoints yet
  - `SqliteStore` (implements `MetadataStore`) is wired up but `insert_event`/`list_events` are stubs (no-op query / always returns an empty list)
  - `DiskVideoStore` (implements `VideoStore`) has working `save_clip`/`load_clip`/`delete_clip` against the filesystem, but nothing calls it yet — it isn't wired to the ingestor
  - `migration_runner.rs` exists but is empty — no schema or migrations defined
  - `Event` type (`id`, `timestamp`, `event_type`, `description`) derives `Serialize` for JSON API responses

### Client (`/frontend`) — scaffolded, unimplemented

- Stack: Tauri + React + Vite
- Responsibilities: push notification handling, live feed playback, event archive browsing
- Status: still the default Tauri/React/Vite starter template (the "Greet" demo) — no doorbell-specific UI, API integration, or push handling has been built yet
