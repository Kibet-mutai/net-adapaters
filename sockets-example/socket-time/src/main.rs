use std::{
    fmt,
    io::{Read, Write},
    mem::size_of,
    net::{self, Shutdown, TcpListener, TcpStream},
    thread,
    time::{self, SystemTime},
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
        Ok(_) => {
             let response = format!("Hello, client! The local time is: {:?}", SystemTime::now());
             stream.write(response.as_bytes()).unwrap();
            true
        }
        Err(e) => {
            println!("Error: {}", e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
