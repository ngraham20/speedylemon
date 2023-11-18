use std::io::Write;
use std::fs::File;

use serde::{Deserialize, Serialize};
use anyhow::Result;

// TODO: change x, y, z into [f32; 3], and implement serde
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(default)]
pub struct Checkpoint {
    #[serde(rename = "STEP")]
    pub step: i16,
    #[serde(rename = "STEPNAME")]
    pub stepname: Stepname,
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
    #[serde(rename = "Z")]
    pub z: f32,
    #[serde(rename = "RADIUS")]
    pub radius: i32
}

impl Checkpoint {
    pub fn new() -> Checkpoint {
        Checkpoint {
            step: 0,
            stepname: Stepname::Checkpoint,
            x: 0f32,
            y: 0f32,
            z: 0f32,
            radius: 15i32,
        }
    }
    pub fn point(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Default for Checkpoint {
    fn default() -> Checkpoint {
        Checkpoint {
            step: 0,
            stepname: Stepname::Checkpoint,
            x: 0f32,
            y: 0f32,
            z: 0f32,
            radius: 15i32,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Stepname {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "*")]
    Checkpoint,
}

#[cfg(test)]
mod tests {
    use super::*; 
}