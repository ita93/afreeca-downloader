use crate::errors::*;
use m3u8_rs;
use reqwest;
use url::Url;
use m3u8_rs::playlist::Playlist;
use std::io::Write;
use std::process::{Command, Child};
use std::fs::OpenOptions;
use pbr::ProgressBar;
use tempdir::TempDir;

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
        let temp_dir = TempDir::new(".afree")?;
        let file_path = temp_dir.path().join(file_name);
        let mut ts_output = OpenOptions::new().append(true).create(true).open(&file_path)?;
        let no_segments = self.playlist.len() as u64;
        let mut pb = ProgressBar::new(no_segments);
        pb.format("╢▌▌░╟");

        self.playlist.iter().for_each(|seg_name| {
            //Inside Lamda function
            let seg_url = format!("{}/{}", self.base_url, seg_name);
            let mut resp = reqwest::get(&seg_url).unwrap();
            //println!("Downloading segment: {}", seg_url);
            let mut downloaded_chunk : Vec<u8> = vec![];
            resp.copy_to(&mut downloaded_chunk);
            ts_output.write_all(&downloaded_chunk);
            pb.inc();
        });
        pb.finish_print("Download done, waiting for converting....");
        let output_mp4 = convert_ts_to_mp4(&file_path.to_str().unwrap(), file_name)?;
        output_mp4.wait_with_output()?;
        drop(ts_output);
        temp_dir.close()?;
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

pub fn convert_ts_to_mp4(input_file: &str, output_file: &str) -> Result<Child> {
    let output_name = format!("{}.mp4", &output_file);
    let output = Command::new("ffmpeg")
                        .arg("-i")
                        .arg(input_file)
                        .arg("-c:v")
                        .arg("libx264")
                        .arg("-c:a")
                        .arg("copy")
                        .arg(output_name)
                        .spawn();
    Ok(output?)
}
