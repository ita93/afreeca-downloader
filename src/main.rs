#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate select;
extern crate reqwest;
extern crate xml;
extern crate encoding_rs;
extern crate m3u8_rs;
extern crate clap;
extern crate pbr;
extern crate tempdir;

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
use clap::{App, Arg};
use regex::Regex;
use crate::ts_playlist::convert_ts_to_mp4;

fn main() -> Result<()>{
    let reg = Regex::new("http://vod.afreecatv.com/PLAYER/STATION/.+").unwrap();
    let matches = App::new("Afreeca Video Downloader")
                    .version("0.1")
                    .author("Phi Nguyen <phind.uet@gmail.com>")
                    .about("A simple cli program to download video from Afreeca").
                    arg(Arg::with_name("video_url")
                        .help("Set the video url to download")
                        .required(true)
                        .index(1))
                    .get_matches();
    //let res = video_info::get_video_info_from_url("http://vod.afreecatv.com/PLAYER/STATION/43597884")?;
    let video_url = matches.value_of("video_url").unwrap();
    if !reg.is_match(video_url) {
        return Err("This is not a valid Afreeca video url".into())            
    }
    let res = video_info::get_video_info_from_url(video_url)?;
    let m3u8_url = res.get_m3u8_url()?;
    m3u8_url.download_to(res.get_video_name())?;
    Ok(())
}

