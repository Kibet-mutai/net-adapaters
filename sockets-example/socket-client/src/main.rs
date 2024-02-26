use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
    time::SystemTime,
};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080");
    match stream {
        Ok(mut s) => {
            println!("connected on port 8080");

            let mut data = [0; 128];
            match s.read_exact(&mut data) {
                Ok(_) => {
                    let server_response = from_utf8(&data).unwrap();
                    println!("Server response: {}", server_response);
                                    }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
    //s.Write(time.as_bytes());
}
