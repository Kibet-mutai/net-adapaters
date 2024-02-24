use libc::{
     getifaddrs, ifaddrs, sockaddr_in, sockaddr_in6, AF_INET, AF_INET6
};
use std::alloc::{alloc, dealloc, Layout};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

type Addresses = *mut *mut ifaddrs;

#[derive(Debug)]
struct AfinetInfo {
    addr: IpAddr,
}

fn main() {
    unsafe {
        let layout = Layout::new::<Addresses>();
        let addr_ptr = alloc(layout);
        let my_addr = addr_ptr as Addresses;
        if getifaddrs(my_addr) != 0 {
            //return Err(format!("Get address returned an error: {}", getifaddrs(my_addr)));
            println!("Get address returned an error: {}", getifaddrs(my_addr));
        }
        let mut interfaces: Vec<AfinetInfo> = Vec::new();

        loop {
            let ifa_addr = (**my_addr).ifa_addr;
            match (*ifa_addr).sa_family as i32 {
                AF_INET => {
                    let int_addr = ifa_addr;
                    let sock_addr_v4: *mut sockaddr_in = int_addr as *mut sockaddr_in;
                    let in_addr = (*sock_addr_v4).sin_addr;
                    let mut ip_addr = Ipv4Addr::from(in_addr.s_addr);

                    if cfg!(target_endian = "little") {
                        ip_addr = Ipv4Addr::from(in_addr.s_addr.swap_bytes());
                    }
                    interfaces.push(AfinetInfo {
                        addr: IpAddr::V4(ip_addr),
                    });
                }
                AF_INET6 => {
                    let int_addr = ifa_addr;
                    let sock_addr_v6: *mut sockaddr_in6 = int_addr as *mut sockaddr_in6;
                    let in_addr = (*sock_addr_v6).sin6_addr;
                    let ip_addr = Ipv6Addr::from(in_addr.s6_addr);

                    interfaces.push(AfinetInfo {
                        addr: IpAddr::V6(ip_addr),
                    });
                }
                _ => {}
            }
            *my_addr = (**my_addr).ifa_next;
            if (*my_addr).is_null() {
                break;
            }
        }
        dealloc(addr_ptr, layout);
        println!("{:?}", interfaces);
    }
}
