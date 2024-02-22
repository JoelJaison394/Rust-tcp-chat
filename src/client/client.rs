use std::net::TcpStream;
use std::io::{Write, stdin, stdout};

fn main() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    loop {

        print!("Enter a message to send: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "exit" {
            println!("Exiting...");
            break;
        }
        let message = format!("[Client]: {}", input.trim());
        stream.write_all(message.as_bytes()).expect("Failed to write to server");


    }
}
