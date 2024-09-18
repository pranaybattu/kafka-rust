#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => handle_client(&mut _stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut length = [0; 4];
    let mut request_api_key = [0; 2];
    let mut request_api_version = [0; 2];
    let mut correlation_id = [0; 4];
    let mut rest = vec![];
    stream.read_exact(&mut length).unwrap();
    stream.read_exact(&mut request_api_key).unwrap();
    stream.read_exact(&mut request_api_version).unwrap();
    stream.read_exact(&mut correlation_id).unwrap();
    let response_header = [0; 4];
    let response = [response_header, correlation_id].concat();

    stream.write(&response).unwrap();
    stream.read_to_end(&mut rest).unwrap();
}