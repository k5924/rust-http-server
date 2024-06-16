use crate::http_request::HttpRequest;

pub trait Handler {
    fn execute(&self) -> &[u8];
}

pub struct RootHandler;
pub struct NotFoundHandler;

impl Handler for RootHandler {
    fn execute(&self) -> &[u8] {
        b"HTTP/1.1 200 OK\r\n\r\n"
    }
}

impl Handler for NotFoundHandler {
    fn execute(&self) -> &[u8] {
        b"HTTP/1.1 404 Not Found\r\n\r\n"
    }
}

pub fn handle_request(request: &HttpRequest) -> Box<dyn Handler> {
    match request.path.as_str() {
        "/" => Box::new(RootHandler),
        _ => Box::new(NotFoundHandler),
    }
}
