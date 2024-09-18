use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const SUPPORTED_VERSIONS: [i16; 5] = [0, 1, 2, 3, 4];
const UNSUPPORTED_VERSION_ERROR_CODE: i16 = 35;

fn main() {
    println!("Server is running on port 9092...");

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

    let api_version = i16::from_be_bytes(request_api_version);
    println!("Received API version: {}", api_version);

    let mut response = vec![];

    if SUPPORTED_VERSIONS.contains(&api_version) {
        let response_header = [0; 4];
        response.extend_from_slice(&response_header);
        response.extend_from_slice(&correlation_id);
    } else {
        let response_header = [0; 4];
        let error_code = UNSUPPORTED_VERSION_ERROR_CODE.to_be_bytes();
        response.extend_from_slice(&response_header);
        response.extend_from_slice(&correlation_id);
        response.extend_from_slice(&error_code);
    }

    stream.write_all(&response).unwrap();
    stream.read_to_end(&mut rest).unwrap();
}
