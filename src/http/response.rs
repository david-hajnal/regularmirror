use super::html::Html;

pub fn create_response(status_code: u16, status_text: &str, content_type: &str, body: &[u8]) -> String {
    let mut response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_code,
        status_text,
        content_type,
        body.len()
    );

    // Append body as string (works for text content)
    response.push_str(&String::from_utf8_lossy(body));

    response
}

pub fn create_html_response(status_code: u16, status_text: &str, html: Html) -> String {
    let mut response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_code,
        status_text,
        html.content_type,
        html.content.len()
    );

    // Append body as string (works for text content)
    response.push_str(&*html.content);

    response
}
