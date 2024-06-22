use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use log::{error, info};

use crate::http_request::HttpRequest;

pub trait Handler {
    fn execute(&self, _request: &HttpRequest) -> Vec<u8>;
}

pub struct RootHandler;
pub struct NotFoundHandler;
pub struct EchoHandler;
pub struct UserAgentHandler;
pub struct FileReadHandler;

impl Handler for RootHandler {
    fn execute(&self, _request: &HttpRequest) -> Vec<u8> {
        b"HTTP/1.1 200 OK\r\n\r\n".to_vec()
    }
}

impl Handler for NotFoundHandler {
    fn execute(&self, _request: &HttpRequest) -> Vec<u8> {
        b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
    }
}

impl Handler for EchoHandler {
    fn execute(&self, request: &HttpRequest) -> Vec<u8> {
        let mut response = "HTTP/1.1 200 OK\r\n".to_string().into_bytes();
        let parts: Vec<&str> = request.path.split('/').collect();
        if parts.len() > 2 {
            info!("found path to echo on response");
            let content: &str = parts.get(2).unwrap();
            response.extend_from_slice(
                format!(
                    "Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                )
                .as_bytes(),
            );
        }
        response
    }
}

impl Handler for UserAgentHandler {
    fn execute(&self, request: &HttpRequest) -> Vec<u8> {
        let mut response = "HTTP/1.1 200 OK\r\n".to_string().into_bytes();
        let empty_string = "".to_string();
        let user_agent = request
            .headers
            .get(&"User-Agent".to_string())
            .unwrap_or(&empty_string);
        if !user_agent.is_empty() {
            info!("Found user agent to write on response");
            response.extend_from_slice(
                format!(
                    "Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent
                )
                .as_bytes(),
            )
        }
        response
    }
}

impl Handler for FileReadHandler {
    fn execute(&self, request: &HttpRequest) -> Vec<u8> {
        let mut response = "HTTP/1.1 ".to_string().into_bytes();
        let parts: Vec<&str> = request.path.split('/').collect();
        if parts.len() > 2 {
            info!("File name is provided, checking if file exists");
            let file_name = parts.get(2).unwrap_or(&"");
            if !file_name.is_empty() {
                info!("Found file name");
                let mut directory = String::new();
                directory.push_str("/tmp/data/codecrafters.io/http-server-tester/");
                directory.push_str(file_name);
                let path = Path::new(&directory);
                if fs::metadata(path).is_ok() {
                    info!("File exists at path, reading content");
                    let mut file = File::open(path).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    response.extend_from_slice(
                        format!(
                            "200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                            contents.len(),
                            contents
                        )
                        .as_bytes(),
                    )
                } else {
                    error!("Path on disk doesnt exist");
                    response.extend_from_slice("404 Not Found\r\n\r\n".as_bytes());
                }
            } else {
                error!("No sub path provided");
                response.extend_from_slice("404 Not Found\r\n\r\n".as_bytes());
            }
        } else {
            error!("No sub path provided");
            response.extend_from_slice("404 Not Found\r\n\r\n".as_bytes());
        }
        response
    }
}

pub fn handle_request(request: &HttpRequest) -> Box<dyn Handler> {
    if request.path.starts_with("/echo") {
        info!("Returning echo handler");
        Box::new(EchoHandler)
    } else if request.path.starts_with("/user-agent") {
        info!("Returning user agent handler");
        Box::new(UserAgentHandler)
    } else if request.path.starts_with("/files") {
        info!("Returning file read handler");
        Box::new(FileReadHandler)
    } else if request.path == "/" {
        info!("Returning root handler");
        Box::new(RootHandler)
    } else {
        info!("Returning not found handler");
        Box::new(NotFoundHandler)
    }
}
