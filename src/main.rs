#![allow(unused_imports)]
use std::{
    io::Write,
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
    let _ = stream.write(&[0, 0, 0, 0, 0, 0, 0, 7]);
}