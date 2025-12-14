use super::event::Event;
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Tz;

pub fn parse_ics(content: &str, timezone: &str) -> Vec<Event> {
    let mut events = Vec::new();
    let mut current_event: Option<EventBuilder> = None;

    // Parse timezone
    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);

    for line in content.lines() {
        let line = line.trim();

        if line == "BEGIN:VEVENT" {
            current_event = Some(EventBuilder::default());
        } else if line == "END:VEVENT" {
            if let Some(builder) = current_event.take() {
                events.push(builder.build());
            }
        } else if let Some(ref mut builder) = current_event {
            if let Some((key, value)) = line.split_once(':') {
                match key {
                    "SUMMARY" => builder.title = value.to_string(),
                    "DTSTART" => builder.start_date = parse_ics_date(value, tz),
                    "DTEND" => builder.end_date = parse_ics_date(value, tz),
                    "LOCATION" => builder.location = value.to_string(),
                    "DESCRIPTION" => builder.description = unescape_ics_text(value),
                    _ => {}
                }
            }
        }
    }

    events
}

#[derive(Default)]
struct EventBuilder {
    title: String,
    start_date: String,
    end_date: String,
    location: String,
    description: String,
}

impl EventBuilder {
    fn build(self) -> Event {
        Event {
            title: self.title,
            start_date: self.start_date,
            end_date: self.end_date,
            location: self.location,
            description: self.description,
        }
    }
}

fn parse_ics_date(date_str: &str, target_tz: Tz) -> String {
    // ICS date format: 20250115T100000Z (UTC)
    // Parse it and convert to target timezone

    if date_str.len() >= 15 && date_str.contains('T') {
        // Remove 'Z' if present
        let clean_date = date_str.trim_end_matches('Z');

        // Parse the datetime string
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(clean_date, "%Y%m%dT%H%M%S") {
            // Assume UTC as source timezone
            let utc_dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_dt, Utc);

            // Convert to target timezone
            let target_dt = utc_dt.with_timezone(&target_tz);

            // Format as: YYYY-MM-DD HH:MM:SS
            return target_dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }

    // Fallback to old format for date-only
    if date_str.len() >= 8 {
        format!(
            "{}-{}-{}",
            &date_str[0..4],
            &date_str[4..6],
            &date_str[6..8]
        )
    } else {
        date_str.to_string()
    }
}

fn unescape_ics_text(text: &str) -> String {
    text.replace("\\n", "\n")
        .replace("\\,", ",")
        .replace("\\;", ";")
}
