use std::fmt;
use std::net::TcpStream;
use std::os::unix::ffi::OsStrExt;
use std::{
    collections::HashMap,
    io::{Read, Write},
    ops::Range,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

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


#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
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
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

impl Connection {
    pub fn new(parse_url: &str) -> Result<Connection, Error> {
        let url = ParsedUrl::from(&parse_url).unwrap();
        let stream = TcpStream::connect(format!("{}:80", url.host)).unwrap();
        Ok(Connection { url, stream })
    }

    pub fn set_headers(&mut self) -> Result<(), Error> {
        let  request = Request::new();
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
        Ok(())
    }

    pub fn get_response(&mut self, response: &str) -> Option<(String, String)> {
        let mut parts = response.split("\r\n\r\n");
        if let (Some(headers), Some(body)) = (parts.next(), parts.next()) {
            Some((headers.to_string(), body.to_string()))
        } else {
            None
        }
    }

    pub fn send_request(&mut self) -> Result<(), Error> {
        let _ = self.set_headers().unwrap();
        //let mut headers = HashMap::new();
        //let mut body = Vec::new();
        let mut buffer = String::new();
           let bytes_read = self.stream.read_to_string(&mut buffer).unwrap();
           let res = &buffer[..bytes_read];
           if let Some((headers, body)) = self.get_response(res) {
               println!("Headers:\n{}\n\nBody:\n{}", headers, body);
           }
           println!("String Data: {:?}", res);
        Ok(())
    }

    //TODO: Implement post method.
    pub fn post(&mut self, data: String) -> Result<(), Error> {
         let request = Request::new();
        self.stream
            .write_all(format!("POST {} HTTP/1.1\r\n", self.url.path).as_bytes())
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
        self.stream
            .write_all(format!("Content-type: application/json\r\n").as_bytes())
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
        //let mut json: T = serde_json::from_str(data).unwrap();
        self.stream.write_all(data.as_bytes()).unwrap();
        let mut buf = String::new();
        match self.stream.read_to_string(&mut buf) {
            Ok(_) => {
                println!("Response from post: {:?}", buf);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }


        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let url = "https://example.com";
    //let body = reqwest::get(url).await?.text().await?;
    //println!("body: {:?}", body);
    let mut connection = Connection::new(&url).unwrap();
    //println!("Connection: {:?}", connection);
    connection.send_request().unwrap();
  let data = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    //connection.post(data.to_string()).unwrap();
    Ok(())
}
