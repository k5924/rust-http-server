// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use log::info;

use crate::thread_pool::ThreadPool;

mod http_request;
mod route_handler;
mod thread_pool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    info!("Read request");

    let parsed_request = http_request::parse_http_request(&http_request);

    info!("Parsed Request: {parsed_request:#?}");

    let handler = route_handler::handle_request(&parsed_request);

    info!("Found handler to handle request");

    let response = handler.execute(&parsed_request);

    info!("Executed logic on handler for request");

    stream.write_all(&response).unwrap();

    info!("Written response");

    stream.flush().unwrap();

    info!("Flushed response");
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("unable to load config");

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    info!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
