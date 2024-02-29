use std::{io::Result, net::UdpSocket, str::from_utf8};

fn main() -> Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        let server_addr = "127.0.0.1:8080";
        let msg = "Hello, World";
        socket.send_to(&msg.as_bytes(), server_addr);
        let mut buf = [0u8; 12];
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                let data = from_utf8(&buf).unwrap();
                println!("Uppercase word: {}", data);
            }
            Err(e) => {
                println!("Error processing data: {}", e);
            }
        }

        println!("Message delivered");
    }
    Ok(())
}
