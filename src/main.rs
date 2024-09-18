use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const VALID_API_VERSIONS: std::ops::RangeInclusive<i16> = 0..=4;
const UNSUPPORTED_VERSION: i16 = 35;

fn main() -> std::io::Result<()> {
    println!("Server starting...");

    let listener = TcpListener::bind("127.0.0.1:9092")?;
    println!("Listening on 127.0.0.1:9092");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(e) = handle_client(&mut stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 12];
    stream.read_exact(&mut buffer)?;

    let (_length, request_api_version, correlation_id) = parse_request(&buffer);

    let error_code = if VALID_API_VERSIONS.contains(&request_api_version) {
        0 // No error
    } else {
        UNSUPPORTED_VERSION
    };

    let response = create_response(correlation_id, error_code);
    stream.write_all(&response)?;

    Ok(())
}

fn parse_request(buffer: &[u8; 12]) -> (i32, i16, i32) {
    let length = i32::from_be_bytes(buffer[0..4].try_into().unwrap());
    let request_api_version = i16::from_be_bytes(buffer[6..8].try_into().unwrap());
    let correlation_id = i32::from_be_bytes(buffer[8..12].try_into().unwrap());
    (length, request_api_version, correlation_id)
}

fn create_response(correlation_id: i32, error_code: i16) -> Vec<u8> {
    let response_length = (4 + 4 + 2).to_be_bytes();
    let correlation_id_bytes = correlation_id.to_be_bytes();
    let error_code_bytes = error_code.to_be_bytes();

    [
        response_length.as_slice(),
        correlation_id_bytes.as_slice(),
        error_code_bytes.as_slice(),
    ]
    .concat()
}