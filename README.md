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

| Flow | Path | Notes |
|------|------|-------|
| Motion trigger | PIR → Camera Pipeline → Server | Pi polls PIR; on trigger, ramps camera to high resolution and begins encoding |
| Video stream | Pi → Server Stream Ingestor | Continuous TCP stream; server buffers and stores on confirmed events |
| False-positive filter | Server Motion Validator | Server-side secondary check before persisting a clip or pushing a notification |
| Live feed | Client → Server → Pi | Client requests live view; server proxies the Pi's current stream on demand |
| Event archive | Client → Server Video Store | Client retrieves stored clips from server for past events |
| Push notification | Server → Client | Server pushes an alert to the client app on each confirmed motion event |

---

## Components

### Pi (`/pi` — planned)
- Language: Rust
- Crates: `rust_gpiozero` (GPIO / PIR), `ctrlc` (graceful shutdown)
- Responsibilities: PIR polling, dynamic camera resolution switching, H.264 encoding, TCP stream to server

### Backend (`/backend`)
- Language: Rust
- Crates: `tokio` (async runtime)
- Responsibilities: TCP listener (port 6379), stream ingestion, motion validation, video storage, event/live-feed/archive API, push notification dispatch

### Client (planned)
- Responsibilities: push notification handling, live feed playback, event archive browsing
