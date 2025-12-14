mod config;
mod calendar;
mod http;

use calendar::{load_events, start_background_sync};

fn main() {
    println!("=== Starting Regular HTTP Server ===\n");

    // Load configuration
    let config = config::Config::load(".env").expect("Failed to load .env");
    println!("✓ Loaded config: {}", config);

    // Create data directory
    std::fs::create_dir_all("data").expect("Failed to create data directory");

    // LOAD EXISTING EVENTS FIRST (so server can start serving immediately)
    println!("\n--- Loading cached events ---");
    let initial_events = load_events();
    println!("✓ Loaded {} cached events from data/events.json\n", initial_events.len());

    // START BACKGROUND SYNC (runs in separate thread)
    println!("--- Starting background sync ---");
    start_background_sync(config.clone());

    // START HTTP SERVER
    println!("\n--- Starting HTTP server ---");
    http::serve(&config.server_address);
}
