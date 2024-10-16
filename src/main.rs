use std::io::{self, Write, Read};
use std::net::TcpStream;

const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server.");

    // Define the key to retrieve
    let key = "test";
    let value = "test_value";
    
    // Create the GET request message
    let get_request = create_get_request(key);
    let put_request = create_put_request(key, value);

    // Send the request to the server
    stream.write_all(&put_request)?;

    // Read the response from the server
    let mut response_buffer = vec![0; 256]; // Adjust size as needed
    let bytes_read = stream.read(&mut response_buffer)?;

    // Print the response
    if bytes_read > 0 {
        parse_response(&response_buffer[..bytes_read]);
    } else {
        println!("No response received.");
    }

    Ok(())
}

fn create_get_request(key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let key_length = key_bytes.len() as u8;

    // Construct the GET request message
    let mut request = vec![VERSION];
    request.push(OPERATION_GET);
    request.push(key_length);
    request.extend_from_slice(key_bytes);

    request
}

fn create_put_request(key: &str, value: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let key_length = key_bytes.len() as u8;
    let value_bytes: &[u8] = value.as_bytes();
    let value_length: u8 = value_bytes.len() as u8;

    // Construct the PUT request message
    let mut request = vec![VERSION];
    request.push(OPERATION_PUT);
    request.push(key_length);
    request.push(value_length);
    request.extend_from_slice(key_bytes);
    request.extend_from_slice(value_bytes);
    request
}

fn parse_response(response: &[u8]) {
    if response.len() < 4 {
        println!("Invalid response length.");
        return;
    }

    let version = response[0];
    let operation_type = response[1];
    let status = response[2];
    let value_length = response[3];

    // Validate the response
    if version != VERSION || operation_type != 0x81 {
        println!("Invalid response version or operation type.");
        return;
    }

    // Check the status
    if status == 0x00 {
        // Success, read the value
        let value = &response[4..(4 + value_length as usize)];
        let value_str = String::from_utf8_lossy(value);
        println!("Received value: {}", value_str);
    } else {
        // Not found
        println!("Key not found.");
    }
}
