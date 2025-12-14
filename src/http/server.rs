use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use super::file_server::serve_file;
use super::response::{create_html_response, create_response};
use crate::calendar::{load_events, get_last_update};
use chrono::{Local, NaiveDateTime, TimeZone};

pub fn serve(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server running on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            let request_line = request.lines().next().unwrap_or("");

            println!("Request: {}", request_line);

            let response = process_request(request_line);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}

fn process_request(request_line: &str) -> String {
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 {
        return create_response(400, "Bad Request", "text/plain", b"Bad Request");
    }

    let method = parts[0];
    let path = parts[1];

    if method != "GET" {
        return create_response(405, "Method Not Allowed", "text/plain", b"Method Not Allowed");
    }

    // Serve dynamic index with events
    if path == "/" {
        return generate_index_html();
    }

    // API endpoint for last update timestamp
    if path == "/api/last-update" {
        let timestamp = get_last_update();
        let json = format!(r#"{{"last_update":"{}"}}"#, timestamp);
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            json.len(),
            json
        );
    }

    // Serve static files
    let result = serve_file(path);

    match result {
        Ok(html) => {
            create_html_response(200, "Ok", html)
        }
        Err(e) => {
            eprintln!("{}", e);
            create_response(404, "Not found", "text/plain", b"Not Found")
        }
    }
}

fn generate_index_html() -> String {
    let events = load_events();
    let now = Local::now();

    // Filter events: only show current or future events (end_date >= now)
    let future_events: Vec<_> = events
        .iter()
        .filter(|event| {
            // Parse end_date (format: YYYY-MM-DD HH:MM:SS)
            if let Ok(end_dt) = NaiveDateTime::parse_from_str(&event.end_date, "%Y-%m-%d %H:%M:%S") {
                // Convert to Local timezone for comparison
                let end_local = Local::now().timezone().from_local_datetime(&end_dt).single();
                if let Some(end_time) = end_local {
                    return end_time >= now;
                }
            }
            false // If parsing fails, exclude the event
        })
        .collect();

    let events_to_show: Vec<_> = future_events.iter().take(10).collect();

    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MY AGENDA</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background-color: #000;
            color: #fff;
            padding: 40px 20px;
            max-width: 700px;
            margin: 0 auto;
        }
        .header {
            text-align: right;
            font-size: 14px;
            letter-spacing: 2px;
            color: #999;
            margin-bottom: 40px;
        }
        .day-section {
            margin-bottom: 30px;
            border-bottom: 1px solid #333;
            padding-bottom: 20px;
        }
        .day-header {
            font-size: 18px;
            font-weight: bold;
            text-transform: uppercase;
            margin-bottom: 15px;
            letter-spacing: 1px;
        }
        .event-item {
            display: flex;
            align-items: flex-start;
            margin-bottom: 12px;
            padding-left: 0;
        }
        .event-dot {
            width: 12px;
            height: 12px;
            border-radius: 50%;
            margin-right: 10px;
            margin-top: 4px;
            flex-shrink: 0;
        }
        .event-dot.active {
            background-color: #ff6b35;
        }
        .event-dot.past {
            background-color: #666;
        }
        .event-dot.video {
            background-color: #4a9eff;
        }
        .event-content {
            flex: 1;
        }
        .event-time {
            color: #ccc;
            font-size: 13px;
            margin-right: 8px;
        }
        .event-title {
            color: #fff;
            font-size: 14px;
            display: inline;
        }
        .event-tag {
            display: inline-block;
            background-color: #4a4a4a;
            color: #fff;
            padding: 2px 10px;
            border-radius: 4px;
            font-size: 12px;
            margin-right: 6px;
            margin-bottom: 4px;
        }
        .event-tag.green {
            background-color: #2d5f2e;
        }
        .event-tag.blue {
            background-color: #1e3a5f;
        }
        .event-tag.orange {
            background-color: #5f3a1e;
        }
        .event-status {
            color: #999;
            font-size: 12px;
            margin-left: 8px;
        }
        .event-location {
            color: #999;
            font-size: 12px;
            text-align: right;
            margin-top: -20px;
            margin-bottom: 10px;
        }
        .event-description {
            color: #999;
            font-size: 13px;
            margin-left: 22px;
            margin-top: 4px;
        }
        .no-events {
            text-align: center;
            padding: 60px 20px;
            color: #666;
            font-size: 14px;
        }
    </style>
</head>
<body>
    <div class="header">MY AGENDA</div>
"#);

    if events_to_show.is_empty() {
        html.push_str(r#"<div class="no-events">No events found. Check back later!</div>"#);
    } else {
        let mut last_date = String::new();

        for event in events_to_show {
            // Extract date from start_date (assuming format: YYYY-MM-DD HH:MM:SS)
            let date_part = event.start_date.split_whitespace().next().unwrap_or("");

            // If new day, create day section
            if date_part != last_date {
                if !last_date.is_empty() {
                    html.push_str("    </div>\n");
                }

                html.push_str(&format!(r#"    <div class="day-section">
        <div class="day-header">{}</div>
"#, format_date(date_part)));

                last_date = date_part.to_string();
            }

            // Extract time from start and end
            let start_time = extract_time(&event.start_date);
            let end_time = extract_time(&event.end_date);

            // Determine event type for dot color
            let dot_class = if !event.description.is_empty() && event.description.contains("video") {
                "video"
            } else {
                "active"
            };

            html.push_str(&format!(r#"        <div class="event-item">
            <div class="event-dot {}"></div>
            <div class="event-content">
                <span class="event-time">{} - {}</span>
                <span class="event-title">{}</span>
"#,
                dot_class,
                escape_html(&start_time),
                escape_html(&end_time),
                escape_html(&event.title)
            ));

            // Add location if present
            if !event.location.is_empty() {
                html.push_str(&format!(r#"
                <div class="event-location">{}</div>
"#, escape_html(&event.location)));
            }

            // Add description if present
            if !event.description.is_empty() {
                html.push_str(&format!(r#"
                <div class="event-description">{}</div>
"#, escape_html(&event.description)));
            }

            html.push_str("            </div>\n        </div>\n");
        }

        if !last_date.is_empty() {
            html.push_str("    </div>\n");
        }
    }

    // Add JavaScript for auto-reload on updates
    html.push_str(r#"
    <script>
        // Get initial timestamp
        let lastKnownUpdate = null;

        // Fetch current timestamp on load
        fetch('/api/last-update')
            .then(r => r.json())
            .then(data => {
                lastKnownUpdate = data.last_update;
            });

        // Check for updates every minute
        setInterval(() => {
            fetch('/api/last-update')
                .then(r => r.json())
                .then(data => {
                    if (lastKnownUpdate && data.last_update > lastKnownUpdate) {
                        console.log('Events updated, reloading...');
                        location.reload();
                    }
                    lastKnownUpdate = data.last_update;
                })
                .catch(err => console.error('Failed to check for updates:', err));
        }, 60000); // Check every 60 seconds
    </script>
"#);

    html.push_str("</body></html>");

    // Build HTTP response
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
}

fn format_date(date_str: &str) -> String {
    // Input format: YYYY-MM-DD
    // Output format: DD. MONTH
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        let months = ["", "JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE",
                     "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER"];
        let month_idx: usize = parts[1].parse().unwrap_or(1);
        let day = parts[2].parse::<u32>().unwrap_or(1);
        format!("{}. {}", day, months.get(month_idx).unwrap_or(&"UNKNOWN"))
    } else {
        date_str.to_string()
    }
}

fn extract_time(datetime: &str) -> String {
    // Extract HH:MM from "YYYY-MM-DD HH:MM:SS"
    datetime.split_whitespace()
        .nth(1)
        .and_then(|time| time.split(':').take(2).collect::<Vec<_>>().join(":").into())
        .unwrap_or_else(|| "00:00".to_string())
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\n', "<br>")
}