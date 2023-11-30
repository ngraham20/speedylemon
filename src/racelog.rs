use serde::{Serialize, Deserialize};

pub trait RaceLog {
    fn export(path: String);
    fn import(path: String) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceLogEntry {
    #[serde(rename = "X")]
    x: f32,
    #[serde(rename = "Y")]
    y: f32,
    #[serde(rename = "Z")]
    z: f32,
    #[serde(rename = "SPEED")]
    speed: f32,
    #[serde(rename = "ANGLE_CAM")]
    cam_angle: f32,
    #[serde(rename = "ANGLE_BEETLE")]
    beetle_angle: f32,
    #[serde(rename = "TIME")]
    timestamp: f64,
    #[serde(rename = "ACCELERATION")]
    acceleration: f32,
    #[serde(rename = "MAP_ANGLE")]
    map_angle: f32,
}

impl RaceLog for Vec<RaceLogEntry> {
    fn export(path: String) {

    }

    fn import(path: String) -> Self {
        Vec::new()
    }
}