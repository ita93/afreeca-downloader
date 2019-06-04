use crate::errors::*;
use m3u8_rs;
use reqwest;

use m3u8_rs::playlist::Playlist;
use std::io::Read;

#[derive(Debug)]
pub struct TsPlaylist {
    master_url: String,
    playlist: Vec<String>,
}

impl TsPlaylist {
    pub fn parse(master_url: String) -> Result<Self>{
        let playlist = m3u8_parser(&master_url);

        Ok(TsPlaylist{
            master_url,
            playlist,
        })
    }

}

fn m3u8_parser(input: &str) -> Vec<String>{
    let mut res = Vec::new();
    let mut response = reqwest::get(input).unwrap();
    let original_pl = response.text().unwrap().clone();

    let parser = m3u8_rs::parse_playlist_res(original_pl.as_bytes());

    match parser {
        Ok(Playlist::MasterPlaylist(pl)) => {
            let max_bw = pl.variants.iter().max_by_key(|var| &var.bandwidth);
            return m3u8_parser(&max_bw.unwrap().uri);
        },
        Ok(Playlist::MediaPlaylist(pl)) => {
            pl.segments.iter().map(|x|{
                res.push(x.uri.clone());
            }).count();
        },
        Err(e) => println!("ERROR: {:?}", e),
    }
    return res;
}
