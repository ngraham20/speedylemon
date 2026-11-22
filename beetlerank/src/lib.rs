use std::{collections::HashMap, fs};
use itertools::Itertools;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

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
    pub client: reqwest::Client,
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
            client: reqwest::Client::builder().use_rustls_tls().build().expect("Failed to build https client"),
        }
    }

    #[tokio::main]
    pub async fn get_cups(&mut self) -> Result<&Vec<String>> {
        if self.cups.is_empty() {
            let url = "https://www.beetlerank.com/api/cups";
            let res = (self.client.get(url).send().await)?;
            let cups: Cups = (res.json().await)?;
            self.cups = cups.cups;
            self.cups.push("CUSTOM TRACKS".to_string());
        }
        Ok(&self.cups)
    }

    #[tokio::main]
    pub async fn post_log(&self, user: String, guildhall: String, file: String, ) -> Result<Vec<String>> {
        let url = "https://www.beetlerank.com/upload-log";
        let filepart = reqwest::multipart::Part::bytes(fs::read(file.clone()).unwrap())
            .file_name(file);
        let form = reqwest::multipart::Form::new()
            .text("user", user)
            .text("guildhall", guildhall)
            .part("file", filepart);
        let response_text = ((self.client.post(url).multipart(form).send().await)?.text().await)?.split("\n").into_iter().map(|s| s.to_string()).collect_vec();
        println!("{:?}", response_text);
        Ok(response_text)
    }

    #[tokio::main]
    pub async fn get_rank(&mut self, track: &String, user: &String) -> Result<&mut Ranking> {
        let url = format!("https://www.beetlerank.com/api/top3/{}/{}", track, user);
        let res = (self.client.get(&url).send().await)?;
        let mut data: Ranking = res.json().await.expect("Failed to parse JSON");
        if data.you.as_ref().unwrap()[1].name != *user {
            data.you = None;
        }

        Ok(self.rankings.entry(track.clone()).or_insert_with(||data))
    }
    #[tokio::main]
    pub async fn get_checkpoints(&self, track: &String) -> Result<String> {
        let url = format!("https://www.beetlerank.com/uploads/checkpoints/{}.csv", track);
        let res = (self.client.get(url).send().await)?;
        let data = (res.text().await)?;

        Ok(data)
    }

    #[tokio::main]
    pub async fn get_tracks(&mut self, cup: &String) -> Result<Vec<String>> {
        let url = format!("https://www.beetlerank.com/api/maps/{}", cup);
        let res = self.client.get(url).send().await.expect("Failed to get tracks from beetlerank");
        let tracks: Tracks = res.json().await.expect("Failed to deserialize json object");

        let tracks = self.tracks.entry(cup.clone()).or_insert_with(||{
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