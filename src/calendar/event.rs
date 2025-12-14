use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub location: String,
    pub description: String,
    #[serde(default)]
    pub calendar_id: String,
}

const STORAGE_PATH: &str = "data/events.json";
const TIMESTAMP_PATH: &str = "data/last_update.txt";

/// Load events from JSON file
/// Returns empty vec if file doesn't exist or on error
pub fn load_events() -> Vec<Event> {
    if !Path::new(STORAGE_PATH).exists() {
        return Vec::new();
    }

    fs::read_to_string(STORAGE_PATH)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

/// Save events to JSON file
pub fn save_events(events: &[Event]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("data")?;
    let json = serde_json::to_string_pretty(events)?;
    fs::write(STORAGE_PATH, json)?;

    // Update timestamp
    let timestamp = Utc::now().to_rfc3339();
    fs::write(TIMESTAMP_PATH, timestamp)?;

    Ok(())
}

/// Get the last update timestamp
pub fn get_last_update() -> String {
    fs::read_to_string(TIMESTAMP_PATH)
        .unwrap_or_else(|_| Utc::now().to_rfc3339())
}
