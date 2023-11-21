use std::{fs::File, io::Write};

use crate::checkpoint::Stepname;

use super::checkpoint::Checkpoint;
use anyhow::Result;
use log;

/// Course is a series of numbered checkpoints with dedicated Start, Reset, and End checkpoints
#[derive(Clone)]
pub struct Course {
    pub checkpoints: Vec<Checkpoint>,
    pub reset: Option<Checkpoint>,
    pub current_checkpoint: usize,
}

#[derive(PartialEq)]
pub enum CourseState {
    Racing,
    WaitingToStart,
    ApproachingFinishLine,
    Finished,
}

impl Course {

    pub fn peek_next(&self) -> Checkpoint {
        self.checkpoints[self.current_checkpoint]
    }

    pub fn restart(&mut self) -> CourseState {
        self.current_checkpoint = 0;
        CourseState::WaitingToStart
    }
    pub fn collect_checkpoint(&mut self) -> CourseState {
        match self.current_checkpoint {
            _x if _x < self.checkpoints.len() - 2 => {
                self.current_checkpoint += 1;
                CourseState::Racing
            },
            _x if _x < self.checkpoints.len() - 1 => {
                self.current_checkpoint += 1;
                CourseState::ApproachingFinishLine
            }
            _x if _x == self.checkpoints.len() - 1 => { CourseState::Finished },
            _ => { CourseState::Racing }
        }
    }
    pub fn from_path(path: String) -> Result<Course> {
        log::info!("Loading race course from path: {}", path);
        let mut reader = csv::Reader::from_path(path)?;
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
            checkpoints: checkpoints,
            reset: reset,
            current_checkpoint: 0usize,
        })
    }
    
    pub fn export_to_path(&self, path: String) -> Result<()> {
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