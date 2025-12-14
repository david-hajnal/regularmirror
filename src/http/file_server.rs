use super::html::Html;
use std::fs;
use std::path::Path;

pub fn serve_file(path: &str) -> Result<Html, std::io::Error> {
    // Convert URL path to file path
    let file_path = if path == "/" {
        "public/index.html"
    } else {
        // Remove leading slash and prepend 'public/'
        &format!("public{}", path)
    };

    // Read file
    match fs::read(file_path) {
        Ok(contents) => {
            let content_type: String = get_content_type(file_path).parse().unwrap();
            let content: String = String::from_utf8_lossy(&contents).parse().unwrap();
            Ok(Html {
                content_type,
                content,
            })
        }
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not found",
        )),
    }
}

fn get_content_type(path: &str) -> &str {
    let path_obj = Path::new(path);
    match path_obj.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "text/plain",
    }
}
