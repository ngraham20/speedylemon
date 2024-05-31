use std::collections::HashMap;
use itertools::Itertools;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use futures::executor::block_on;

use anyhow::Result;

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Ranking {
    #[serde(rename = "ranking")]
    top_3: Vec<Rank>,
    you: Option<Vec<Rank>>,
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
        }
    }

    pub fn mock() -> BeetleRank {
        let mut tracks = HashMap::new();
        tracks.insert(String::from("Cup 1"), vec!["C1T1", "C1T2", "C1T3"].iter().map(|s| s.to_string()).collect_vec());
        tracks.insert(String::from("Cup 2"), vec!["C2T1", "C2T2", "C2T3"].iter().map(|s| s.to_string()).collect_vec());
        tracks.insert(String::from("Cup 3"), vec!["C3T1", "C3T2", "C3T3"].iter().map(|s| s.to_string()).collect_vec());
        tracks.insert(String::from("Cup 4"), vec!["C4T1", "C4T2", "C4T3"].iter().map(|s| s.to_string()).collect_vec());
        BeetleRank {
            cups: vec!["Cup 1", "Cup 2", "Cup 3", "Cup 4"]
                .iter().map(|s| s.to_string()).collect_vec(),
            tracks: HashMap::new()
        }
    }
    pub fn get_cups(&mut self) -> Result<&Vec<String>> {
        if self.cups.is_empty() {
            let client = reqwest::Client::builder().use_rustls_tls().build()?;
            let url = "https://www.beetlerank.com/api/cups";
            let res = block_on(client.get(url).send())?;
            let cups: Cups = block_on(res.json())?;
            self.cups = cups.cups;
        }
        Ok(&self.cups)
    }

    pub fn get_tracks(&mut self, cup: String) -> Result<Vec<String>> {
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
    rank: u32,
    #[serde(rename = "time", deserialize_with = "deserialize_number_from_string")]
    timestamp: String,
    name: String,
    #[serde(rename = "realtime", deserialize_with = "deserialize_number_from_string")]
    laptime: f64,
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;


    #[derive(Deserialize)]
    struct DevResponse {
        succeed: bool,
    }

    #[derive(Deserialize)]
    struct Cups {
        cups: Vec<String>,
    }

    #[derive(Deserialize)]
    struct Maps {
        maps: Vec<String>,
    }

    #[test]
    fn test_motd() -> Result<()> {
        let url = "http://localhost:3000/api/dev/info";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }

    #[test]
    fn test_devapi() -> Result<()> {
        let url = "http://localhost:3000/api/dev/info";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }

    #[test]
    fn test_cups() -> Result<()> {
        let url = "http://localhost:3000/api/dev/cups";
        let response: Cups = reqwest::blocking::get(url)?.json()?;
        assert_eq!(response.cups, vec![
            String::from("TYRIA CUP"),
            String::from("GUILDHALL CUP")
        ]);
        Ok(())
    }

    #[test]
    fn test_maps() -> Result<()> {
        let url = "http://localhost:3000/api/dev/maps/TYRIA CUP";
        let response: Maps = reqwest::blocking::get(url)?.json()?;
        assert_eq!(response.maps, vec![
            String::from("TYRIA GENDARRAN")
        ]);
        Ok(())
    }

    #[test]
    fn test_map_rankings() -> Result<()> {
        let url = "http://localhost:3000/api/dev/top3/DEV";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }

    #[test]
    fn test_user_rankings() -> Result<()> {
        let url = "http://localhost:3000/api/dev/top3/DEV/Test User";
        let response: Ranking = reqwest::blocking::get(url)?.json()?;
        println!("{:?}", response);
        assert_eq!(response.top_3.len(), 3);
        assert!(response.you.is_some());
        assert_eq!(response.you.clone().unwrap().len(), 3);
        assert_eq!(response.top_3[0], Rank {
            rank: 1,
            timestamp: String::from("01:00,000"),
            name: String::from("First"),
            laptime: 60.0,
            date: String::from("2022-09-18 02:21:16"),
            map: String::from("TYRIA GENDARRAN"),
            file: String::from("test.csv"),
        });
        assert_eq!(response.you.unwrap()[1], Rank {
            rank: 72,
            timestamp: String::from("01:00,000"),
            name: String::from("Seventy-Second"),
            laptime: 60.0,
            date: String::from("2022-09-18 02:21:16"),
            map: String::from("TYRIA GENDARRAN"),
            file: String::from("test.csv"),
        });
        Ok(())
    }
}