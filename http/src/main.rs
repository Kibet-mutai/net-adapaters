use std::net::TcpStream;
use std::str::from_utf8;
use std::{
    collections::HashMap,
    io::{Read, Write},
    ops::Range,
};
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    ParseUrlError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "URL parsing error",)
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub path: String,
}

impl ParsedUrl {
    pub fn from(url: &str) -> Result<ParsedUrl, Error> {
        let addr = if url.starts_with("http") || url.starts_with("https") {
            url.to_owned()
        } else {
            format!("http://{}", url)
        };

        let mut split = addr.split("://");

        let scheme = match split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::ParseUrlError),
        };

        split = match split.next() {
            Some(v) => v.split("/"),
            None => return Err(Error::ParseUrlError),
        };

        let host = match split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::ParseUrlError),
        };

        let mut path = String::new();
        loop {
            match split.next() {
                Some(v) => path.push_str(format!("/{}", v).as_str()),
                None => {
                    if path.is_empty() {
                        path.push('/');
                    }
                    break;
                }
            }
        }

        Ok(ParsedUrl { scheme, host, path })
    }
}

#[derive(Debug)]
pub struct Connection {
    url: ParsedUrl,
    stream: TcpStream,
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
        }
    }
}

pub struct Request {
    method: Method,
    headers: HashMap<String, String>,
    query_strings: String,
    range: Option<Range<usize>>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: Method::GET,
            headers: HashMap::new(),
            query_strings: String::new(),
            range: None,
            body: None,
        }
    }

    pub fn set_method(mut self, method: Method) -> Request {
        self.method = method;
        self
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn set_headers(mut self, headers: HashMap<String, String>) -> Request {
        self.headers = headers;
        self
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn set_range(mut self, range: Range<usize>) -> Request {
        self.range = Some(range);
        self
    }

    pub fn get_range(&self) -> &Option<Range<usize>> {
        &self.range
    }

    pub fn get_query_strings(&self) -> &String {
        &self.query_strings
    }

    pub fn get_body(&self) -> &Option<Vec<u8>> {
        &self.body
    }

    pub fn get_content_length(&self) -> usize {
        if let Some(body) = &self.body {
            body.len()
        } else {
            0
        }
    }
}
impl Connection {
    pub fn new(parse_url: &str) -> Result<Connection, Error> {
        let url = ParsedUrl::from(&parse_url).unwrap();
        let stream = TcpStream::connect(format!("{}:80", url.host)).unwrap();
        Ok(Connection { url, stream })
    }

    pub fn send_request(&mut self) -> Result<(), Error> {
        let mut request = Request::new();
        self.stream
            .write_all(format!("GET {} HTTP/1.1\r\n", self.url.path).as_bytes())
            .unwrap();
        self.stream
            .write_all(format!("HOST: {}\r\n", self.url.host).as_bytes())
            .unwrap();
        for header in request.get_headers() {
            self.stream
                .write_all(format!("{}: {}\r\n", header.0, header.1).as_bytes())
                .unwrap();
        }
        self.stream
            .write_all(format!("Content-Length: {}\r\n", request.get_content_length()).as_bytes())
            .unwrap();
        if let Some(range) = request.get_range() {
            self.stream
                .write_all(format!("Range: bytes={}-{}\r\n", range.start, range.end).as_bytes())
                .unwrap();
        }

        self.stream.write_all(b"Connection: Close\r\n").unwrap();
        self.stream.write_all(b"\r\n").unwrap();

        if let Some(body) = request.get_body() {
            self.stream.write_all(body.as_slice()).unwrap();
        }

        self.stream.write_all(b"\r\n\r\n").unwrap();
        let mut buf = String::new();
        match self.stream.read_to_string(&mut buf) {
            Ok(_) => {
                let mut response = buf.split("/");
                let scheme = response.next();
                let protocol = response.next();
                let status_code = response.next();
                println!("Response:\n{:?}", buf);
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let url = "https://example.com";

    let mut connection = Connection::new(&url).unwrap();
    //println!("Connection: {:?}", connection);
    connection.send_request().unwrap();
    Ok(())
}
