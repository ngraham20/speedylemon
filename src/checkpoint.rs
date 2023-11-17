use std::io::Write;
use std::fs::File;

use serde::{Deserialize, Serialize};
use anyhow::Result;

// TODO: change x, y, z into [f32; 3], and implement serde
#[derive(Debug, Deserialize, Serialize)]
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
    pub radius:Option<i32>
}

impl Checkpoint {
    pub fn point(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Stepname {
    #[serde(rename = "start")]
    START,
    #[serde(rename = "reset")]
    RESET,
    #[serde(rename = "end")]
    END,
    #[serde(rename = "*")]
    CHECKPOINT,
}

#[cfg(test)]
mod tests {
    use super::*; 
}