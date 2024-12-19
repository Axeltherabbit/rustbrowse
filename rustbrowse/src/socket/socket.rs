use std::net::{TcpStream};
use std::time::Duration;
use std::io::{Write, Read};

pub struct Socket {
    host: String,
    port: u16,
    pub stream: TcpStream
}

impl Socket{
    pub fn new(host: String, port: u16) -> Self {
        let hostport = format!("{}:{}", host, port);
        let stream =  TcpStream::connect(hostport).unwrap();

        stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();
        Self { host, port, stream}
    } 

    pub fn write_header(&mut self, b_headers: String) {

        self.stream.write(b_headers.as_bytes()).unwrap();
    }

    pub fn read_response(&mut self) -> String{
        let mut response = String::new();
        self.stream.read_to_string(&mut response).unwrap();
        return response;
    }
}
