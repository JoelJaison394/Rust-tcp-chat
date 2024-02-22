use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    loop {
        // Read data from the client
        let mut buffer = [0; 1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                return;
            }
        };

        // If no bytes were read, the client has disconnected
        let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
        if bytes_read == 0 {
            println!("Client disconnected");
            return;
        }

        println!("Received from client: {}", received_data);
    }
}

fn main() {
    // Bind to the desired address and port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    println!("Server listening on 127.0.0.1:8080");

    // Accept incoming connections and handle them in separate threads
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
