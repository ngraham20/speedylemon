use std::io::Write;
use std::fs::File;

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Checkpoint {
    #[serde(rename = "STEP")]
    pub step: i16,
    #[serde(rename = "STEPNAME")]
    pub stepname: Stepname,
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
    #[serde(rename = "RADIUS")]
    pub radius:Option<i32>
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