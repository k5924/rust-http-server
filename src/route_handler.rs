use crate::http_request::HttpRequest;

pub trait Handler {
    fn execute(&self, _request: &HttpRequest) -> Vec<u8>;
}

pub struct RootHandler;
pub struct NotFoundHandler;
pub struct EchoHandler;
pub struct UserAgentHandler;

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
            println!("found path to echo on response");
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
            println!("Found user agent to write on response");
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

pub fn handle_request(request: &HttpRequest) -> Box<dyn Handler> {
    if request.path.starts_with("/echo") {
        println!("Returning echo handler");
        Box::new(EchoHandler)
    } else if request.path.starts_with("/user-agent") {
        println!("Returning user agent handler");
        Box::new(UserAgentHandler)
    } else if request.path == "/" {
        println!("Returning root handler");
        Box::new(RootHandler)
    } else {
        println!("Returning not found handler");
        Box::new(NotFoundHandler)
    }
}
