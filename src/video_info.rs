use serde::{Serialize, Deserialize};
use url::Url;
use regex::Regex;
use select::document::Document;
use select::predicate::{Name, Attr};
use crate::errors::*;
use std::io::Read;

//Fields: Ok([("http://vod.afreecatv.com/embed.php?isAfreeca", "false"), ("autoPlay", "true"), ("showChat", "true"), 
//("szBjId", "afenglish"), ("nStationNo", "18027548"), ("nBbsNo", "59587689"), ("nTitleNo", "43597884"), 
//("szCategory", "00080000"), ("szPart", "NORMAL"), ("type", "station"), ("szSysType", "html5")])
pub const GET_VIDEO_INFO_URL: &str = "http://afbbs.afreecatv.com:8080/api/video/get_video_info.php";

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInfo{
    #[serde(rename="isAfreeca")]
    is_afreeca: bool,
    #[serde(rename="autoPlay")]
    auto_play: bool,
    #[serde(rename="showChat")]
    show_chat: bool,
    #[serde(rename="szBjId")]
    sz_bj_id: String,
    #[serde(rename="nStationNo")]
    n_station_no: String,
    #[serde(rename="nBbsNo")]
    n_bbs_no: String,
    #[serde(rename="nTitleNo")]
    n_title_no: String,
    #[serde(rename="szCategory")]
    sz_category: String,
    #[serde(rename="szPart")]
    sz_part: String,
    r#type: String,
    #[serde(rename="szSysType")]
    sz_sys_type: String,
}

impl VideoInfo{
    //parse from full path url.
    pub fn from_str(inp: &str) -> Result<VideoInfo>{
        let formated_url = Url::parse(inp)?;
        if let Some(params) = formated_url.query() {
            let parsed_params:VideoInfo = match serde_urlencoded::from_str(params){
                Ok(r) => r,
                Err(ori_err) => return Err(Error::from(ori_err))
            };
            Ok(parsed_params)
        }else{
            Err("Input string is an invalid url".into())
        }
    }
}

//video_url : view-source:http://vod.afreecatv.com/PLAYER/STATION/43597884
pub fn get_video_info_from_url(video_url: &str) -> Result<VideoInfo>{
    let mut res = reqwest::get(video_url)?;
    let mut buf = String::new();
    res.read_to_string(&mut buf)?;
    let doc = Document::from(buf.as_str());    
    if let Some(head) = doc.find(Name("head")).nth(0){
        if let Some(node) = head.find(Attr("property", "og:video")).nth(0) {
            let res = node.attr("content").unwrap(); //dont need to check this case
            VideoInfo::from_str(&res)
        } else {
            Err("Couldn't find video tag".into())
        }
    }else{
        Err("Couldn't find head tag".into())
    }
}

//video
pub fn get_m3u8_url(video_url: &str) -> String {
    
}
