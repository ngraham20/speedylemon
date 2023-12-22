use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Ranking {
    #[serde(rename = "ranking")]
    top_3: Vec<Rank>,
    you: Vec<Rank>,
}

impl Default for Ranking {
    fn default() -> Self {
        Ranking {
            top_3: Vec::new(),
            you: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
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
    use anyhow::Result;
    use serde::Deserialize;

    use super::*;


    #[derive(Deserialize)]
    struct DevResponse {
        succeed: bool
    }

    #[derive(Deserialize)]
    struct Cups {
        cups: Vec<String>
    }

    #[derive(Deserialize)]
    struct Maps {
        maps: Vec<String>
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
        assert_eq!(response.top_3[0], Rank {
            rank: 1,
            timestamp: String::from("01:00,000"),
            name: String::from("First"),
            laptime: 60.0,
            date: String::from("2022-09-18 02:21:16"),
            map: String::from("TYRIA GENDARRAN"),
            file: String::from("test.csv"),
        });
        assert_eq!(response.you[1], Rank {
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