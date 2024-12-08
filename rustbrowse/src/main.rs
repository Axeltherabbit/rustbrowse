
use url::{Url};
use std::env;
use std::net::{TcpStream};

fn parse_url(s_url: &str) -> Url {
    let url = Url::parse(s_url).unwrap();
    
    let scheme = url.scheme();
    if scheme != "http" {
        panic!("Error, only http supported");
    }
    
    url.host_str().unwrap();

    return url;
}

fn sock_connect(url: Url){

    let host = url.host_str().unwrap();
    let port = url.port_or_known_default().unwrap();
    let hostport = format!("{host}:{port}");

    let mut _stream = TcpStream::connect(hostport).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Error. Pass a single argument as URL");
        return;
    }

    let url = parse_url(args[1].as_str());
    sock_connect(url);   
}
