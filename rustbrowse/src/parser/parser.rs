use std::collections::HashMap;
pub use url::Url;

pub struct HttpStatus{
    version: String,
    status: String,
    explanation: String
}
impl Default for HttpStatus {
    fn default () -> HttpStatus {
        HttpStatus {status:String::new(), version:String::new(), explanation:String::new()}
    }
}


pub struct Parser {}

impl Parser{
    pub fn parse_response(response: &str, status: &mut HttpStatus, headers: &mut HashMap<String, String>, body: &mut String){
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

    pub fn parse_url(s_url: &str) -> Url {
        let url = Url::parse(s_url).unwrap();
        
        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            panic!("Error, only http supported");
        }
        
        url.host_str().unwrap();

        return url;
    }
}
