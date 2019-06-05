use crate::errors::*;
use m3u8_rs;
use reqwest;
use url::Url;
use m3u8_rs::playlist::Playlist;
use std::io::Write;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct TsPlaylist {
    base_url: String,
    playlist: Vec<String>,
}

impl TsPlaylist {
    pub fn parse(master_url: String) -> Result<Self>{
        let playlist = m3u8_parser(&master_url);
        let mut url = Url::parse(&master_url)?;
        url.path_segments_mut().map_err(|_| "cannot be base")?.pop();
        let base_url = url.as_str().to_string();
        Ok(TsPlaylist{
            base_url,
            playlist,
        })
    }
    
    pub fn download_to(&self, file_name: &str) -> Result<()>{
        //let temp_dir = TempDir::new(".afree")?;
        let mut ts_output = OpenOptions::new().append(true).create(true).open(file_name)?;
        self.playlist.iter().for_each(|seg_name| {
            //Inside Lamda function
            let seg_url = format!("{}/{}", self.base_url, seg_name);
            let mut resp = reqwest::get(&seg_url).unwrap();
            println!("Downloading segment: {}", seg_url);
            let mut downloaded_chunk : Vec<u8> = vec![];
            resp.copy_to(&mut downloaded_chunk);
            ts_output.write_all(&downloaded_chunk);
        });
        Ok(())
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
