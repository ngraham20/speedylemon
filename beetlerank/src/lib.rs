use std::{collections::HashMap, fs};
use itertools::Itertools;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use futures::executor::block_on;

use anyhow::Result;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Ranking {
    #[serde(rename = "ranking")]
    pub top_3: Vec<Rank>,
    pub you: Option<Vec<Rank>>,
}

impl Default for Ranking {
    fn default() -> Self {
        Ranking {
            top_3: Vec::new(),
            you: Some(Vec::new()),
        }
    }
}

pub struct BeetleRank {
    pub cups: Vec<String>,
    pub tracks: HashMap<String, Vec<String>>,
    pub rankings: HashMap<String, Ranking>,
}

#[derive(Deserialize)]
struct Cups {
    cups: Vec<String>
}

#[derive(Deserialize)]
struct Tracks {
    maps: Vec<String>,
}

impl BeetleRank {
    pub fn new() -> BeetleRank {
        BeetleRank {
            cups: Vec::new(),
            tracks: HashMap::new(),
            rankings: HashMap::new(),
        }
    }

    pub fn get_cups(&mut self) -> Result<&Vec<String>> {
        if self.cups.is_empty() {
            let client = reqwest::Client::builder().use_rustls_tls().build()?;
            let url = "https://www.beetlerank.com/api/cups";
            let res = block_on(client.get(url).send())?;
            let cups: Cups = block_on(res.json())?;
            self.cups = cups.cups;
            self.cups.push("CUSTOM TRACKS".to_string());
        }
        Ok(&self.cups)
    }

    pub fn post_log(&self, user: String, guildhall: String, file: String, ) -> Result<Vec<String>> {
        let client = reqwest::Client::builder().use_rustls_tls().build()?;
        let url = "https://www.beetlerank.com/upload-log";
        let filepart = reqwest::multipart::Part::bytes(fs::read(file.clone()).unwrap())
            .file_name(file);
        let form = reqwest::multipart::Form::new()
            .text("user", user)
            .text("guildhall", guildhall)
            .part("file", filepart);
        let response_text = block_on(block_on(client.post(url).multipart(form).send())?.text())?.split("\n").into_iter().map(|s| s.to_string()).collect_vec();
        println!("{:?}", response_text);
        Ok(response_text)
    }

    pub fn get_rank(&mut self, track: &String, user: &String) -> Result<&Ranking> {
        let ranking = self.rankings.entry(track.clone()).or_insert_with(|| {
            let client = reqwest::Client::builder().use_rustls_tls().build().expect("Failed to create builder");
            let url = format!("https://www.beetlerank.com/api/top3/{}/{}", track, user);
            let res = block_on(client.get(url).send()).expect("Failed to reach beetlerank");
            let mut data: Ranking = block_on(res.json()).expect("Failed to parse JSON");
            if data.you.as_ref().unwrap()[1].name != *user {
                data.you = None;
            }
            data
        });

        Ok(ranking)
    }

    pub fn get_checkpoints(track: &String) -> Result<String> {
        let client = reqwest::Client::builder().use_rustls_tls().build()?;
        let url = format!("https://www.beetlerank.com/uploads/checkpoints/{}.csv", track);
        let res = block_on(client.get(url).send())?;
        let data = block_on(res.text())?;

        Ok(data)
    }

    pub fn get_tracks(&mut self, cup: &String) -> Result<Vec<String>> {
        let tracks = self.tracks.entry(cup.clone()).or_insert_with(||{
            let client = reqwest::Client::builder().use_rustls_tls().build().expect("Failed to build client");
            let url = format!("https://www.beetlerank.com/api/maps/{}", cup);
            let res = block_on(client.get(url).send()).expect("Failed to get tracks from beetlerank");
            let tracks: Tracks = block_on(res.json()).expect("Failed to deserialize json object");
            tracks.maps
        });
        Ok(tracks.clone())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(default)]
pub struct Rank {
    #[serde(rename = "pos")]
    pub rank: u32,
    #[serde(rename = "time", deserialize_with = "deserialize_number_from_string")]
    pub timestamp: String,
    pub name: String,
    #[serde(rename = "realtime", deserialize_with = "deserialize_number_from_string")]
    pub laptime: f64,
    date: String,
    map: String,
    file: String,
}

impl Default for Rank {
    fn default() -> Self {
        Rank {
            rank: 0,
            timestamp: String::new(),
            name: String::new(),
            laptime: 0.0,
            date: String::new(),
            map: String::new(),
            file: String::new(),
        }
    }
}