use std::{fs::OpenOptions, io::Write};

use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

#[derive(Debug, Deserialize, Serialize)]
pub struct Checkpoint {
    #[serde(rename = "STEP")]
    step: i16,
    #[serde(rename = "STEPNAME")]
    stepname: String,
    #[serde(rename = "X")]
    x: f32,
    #[serde(rename = "Y")]
    y: f32,
    #[serde(rename = "Z")]
    z: f32,
    #[serde(rename = "RADIUS")]
    radius:Option<i32>
}

pub fn load_checkpoint_file(file: String) -> Result<Vec<Checkpoint>> {
    let mut reader = csv::Reader::from_path(file)?;
    let iter = reader.deserialize();
    let mut checkpoints = Vec::new();

    for record in iter {
        let checkpoint = record?;
        println!("{:?}", checkpoint);
        checkpoints.push(checkpoint);
    }

    Ok(checkpoints)
}

pub fn write_checkpoints_to_file(checkpoints: Vec<Checkpoint>, path: String) -> Result<()> {
    use std::fs::File;

    let mut writer = csv::Writer::from_writer(vec![]);
    for checkpoint in checkpoints.iter() {
        writer.serialize(checkpoint)?;
    }

    let mut file = File::create(path)?;
    file.write_all(&writer.into_inner()?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
}