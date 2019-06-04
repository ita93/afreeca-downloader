#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate select;
extern crate reqwest;
extern crate xml;
extern crate encoding_rs;
extern crate m3u8_rs;

mod video_info;
mod ts_playlist;
mod test;

mod errors {
    error_chain!{
        foreign_links{
            IO(std::io::Error);
            UrlParse(url::ParseError);
            HttpRequest(reqwest::Error);
            SerdeParse(serde_urlencoded::de::Error);
            XmlParser(xml::reader::Error);
        }
    }
}

use crate::errors::*;

fn main() -> Result<()>{
    let res = video_info::get_video_info_from_url("http://vod.afreecatv.com/PLAYER/STATION/43597884")?;
    let m3u8_url = res.get_m3u8_url()?;
    println!("{:?}", m3u8_url);
    Ok(())
}

