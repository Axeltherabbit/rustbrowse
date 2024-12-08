
use url::{Url};
use std::env;
use std::net::{TcpStream};
use std::io::{Result, Write, Read};
use std::time::Duration;


fn parse_url(s_url: &str) -> Url {
    let url = Url::parse(s_url).unwrap();
    
    let scheme = url.scheme();
    if scheme != "http" {
        panic!("Error, only http supported");
    }
    
    url.host_str().unwrap();

    return url;
}

fn sock_connect(url: &Url) -> Result<TcpStream> {

    let host = url.host_str().unwrap();
    let port = url.port_or_known_default().unwrap();
    let hostport = format!("{host}:{port}");

    return TcpStream::connect(hostport);
}

fn build_header(url: &Url) -> String {

    let request_path_protocol = format!("GET {} HTTP/1.0", url.path());
    let request_host = format!("Host: {}", url.host_str().unwrap());
    let request_end = String::from("\r\n");

    return [request_path_protocol, request_host, request_end].join("\r\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Error. Pass a single argument as URL");
        return;
    }

    let url: Url = parse_url(args[1].as_str());

    let mut sock = sock_connect(&url).unwrap();   

    sock.set_read_timeout(Some(Duration::new(5, 0))).unwrap();
    sock.write(build_header(&url).as_bytes()).unwrap();

    let mut response: String = String::new();
    sock.read_to_string(&mut response).unwrap();

    println!("{}", response);
}
