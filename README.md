# Simple HTTP Server for Displaying Calendar Events

A lightweight HTTP server built with vanilla Rust (no web frameworks) that displays your Google Calendar events in a clean, dark-themed agenda view.

## Features

- Vanilla Rust HTTP server using only `std::net`
- Google Calendar integration via ICS feeds
- Automatic timezone conversion
- Dark, minimalist UI design
- Auto-refresh when events are updated
- Multi-calendar support with color coding
- Filters to show only current and future events
- Background sync every hour

## Prerequisites

- Rust (latest stable version)
- Google Calendar with private ICS URL

## Setup

1. Clone or download the project

2. Get your Google Calendar private ICS URL:
   - Go to Google Calendar Settings
   - Select your calendar
   - Find "Secret address in iCal format" under "Integrate calendar"
   - Copy the URL

3. Create `.env` file in the project root:

```env
SERVER_ADDRESS=127.0.0.1:8080
ICS_URLS=https://calendar.google.com/calendar/ical/YOUR_EMAIL/private-XXXXX/basic.ics
REFRESH_PERIOD_SECONDS=3600
MAX_EVENTS_DISPLAY=10
TIMEZONE=Europe/Berlin
```

4. Run the server:

```bash
cargo run
```

5. Open your browser:

```
http://127.0.0.1:8080
```

## Configuration

### Environment Variables

- `SERVER_ADDRESS` - Server bind address (default: 127.0.0.1:8080)
- `ICS_URLS` - Comma-separated list of ICS URLs for multiple calendars
- `REFRESH_PERIOD_SECONDS` - How often to fetch calendar updates (default: 3600 = 1 hour)
- `MAX_EVENTS_DISPLAY` - Maximum number of events to show (default: 10)
- `TIMEZONE` - IANA timezone for event display (e.g., America/New_York, Asia/Tokyo)

### Multiple Calendars

To display events from multiple calendars with different colors:

```env
ICS_URLS=https://calendar.google.com/.../personal.ics,https://calendar.google.com/.../work.ics,https://calendar.google.com/.../shared.ics
```

Each calendar gets a unique color:
- Calendar 1: Orange
- Calendar 2: Blue
- Calendar 3: Purple
- Calendar 4: Green
- Calendar 5: Yellow
- Calendar 6: Red

## Project Structure

```
src/
├── main.rs              # Entry point
├── config.rs            # Configuration loading
├── calendar/            # Calendar feature
│   ├── mod.rs
│   ├── event.rs        # Event storage (JSON)
│   ├── fetcher.rs      # HTTP client
│   ├── ics_parser.rs   # ICS parsing
│   └── scheduler.rs    # Background sync
└── http/                # HTTP server
    ├── mod.rs
    ├── html.rs         # HTML types
    ├── response.rs     # Response builders
    ├── file_server.rs  # Static files
    └── server.rs       # TCP server & routing
```

## How It Works

1. Server loads cached events from `data/events.json` on startup
2. Background thread fetches ICS feeds from Google Calendar
3. Events are parsed and converted to configured timezone
4. Events are filtered to show only current/future events
5. HTML page is generated with events grouped by day
6. Page auto-reloads when new events are fetched
7. Process repeats every hour (or configured period)

## API Endpoints

- `GET /` - Main agenda page with events
- `GET /api/last-update` - Returns last update timestamp (JSON)
- `GET /path/to/file` - Serves static files from `public/` directory

## Dependencies

- `serde` + `serde_json` - JSON serialization
- `reqwest` - HTTP client for fetching ICS feeds
- `chrono` + `chrono-tz` - Timezone conversion

## Development

Build:
```bash
cargo build
```

Run:
```bash
cargo run
```

Run in release mode:
```bash
cargo run --release
```

## License

MIT
