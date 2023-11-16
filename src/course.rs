use std::{fs::File, io::Write};

use super::checkpoint::{Checkpoint, Stepname};
use anyhow::Result;

/// Course is a series of numbered checkpoints with dedicated Start, Reset, and End checkpoints
pub struct Course {
    pub checkpoints: Vec<Checkpoint>,
}

impl Course {
    pub fn from_path(file: String) -> Result<Course> {
        let mut reader = csv::Reader::from_path(file)?;
        let iter = reader.deserialize();
        let mut checkpoints = Vec::new();
    
        for record in iter {
            let checkpoint: Checkpoint = record?;
            println!("{:?}", checkpoint);
            checkpoints.push(checkpoint);

        }
        Ok(Course {
            checkpoints: checkpoints
        })
    }
    
    pub fn export_to_path(self, path: String) -> Result<()> {
        let mut writer = csv::Writer::from_writer(vec![]);
        for checkpoint in self.checkpoints.iter() {
            writer.serialize(checkpoint)?;
        }
    
        let mut file = File::create(path)?;
        file.write_all(&writer.into_inner()?)?;
        Ok(())
    }
}