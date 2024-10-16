use std::time::Duration;

use anyhow::Result;

pub trait Importable {
    fn import(path: &String) -> Result<Option<Self>> where Self: Sized;
}

pub trait Exportable {
    fn export(&self, path: String) -> Result<()>;
}
    
pub fn euclidian_distance_3d(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    ((a[0]-b[0]).powi(2) + (a[1]-b[1]).powi(2) + (a[2]-b[2]).powi(2)).sqrt()
}

pub fn euclidian_distance_2d(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    ((a[0]-b[0]).powi(2) + (a[2]-b[2]).powi(2)).sqrt()
}

pub trait Timestamp {
    fn timestamp(&self) -> String;
}

impl Timestamp for Duration {
    fn timestamp(&self) -> String {
        format!("{:02}:{:02}:{:03}", self.as_secs()/60, self.as_secs()%60, self.subsec_millis())
    }
}