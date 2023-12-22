use std::{fs::File, io::Write};

use crate::checkpoint::Stepname;
use crate::checkpoint::Checkpoint;
use anyhow::Result;
use log;

/// Course is a series of numbered checkpoints with dedicated Start, Reset, and End checkpoints
#[derive(Clone, Debug)]
pub struct Course {
    pub name: String,
    pub checkpoints: Vec<Checkpoint>,
    pub reset: Option<Checkpoint>,
}

impl Course {
    pub fn new() -> Course {
        Course {
            name: String::new(),
            checkpoints: Vec::new(),
            reset: None,
        }
    }

    pub fn from_url(url: String) -> Result<Course> {
        log::info!("Loading race course from url: {}", url);
        let path = url.clone();
        let course_name = std::path::Path::new(&path).file_stem().unwrap().to_str().unwrap();
        
        let response = reqwest::blocking::get(url)?.text()?;
        let mut reader = csv::Reader::from_reader(response.as_bytes());
        let iter = reader.deserialize();
        let mut checkpoints = Vec::new();
        let mut reset: Option<Checkpoint> = None;

        for record in iter {
            let checkpoint: Checkpoint = record?;
            match checkpoint.stepname {
                Stepname::Reset => { reset = Some(checkpoint) },
                Stepname::Checkpoint |
                Stepname::Start |
                Stepname::End => { checkpoints.push(checkpoint) },
            }

        }
        checkpoints.sort_by(|a, b| a.step.partial_cmp(&b.step).unwrap());
        Ok(Course {
            name: String::from(course_name),
            checkpoints: checkpoints,
            reset: reset,
        })
    }

    pub fn from_path(path: String) -> Result<Course> {
        log::info!("Loading race course from path: {}", path);
        let course_name = std::path::Path::new(&path).file_stem().unwrap().to_str().unwrap();

        let mut reader = csv::Reader::from_path(&path)?;
        let iter = reader.deserialize();
        let mut checkpoints = Vec::new();
        let mut reset: Option<Checkpoint> = None;
    
        for record in iter {
            let checkpoint: Checkpoint = record?;
            match checkpoint.stepname {
                Stepname::Reset => { reset = Some(checkpoint) },
                Stepname::Checkpoint |
                Stepname::Start |
                Stepname::End => { checkpoints.push(checkpoint) },
            }

        }
        checkpoints.sort_by(|a, b| a.step.partial_cmp(&b.step).unwrap());
        Ok(Course {
            name: String::from(course_name),
            checkpoints: checkpoints,
            reset: reset,
        })
    }
    
    pub fn _export_to_path(&self, path: String) -> Result<()> {
        let mut writer = csv::Writer::from_writer(vec![]);

        if let Some(cp) = &self.reset {
            writer.serialize(cp)?;
        }

        for checkpoint in self.checkpoints.iter() {
            writer.serialize(checkpoint)?;
        }
    
        let mut file = File::create(path)?;
        file.write_all(&writer.into_inner()?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    #[test]
    fn test_course_from_url() {
        let url = String::from("https://www.beetlerank.com/uploads/checkpoints/TYRIA GENDARRAN.csv");
        let course = Course::from_url(url.clone()).context(format!("Failed to load checkpoint file from url: {}", url)).unwrap();
        assert!(course.checkpoints.len() == 14)
    }
}