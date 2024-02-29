use std::{env, net::{ToSocketAddrs}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Problem parsing arguments")
    } 
    let host = &args[1];
    let port = &args[2];
    
    match format!("{}:{}", host, port).to_socket_addrs() {
        Ok(addrs) => {
            for addr in addrs {
                println!("Resolved address: {:?}", addr.ip());
            }
        }
        Err(e) => {
            eprintln!("Error resolving address: {}", e);
        }
    }
}

