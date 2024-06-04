use std::path::Path;
use std::{fs::File, io::Write};

use super::checkpoint::Stepname;
use super::checkpoint::Checkpoint;
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

    pub fn push_cp(&mut self, x: f32, y: f32, z: f32, radius: i32) {
        let idx = self.checkpoints.len();
        self.checkpoints.push(Checkpoint { step: idx as i16, stepname: Stepname::Checkpoint, x, y, z, radius })
    }
    pub fn add_reset(&mut self, x: f32, y: f32, z: f32, radius: i32) {
        self.reset = Some(Checkpoint {
            step: -1,
            stepname: Stepname::Reset,
            x, y, z,
            radius,
        });
    }

    pub fn from_path(path: &String) -> Result<Course> {
        let mut reader = csv::Reader::from_path(&path)?;
        let filename = Path::new(path).file_stem().unwrap().to_string_lossy().to_string();
        Course::from_reader(&filename, &mut reader)
    }

    pub fn from_reader<T: std::io::Read>(track: &String, reader: &mut csv::Reader<T>) -> Result<Course> {
        let iter = reader.deserialize();
        let mut checkpoints: Vec<Checkpoint> = Vec::new();
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
        let course = Course {
            name: String::from(track),
            checkpoints,
            reset,
        };
        Ok(course)
    }
    
    pub fn export(&self, path: String) -> Result<()> {
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

    // #[test]
    // fn test_export_import() -> Result<()> {
    //     let path = String::from("/tmp/speedylemon-test-course.csv");
    //     let course: Course = Course {
    //         name: String::from("speedylemon-test-course"),
    //         checkpoints: vec![Checkpoint {
    //             step: 0,
    //             stepname: Stepname::Checkpoint,
    //             x: 0.0,
    //             y: 1.0,
    //             z: 2.0,
    //             radius: 15,
    //         }],
    //         reset: Some(Checkpoint {
    //             step: -1,
    //             stepname: Stepname::Reset,
    //             x: 1.0,
    //             y: 2.0,
    //             z: 3.0,
    //             radius: 15,
    //         }),
    //     };
    //     course.export_to_path(path.clone())?;
    //     let imported = Course::from_path(path)?;
    //     assert_eq!(course.name, imported.name);
    //     assert!(course.reset.is_some() && imported.reset.is_some());
    //     assert_eq!(course.reset.unwrap(), imported.reset.unwrap());
    //     for (c, i) in course.checkpoints.iter().zip(imported.checkpoints) {
    //         assert_eq!(c, &i);
    //     }

    //     Ok(())
    // }

    // #[test]
    // fn test_course_from_url() {
    //     let url = String::from("http://localhost:3000/api/dev/uploads/checkpoints");
    //     let course = Course::from_url(url.clone()).context(format!("Failed to load checkpoint file from url: {}", url)).unwrap();
    //     assert!(course.reset.is_some());
    //     assert!(course.checkpoints.len() == 3);
    //     assert_eq!(course.checkpoints[0], Checkpoint {
    //         step: 0,
    //         stepname: Stepname::Start,
    //         x: 0.0,
    //         y: 1.0,
    //         z: 2.0,
    //         radius: 15
    //     });
    //     assert_eq!(course.checkpoints[1], Checkpoint {
    //         step: 1,
    //         stepname: Stepname::Checkpoint,
    //         x: 2.0,
    //         y: 3.0,
    //         z: 4.0,
    //         radius: 15,
    //     });
    //     assert_eq!(course.checkpoints[2], Checkpoint {
    //         step: 2,
    //         stepname: Stepname::End,
    //         x: 3.0,
    //         y: 4.0,
    //         z: 5.0,
    //         radius: 15,
    //     });
    // }
}