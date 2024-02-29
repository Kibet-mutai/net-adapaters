use std::{
    io::{self, Read, Write},
    net::TcpStream,
    str::from_utf8,
};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080");
    match stream {
        Ok(mut s) => {
            println!("connected on port 8080");
            loop {
                println!("Please input words to be converted:");
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                if user_input.trim() == "exit" {
                    break;
                }
                s.write(user_input.as_bytes()).unwrap();
                println!("Send the lower case, waiting reply...");

                let mut data = [0; 128];
                match s.read(&mut data) {
                    Ok(_) => {
                        let server_response = from_utf8(&data).unwrap();
                        println!("Server response: {}", server_response);
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
