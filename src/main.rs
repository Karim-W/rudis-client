use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            loop {
            
            thread::sleep(time::Duration::from_millis(1000));
            // std in to send data
            println!(">");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            // Write the data
            let msg = input.trim().as_bytes();
            // let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent command awaiting reply...");

            let mut data = [0 as u8; 1024]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                    println!("Reply: {}", from_utf8(&data).unwrap());
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
