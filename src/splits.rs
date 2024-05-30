use std::{fs::{create_dir_all, File}, path::Path, io::Write, time::Duration};

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

use crate::{util::{Importable, Exportable}, checkpoint};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct RaceLap {
    pb_laptime: u64,
    splits: Splits,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
struct Splits {
    pb: Vec<u64>,
    best: Vec<u64>,
}

impl RaceLap {
    pub fn new(checkpoint_times: &Vec<Duration>) -> RaceLap {
        RaceLap {
            pb_laptime: checkpoint_times.last().unwrap().as_millis() as u64,
            splits: Splits {
                pb: splits(&checkpoint_times),
                best: splits(&checkpoint_times),
            }
        }
    }
}

impl Importable for RaceLap {
    fn import(path: &String) -> anyhow::Result<Option<Self>> where Self: Sized {
        log::info!("Importing checkpoint splits from {}", path);
        if !Path::new(path).exists() {
            return Ok(None)
        }
        let toml_str = std::fs::read_to_string(path).context("Failed to read toml file")?;
        let splits = toml::from_str(&toml_str).context("Failed to parse toml")?;
        Ok(Some(splits))
    }
}

impl Exportable for RaceLap {
    fn export(&self, path: String) -> anyhow::Result<()> {
        log::info!("Exporting checkpoint splits to {}", path);
        create_dir_all(Path::new(&path).parent().unwrap()).context("Failed to create splits directory")?;
        let toml_str = toml::to_string(&self)?;
        std::fs::write(path, toml_str)?;
        Ok(())
    }
}

fn splits(checkpoints: &Vec<Duration>) -> Vec<u64> {
    checkpoints[1..].iter().enumerate().map(|(idx, split)| split.saturating_sub(checkpoints[idx]).as_millis() as u64).collect()
}

pub fn calculate_pb(previous_data: &RaceLap, checkpoint_times: &Vec<Duration>) -> RaceLap {
    let laptime = checkpoint_times.last().unwrap().as_millis() as u64;
    let splits = splits(&checkpoint_times);

    let mut new_data = previous_data.clone();

    let mut new_best_splits: Vec<u64> = Vec::new();

    println!("Previous data: {:?}", previous_data.splits.best);
    println!("Splits: {:?}", splits);
    for (a, b) in previous_data.splits.best.iter().zip(splits.iter()) {
        println!("{}, {}", a, b);
        new_best_splits.push(u64::min(*a, *b));
    }

    new_data.splits.best = new_best_splits;

    if laptime < previous_data.pb_laptime {
        new_data.pb_laptime = laptime;
        new_data.splits.pb = splits;
    }

    new_data
}

/// Updates the track data with new PB information if necessary
pub fn update_track_data(checkpoint_times: &Vec<Duration>, path: String) -> Result<()> {
    let new_data;
    if let Some(previous_data) = RaceLap::import(&path)? {
        new_data = calculate_pb(&previous_data, &checkpoint_times);
        if new_data != previous_data {
            new_data.export(path)?;
        }
    } else {
        new_data = RaceLap::new(&checkpoint_times);
        new_data.export(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_pb() {
        let old_data = RaceLap::new(&vec![
            Duration::from_millis(0),
            Duration::from_millis(200),
            Duration::from_millis(400),
            Duration::from_millis(600)
        ]);

        let final_data = calculate_pb(&old_data, &vec![
            Duration::from_millis(0),
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(300),
        ]);

        assert_eq!(final_data, RaceLap {
            pb_laptime: 300,
            splits: Splits {
                pb: vec![100, 100, 100],
                best: vec![100, 100, 100],
            }
        })
    }

    #[test]
    fn test_new_best_splits() {
        let old_data = RaceLap::new(&vec![
            Duration::from_millis(0),
            Duration::from_millis(200),
            Duration::from_millis(400),
            Duration::from_millis(500)
        ]);

        let final_data = calculate_pb(&old_data, &vec![
            Duration::from_millis(0),
            Duration::from_millis(100),
            Duration::from_millis(300),
            Duration::from_millis(500),
        ]);

        assert_eq!(final_data, RaceLap {
            pb_laptime: 500,
            splits: Splits {
                pb: vec![200, 200, 100],
                best: vec![100, 200, 100],
            }
        })
    }

    #[test]
    fn test_no_change() {
        let old_data = RaceLap::new(&vec![
            Duration::from_millis(0),
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(300)
        ]);
        let final_data = calculate_pb(&old_data, &vec![
            Duration::from_millis(0),
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(300),
        ]);
        assert_eq!(old_data, final_data);
    }

    #[test]
    fn test_worse_run() {
        let old_data = RaceLap::new(&vec![
            Duration::from_millis(0),
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(300)
        ]);
        let final_data = calculate_pb(&old_data, &vec![
            Duration::from_millis(0),
            Duration::from_millis(400),
            Duration::from_millis(600),
            Duration::from_millis(800),
        ]);
        assert_eq!(final_data, RaceLap {
            pb_laptime: 300,
            splits: Splits {
                pb: vec![100, 100, 100],
                best: vec![100, 100, 100],
            }
        });
    }

    #[test]
    fn test_export_import() -> Result<()> {
        let path = String::from("/tmp/speedylemon-test-splits.toml");
        let splits_vecs: Vec<Duration> = vec![
            Duration::from_millis(0),
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(300),
        ];
        let splits = RaceLap::new(&splits_vecs);
        splits.export(path.clone()).context("Failed to export splits")?;
        let imported = RaceLap::import(&path).context("Failed to import splits")?;
        assert_eq!(splits, imported.unwrap());
        Ok(())
    }
}