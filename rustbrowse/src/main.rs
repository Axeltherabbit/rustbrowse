
use url::{Url};
use std::env;
use std::collections::HashMap;

mod socket; // This line is necessary to include the socket module
use socket::socket::Socket;


fn parse_url(s_url: &str) -> Url {
    let url = Url::parse(s_url).unwrap();
    
    let scheme = url.scheme();
    if scheme != "http" {
        panic!("Error, only http supported");
    }
    
    url.host_str().unwrap();

    return url;
}

fn build_header(url: &Url) -> String {

    let request_path_protocol = format!("GET {} HTTP/1.0", url.path());
    let request_host = format!("Host: {}", url.host_str().unwrap());
    let request_end = String::from("\r\n");

    return [request_path_protocol, request_host, request_end].join("\r\n");
}

struct HttpStatus{
    version: String,
    status: String,
    explanation: String
}
impl Default for HttpStatus {
    fn default () -> HttpStatus {
        HttpStatus {status:String::new(), version:String::new(), explanation:String::new()}
    }
}

fn parse_response(response: &str, status: &mut HttpStatus, headers: &mut HashMap<String, String>, body: &mut String){
    let mut lines = response.lines();
    let mut statusline = lines.next().unwrap().splitn(3, " ");

    status.version = String::from(statusline.next().unwrap());
    status.status = String::from(statusline.next().unwrap());
    status.explanation = String::from(statusline.next().unwrap());

    let mut headerline = lines.next().unwrap();
    while headerline != "" {
        let mut keyval = headerline.splitn(2, ":");
        // println!("{}", headerline);
        headers.insert(
             keyval.next().unwrap().to_lowercase(), 
             String::from(keyval.next().unwrap().trim()));

        headerline = lines.next().unwrap();
    }

    assert!(!headers.contains_key("transfer-encoding"));
    assert!(!headers.contains_key("content-encoding"));
    
    body.push_str(lines.fold(String::new(), |a, b| a + b + "\n").as_str());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Error. Pass a single argument as URL");
        return;
    }

    let url: Url = parse_url(args[1].as_str());

    let mut sock = Socket::new(String::from(url.host_str().unwrap()),
                           url.port_or_known_default().unwrap());

    sock.write_header(build_header(&url));

    let response = sock.read_response();

    println!("{}", response);
  
    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut status = HttpStatus::default();
    parse_response(response.as_str(), &mut status, &mut headers, &mut body);
}
