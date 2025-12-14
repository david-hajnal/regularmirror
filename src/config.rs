use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
    pub ics_urls: Vec<String>,
    pub refresh_period: u64,
    pub max_events: usize,
    pub timezone: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n * Server address: {} \n * ICS Urls: {} \n * Refresh period: {} \n * Max events {} \n * Timezone: {}",
            self.server_address,
            self.ics_urls.join(","),
            self.refresh_period,
            self.max_events,
            self.timezone
        )
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let vars: HashMap<String, String> = reader
            .lines() // 1. Creates a stream (Iterator) of Result<String>
            .filter_map(|line_result| line_result.ok()) // 2. Unwrap result, skipping read errors
            .map(|line| line.trim().to_string()) // 3. Transform: Trim whitespace
            .filter(|line| !line.is_empty()) // 4. Filter: Skip empty lines
            .filter_map(|line| {
                // 5. Filter & Map: Parse key=value
                line.split_once('=')
                    .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
            })
            .collect(); // 6. Consumer: Collect into HashMap

        Self::parse(vars)
    }

    fn parse(vars: HashMap<String, String>) -> Result<Config, String> {
        let server_address = vars
            .get("SERVER_ADDRESS")
            .ok_or("Missing SERVER_ADDRESS")?
            .clone();

        let ics_urls_str = vars.get("ICS_URLS").ok_or("Missing ICS_URLS")?;
        let ics_urls: Vec<String> = ics_urls_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let refresh_period = vars
            .get("REFRESH_PERIOD_SECONDS")
            .ok_or("Missing REFRESH_PERIOD_SECONDS")?
            .parse()
            .map_err(|_| "Invalid REFRESH_PERIOD_SECONDS")?;

        let max_events = vars
            .get("MAX_EVENTS_DISPLAY")
            .ok_or("Missing MAX_EVENTS_DISPLAY")?
            .parse()
            .map_err(|_| "Invalid MAX_EVENTS_DISPLAY")?;

        let timezone = vars
            .get("TIMEZONE")
            .ok_or("Missing TIMEZONE")?
            .clone();

        Ok(Config {
            server_address,
            ics_urls,
            refresh_period,
            max_events,
            timezone,
        })
    }
}
