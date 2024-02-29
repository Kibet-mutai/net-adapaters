use std::{io::Result, net::UdpSocket};

fn main() -> Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:8080")?;
        println!("Listening from port 8080");
        let mut buffer = [0 as u8; 128];
        loop {
            let (_, src) = socket.recv_from(&mut buffer).expect("Failed to receive data");
            println!("Received data from: {}", src);
            if let Ok(msg) = std::str::from_utf8(&buffer) {
                println!("message recieved: {}", msg);
                 socket.send_to(&msg.to_uppercase().as_bytes(), src);
                //println!("Sending converted words {:?}", msg_snt);
            } else {
                println!("Invalid UTF-8 data");
            }
            if buffer.starts_with(b"exit") {
                println!("Exiting..");
                break;
            }
            buffer = [0; 128];
        }
    }
    Ok(())
}
