use serde::{Serialize, Deserialize};
use url::Url;

use error_chain;

error_chain!{
    foreign_links{
        IO(std::io::Error);
        UrlParse(url::ParseError);
        SerdeParse(serde_urlencoded::de::Error);
    }
}

//Fields: Ok([("http://vod.afreecatv.com/embed.php?isAfreeca", "false"), ("autoPlay", "true"), ("showChat", "true"), 
//("szBjId", "afenglish"), ("nStationNo", "18027548"), ("nBbsNo", "59587689"), ("nTitleNo", "43597884"), 
//("szCategory", "00080000"), ("szPart", "NORMAL"), ("type", "station"), ("szSysType", "html5")])

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
