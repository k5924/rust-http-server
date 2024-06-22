use crate::http_request::HttpRequest;

pub trait Handler {
    fn execute(&self, _request: &HttpRequest) -> Vec<u8>;
}

pub struct RootHandler;
pub struct NotFoundHandler;
pub struct EchoHandler;

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
        let mut response = "HTTP/1.1 200 OK \r\n".to_string().into_bytes();
        let parts: Vec<&str> = request.path.split('/').collect();
        if parts.len() > 2 {
            let content: &str = parts.get(2).unwrap();
            response.extend_from_slice(
                format!(
                    "Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                )
                .as_bytes(),
            );
            response.extend_from_slice(content.as_bytes());
        }
        response
    }
}

pub fn handle_request(request: &HttpRequest) -> Box<dyn Handler> {
    if request.path.starts_with("/echo") {
        Box::new(EchoHandler)
    } else if request.path == "/" {
        Box::new(RootHandler)
    } else {
        Box::new(NotFoundHandler)
    }
}
