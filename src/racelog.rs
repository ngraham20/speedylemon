use std::{fs::{File, create_dir_all}, io::Write, path::Path};
use crate::util::{Importable, Exportable};

use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use log;

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceLogEntry {
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
    #[serde(rename = "Z")]
    pub z: f32,
    #[serde(rename = "SPEED")]
    pub speed: f32,
    #[serde(rename = "ANGLE_CAM")]
    pub cam_angle: f32,
    #[serde(rename = "ANGLE_BEETLE")]
    pub beetle_angle: f32,
    #[serde(rename = "TIME")]
    pub timestamp: f64,
    #[serde(rename = "ACCELERATION")]
    pub acceleration: f32,
    #[serde(rename = "MAP_ANGLE")]
    pub map_angle: f32,
}

impl Importable for Vec<RaceLogEntry> {
    fn import(path: &String) -> Result<Self> where Self: Sized {
        log::info!("Importing racelog from path: {}", path);
        let mut reader = csv::Reader::from_path(path)?;
        let iter = reader.deserialize();
        let mut entries: Vec<RaceLogEntry> = Vec::new();
        for record in iter {
            entries.push(record?);
        }
        Ok(entries)
    }
}

impl Exportable for Vec<RaceLogEntry> {
    fn export(&self, path: String) -> Result<()> {
        log::info!("Exporting racelog to path: {}", path);
        create_dir_all(Path::new(&path).parent().unwrap()).context("Failed to create racelog directory")?;
        let mut writer = csv::Writer::from_writer(vec![]);
        for entry in self.iter() {
            writer.serialize(entry)?;
        }

        let mut file = File::create(path).context("Failed to create racelog file")?;
        file.write_all(&writer.into_inner()?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_import() {
        // TODO: test racelog import/export
    }
}