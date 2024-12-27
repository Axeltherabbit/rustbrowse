use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;
use std::time::Duration;
use std::io;
use std::io::{Write, Read};

enum Stream {
    Tcp(TcpStream),
    Tls(TlsStream<TcpStream>),
}

impl Stream {
    // Example method to send data over the stream
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        match self {
            Stream::Tcp(stream) => stream.write(data),
            Stream::Tls(stream) => stream.write(data),
        }
    }

    // Example method to read data from the stream
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<usize> {
        match self {
            Stream::Tcp(stream) => stream.read_to_string(buffer),
            Stream::Tls(stream) => stream.read_to_string(buffer),
        }
    }
}


pub struct Socket {
    host: String,
    port: u16,
    stream: Stream,
    is_secure: bool
}

impl Socket{
    pub fn new(host: String, port: u16, is_secure: bool) -> Self {
        let hostport = format!("{}:{}", host, port);
        let tcp_stream =  TcpStream::connect(hostport).unwrap();
        tcp_stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();

        if is_secure {
            let connector = TlsConnector::new().unwrap();
            let tls_stream = connector.connect(host.as_str(), tcp_stream).unwrap();

            Self { host, port, stream: Stream::Tls(tls_stream), is_secure}
        }
        else {

            Self { host, port, stream: Stream::Tcp(tcp_stream), is_secure}
        }
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
