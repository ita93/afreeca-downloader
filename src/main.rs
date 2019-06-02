#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate select;
extern crate reqwest;
mod video_info;


mod errors {
    error_chain!{
        foreign_links{
            IO(std::io::Error);
            UrlParse(url::ParseError);
            HttpRequest(reqwest::Error);
            SerdeParse(serde_urlencoded::de::Error);
        }
    }
}


fn main() {
    println!("Hello world!");
}
