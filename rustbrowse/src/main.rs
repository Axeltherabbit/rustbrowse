use std::env;
use std::collections::HashMap;

mod socket; 
use socket::socket::Socket;

mod parser;
use parser::parser::{Parser, HttpStatus, Url};


const HEADER_NEWLINE: &str = "\r\n";

fn build_header(url: &Url) -> String {
    let request_path_protocol = format!("GET {} HTTP/1.0", url.path());
    let request_host = format!("Host: {}", url.host_str().unwrap());
    let request_end = String::from(HEADER_NEWLINE);

    return [request_path_protocol, request_host, request_end].join(HEADER_NEWLINE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Error. Pass a single argument as URL");
        return;
    }

    let url: Url = Parser::parse_url(args[1].as_str());
    
    let is_secure = url.scheme() == "https";

    let mut sock = Socket::new(String::from(url.host_str().unwrap()),
                           url.port_or_known_default().unwrap(),
                            is_secure);

    sock.write_header(build_header(&url));

    let response = sock.read_response();

    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut status = HttpStatus::default();
    Parser::parse_response(response.as_str(), &mut status, &mut headers, &mut body);

    println!("{}", body);

}
