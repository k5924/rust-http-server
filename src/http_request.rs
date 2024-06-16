use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(
        method: String,
        path: String,
        http_version: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Self {
            method,
            path,
            http_version,
            headers,
            body,
        }
    }
}

pub fn parse_http_request(request_lines: &[String]) -> HttpRequest {
    let method_path_version: Vec<&str> = request_lines[0].split_whitespace().collect();
    let method = method_path_version[0].to_string();
    let path = method_path_version[1].to_string();
    let http_version = method_path_version[2].to_string();

    let mut headers = HashMap::new();
    for line in &request_lines[1..] {
        if let Some(index) = line.find(':') {
            let header_name = line[..index].trim().to_string();
            let header_value = line[(index + 1)..].trim().to_string();
            headers.insert(header_name, header_value);
        }
    }

    let body = None; // No body parsing implemented in this example

    HttpRequest::new(method, path, http_version, headers, body)
}
