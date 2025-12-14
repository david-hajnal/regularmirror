use std::thread;
use std::time::Duration;
use crate::config::Config;
use super::{event, fetcher, ics_parser};

pub fn start_background_sync(config: Config) {
    thread::spawn(move || {
        // Small initial delay to let server start
        println!("[Background] Starting in 5 seconds...");
        thread::sleep(Duration::from_secs(5));

        let client = match fetcher::HttpClient::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[Background] Failed to create HTTP client: {}", e);
                return;
            }
        };

        loop {
            println!("\n[Background] Fetching ICS feeds...");

            let mut all_events = Vec::new();

            for url in &config.ics_urls {
                match client.fetch_url(url) {
                    Ok(ics_content) => {
                        let events = ics_parser::parse_ics(&ics_content, &config.timezone);
                        println!("[Background] ✓ Parsed {} events from {}", events.len(), url);
                        all_events.extend(events);
                    }
                    Err(e) => {
                        eprintln!("[Background] ✗ Failed to fetch {}: {}", url, e);
                    }
                }
            }

            // Sort by start date
            all_events.sort_by(|a, b| a.start_date.cmp(&b.start_date));

            // Save to JSON
            match event::save_events(&all_events) {
                Ok(_) => println!("[Background] ✓ Saved {} events to data/events.json", all_events.len()),
                Err(e) => eprintln!("[Background] ✗ Failed to save events: {}", e),
            }

            println!("[Background] Sleeping for {} seconds...", config.refresh_period);
            thread::sleep(Duration::from_secs(config.refresh_period));
        }
    });
}
