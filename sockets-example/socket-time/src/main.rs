use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

//impl fmt::Display for SystemTime {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self.)
//    }
//}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("New connection: {}", s.peer_addr().unwrap());
                thread::spawn(move || handle_client(s));
            }
            Err(e) => {
                println!("Error connection: {}", e);
            }
        }
    }
    drop(listener);
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(s) => {
            let client_request = String::from_utf8_lossy(&buffer[0..s]);
            println!("Converting to upper case....");
            let upper_case = client_request.to_uppercase();
            stream.write(upper_case.as_bytes()).unwrap();
            return;
        }
        Err(e) => {
            println!("Error: {}", e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
