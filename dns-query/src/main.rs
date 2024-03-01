use std::{env::args, io::Result, net::UdpSocket, process::exit};

fn print_msg_name<'a>(msg: &'a [u8], p: &'a [u8], end: &'a [u8]) -> &'a [u8] {
    if &p[2..] > end {
        eprintln!("Endof message");
        exit(0x0100);
    }
    if p[0] & 0xC0 == 0xC0 {
        let k = ((p[0] & 0x3F) as u16) << 8 | p[1] as u16;
        let new_p = &p[2..];
        print_msg_name(msg, &msg[..k as usize], end);
        new_p
    } else {
        let len = p.iter().map(|&i| i as usize).sum::<usize>();
        let end_line = len + 1;
        if &p[end_line..] > end {
            eprintln!("End of message!");
        }
        let p_len = &p[len..];
        if !p_len.is_empty() {
            print_msg_name(msg, p_len, end)
        } else {
            &p_len[1..]
        }
    }
}

fn print_dns_message(msg_len: usize, msg: &[u8]) {
    if msg_len < 12 {
        eprintln!("Message too short to be valid");
        exit(0x0100);
    }
    let  message = msg;
    for i in 0..msg_len {
        let r = message[i];
        println!("Raw DNS message: {}\n {}\n {}\n {}\n", i, r, r, r);
    }
    println!("Transaction ID {} {}", message[0], message[1]);
    let qr = (message[2] & 0x80) >> 7;
    println!("QR = {} {}", qr, if qr != 0 { "response" } else { "query" });

    let opcode = (message[2] & 0x78) >> 3;
    println!("OPCODE: {}", opcode);
    match opcode {
        0 => {
            println!("Standard query");
        }
        1 => {
            println!("Reverse Query");
        }
        2 => {
            println!("Server status request");
        }
        _ => {
            println!("Reserved?");
        }
    }
    let aa = (message[2] & 0x04) >> 2;
    println!("AA = {} {}", aa, if aa != 0 { "Authoritative" } else { "" });
    let tc = (message[2] & 0x02) >> 1;
    println!(
        "TC = {} {}",
        tc,
        if tc != 0 { "Message truncated" } else { "" }
    );
    let rd = message[2] & 0x01;
    println!(
        "RD = {} {}",
        rd,
        if rd != 0 { "recursion desired" } else { "" }
    );

    let rcode = message[3] & 0x07;
    println!("RCODE: {}", rcode);
    match rcode {
        0 => {
            println!("Success");
        }
        1 => {
            println!("format error");
        }
        2 => {
            println!("Server failure");
        }
        3 => {
            println!("name error");
            return;
        }
        4 => {
            println!("Not Implemented");
            return;
        }
        5 => {
            println!("Refused");
            return;
        }
        _ => {
            println!("error End");
            return;
        }
    }
    if rcode != 0 {
        return;
    }
    let qdcount = (u16::from(message[4]) << 8) + u16::from(message[5]);
    let ancount = (u16::from(message[6]) << 8) + u16::from(message[7]);
    let nscount = (u16::from(message[8]) << 8) + u16::from(message[9]);
    let arcount = (u16::from(message[10]) << 8) + u16::from(message[11]);

    println!(
        "QDCOUNT: {}\nANCOUNT: {}\nNSCOUNT: {}\nARCOUNT: {}\n",
        qdcount, ancount, nscount, arcount
    );

    let mut p = &message[12..];
    let end_len = message.len() + msg_len;
    let mut end: Vec<u8> = vec![];
    end.push(end_len.try_into().unwrap());

    if qdcount != 0 {
        for i in 0..qdcount {
            if *p >= *end {
                println!("End of message!!");
            }
            println!("Query: {}", i + 1);
            println!("Name: ");
            let p_scope = print_msg_name(message, &p, &end);
            if p_scope[4..] > *end {
                println!("End of message!!!");
                return;
            }
            let typ = (u16::from(p_scope[0]) << 8) + u16::from(p_scope[1]);
            println!("Type: {}", typ);
            let new_p_scope = &p_scope[2..];
            let q_class = (u16::from(new_p_scope[0]) << 8) + u16::from(new_p_scope[1]);
            println!("Class: {}", q_class);
            p = &new_p_scope[2..];
        }
    }

    if ancount != 0 || nscount != 0 || arcount != 0 {
        let sum_range = ancount + nscount + arcount;
        for i in 0..sum_range {
            if *p >= *end {
                println!("End of message!!!!");
                return;
            }
            println!("Answer: {}", i + 1);
            println!("Name_: ");
            p = print_msg_name(message, &p, &end);
            if p[10..] > *end {
                println!("End of message!!!!!");
                return;
            }
            let typ = (u16::from(p[0]) << 8) + u16::from(p[1]);
            println!("Type second {}", typ);
            let new_p = &p[2..];
            let ttl = (u32::from(new_p[0]) << 24)
                | (u32::from(new_p[1]) << 16)
                | (u32::from(new_p[2]) << 8)
                | u32::from(new_p[3]);
            println!("TTL: {}", ttl);
            let new_4_p = &new_p[4..];
            let rdlen = (u16::from(new_4_p[0]) << 8) + u16::from(new_4_p[1]);
            println!("RDLENGTH: {}", rdlen);
            let len = new_4_p.iter().map(|&x| x as usize).sum::<usize>();
            if new_4_p[len..] > *end {
                println!("End of message!!!!!!!");
                return;
            }
            if rdlen == 4 && typ == 1 {
                println!(
                    "Address {}.{}.{}.{}",
                    new_4_p[0], new_4_p[1], new_4_p[2], new_4_p[3]
                );
            } else if typ == 15 && rdlen > 3 {
                let preference = (u16::from(new_4_p[0]) << 8) + u16::from(new_4_p[1]);
                println!(" pref: {}", preference);
                print!("MX: ");
                print_msg_name(msg, &new_4_p[2..], &end);
                println!();
            } else if rdlen == 16 && typ == 28 {
                print!("Address ");
                for j in (0..rdlen).step_by(2) {
                    print!("{:02x}{:02x}", new_4_p[j as usize], new_4_p[j as usize + 1]);
                    if j + 2 < rdlen {
                        print!(":");
                    }
                }
                println!();
            } else if typ == 16 {
                println!(
                    "TXT: '{}'",
                    std::str::from_utf8(&new_4_p[1..rdlen as usize]).unwrap()
                );
            } else if typ == 5 {
                print!("CNAME: ");
                print_msg_name(msg, new_4_p, &end);
                println!();
            }
            p = &new_4_p[rdlen as usize..];
        }
    }
    if p != end {
        println!("there is some unread data");
    }
    println!();
}


#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Invalid arguments");
    }
    if args[1].len() > 255 {
        println!("Hostname too long");
    }
    let mut typ: u8 = 0;
    match args[2].as_str() {
        "a" => {
            typ = 1;
        }
        "mx" => typ = 15,
        "txt" => typ = 16,
        "aaaa" => typ = 28,
        "any" => typ = 255,
        _ => {
            println!("Error");
        }
    }
    let socket = UdpSocket::bind("8.8.8.8:53");
    println!("Connecting on port 53");
    let mut query = [
        0xAB, 0xCD, // ID
        0x01, 0x00, // Set recursion
        0x00, 0x01, // QDCOUNT
        0x00, 0x00, // ANCOUNT
        0x00, 0x00, // NSCOUNT
        0x00, 0x00, // ARCOUNT
    ];

    //TODO: Encode Hostname into the query.
    let mut port = &query[2];
    println!("Porororor: {:X}", port);
    let mut host = args[1].as_bytes();
    Ok(())
}
