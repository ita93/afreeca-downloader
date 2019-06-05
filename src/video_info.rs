use serde::{Serialize, Deserialize};
use url::Url;
use regex::Regex;
use select::document::Document;
use select::predicate::{Name, Attr};
use crate::errors::*;
use crate::ts_playlist::TsPlaylist;
use std::io::Read;
use xml::reader::{XmlEvent, EventReader};

//Fields: Ok([("http://vod.afreecatv.com/embed.php?isAfreeca", "false"), ("autoPlay", "true"), ("showChat", "true"), 
//("szBjId", "afenglish"), ("nStationNo", "18027548"), ("nBbsNo", "59587689"), ("nTitleNo", "43597884"), 
//("szCategory", "00080000"), ("szPart", "NORMAL"), ("type", "station"), ("szSysType", "html5")])
pub const GET_VIDEO_INFO_URL: &str = "http://afbbs.afreecatv.com:8080/api/video/get_video_info.php";

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInfo{
    #[serde(skip)]
    video_name: String,
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
    pub fn from_str(inp: &str, vid_name: String) -> Result<VideoInfo>{
        let formated_url = Url::parse(inp)?;
        if let Some(params) = formated_url.query() {
            let mut parsed_params:VideoInfo = match serde_urlencoded::from_str(params){
                Ok(r) => r,
                Err(ori_err) => return Err(Error::from(ori_err))
            };
            parsed_params.video_name = vid_name; //move
            Ok(parsed_params)
        }else{
            Err("Input string is an invalid url".into())
        }
    }
    
    //Getting and parsing XML file.

    pub fn get_m3u8_url(&self) -> Result<TsPlaylist> {
        let mut tag_name = String::new(); 
        let mut is_m3u8 = false;

        let client = reqwest::Client::new();
        let mut res = client.post(GET_VIDEO_INFO_URL).query(&self).send()?;
        let response_body = res.text_with_charset("utf-8")?;
        let normalized_body = response_body.replace("\u{feff}", "");
        let parser = EventReader::from_str(&normalized_body);
        
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement{name, attributes, .. }) => {
                    tag_name = name.to_string();
                    if tag_name == "video" {
                        is_m3u8 = attributes.iter().any(|attr| {
                            attr.name.local_name == "duration".to_string()
                        });
                    }
                },
                Ok(XmlEvent::Characters(text)) => {
                    if tag_name == "video" && is_m3u8 {                        
                        return TsPlaylist::parse(text);
                    }
                },
                Err(e) => {
                     return Err(Error::from(e));
                },
                _ => {
                    //ignore them
                }
            }
        }
        Err("XML parser: Unexpected Error".into())
    }

    //getters
    pub fn get_video_name(&self) -> &str {
        &self.video_name
    }
}

//video_url : view-source:http://vod.afreecatv.com/PLAYER/STATION/43597884
pub fn get_video_info_from_url(video_url: &str) -> Result<VideoInfo>{
    let mut res = reqwest::get(video_url)?;
    let mut buf = String::new();
    res.read_to_string(&mut buf)?;
    let doc = Document::from(buf.as_str());    
    if let Some(head) = doc.find(Name("head")).nth(0){
        //og:title
        let video_name = match head.find(Attr("property", "og:title")).nth(0) {
            Some(nd) => nd.attr("content").unwrap_or("UNKNOWN").to_string(),
            None => String::new(),
        };
        //og::video
        if let Some(node) = head.find(Attr("property", "og:video")).nth(0) {
            let res = node.attr("content").unwrap(); //dont need to check this case
            VideoInfo::from_str(&res, video_name)
        } else {
            Err("Couldn't find video tag".into())
        }
    }else{
        Err("Couldn't find head tag".into())
    }
}

