
use url::{Url};
use std::env;


fn parse_url(s_url: &str) -> Url {
    let url = Url::parse(s_url).unwrap();
    
    let scheme = url.scheme();
    if scheme != "http" {
        panic!("Error, only http supported");
    }

    url.host_str().unwrap();

    return url;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Error. Pass a single argument as URL");
        return;
    }

    let url = parse_url(args[1].as_str());
}
