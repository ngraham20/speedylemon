use std::{fs::{create_dir_all, File}, path::Path, io::Write, time::Duration};

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

use crate::util::{Importable, Exportable};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RaceLap {
    laptime: u64,
    splits: Splits,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Splits {
    pb: Vec<u64>,
    best: Vec<u64>,
}

impl RaceLap {

    pub fn update_best(&mut self, splits: Vec<u64>) -> Result<()> {
        for 
        self.best = splits;
        Ok(())
    }
}

impl Importable for RaceLap {
    fn import(path: String) -> anyhow::Result<Self> where Self: Sized {
        log::info!("Importing checkpoint splits from {}", path);
        let toml_str = std::fs::read_to_string(path).context("Failed to read toml file")?;
        let splits = toml::from_str(&toml_str).context("Failed to parse toml")?;
        Ok(splits)
    }
}

impl Exportable for RaceLap {
    fn export(&self, path: String) -> anyhow::Result<()> {
        log::info!("Exporting checkpoint splits to {}", path);
        create_dir_all(Path::new(&path).parent().unwrap()).context("Failed to create splits directory")?;
        let toml_str = toml::to_string(&self)?;
        std::fs::write(path, toml_str)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::splits;

    use super::*;

    #[test]
    fn test_export_import() -> Result<()> {
        let path = String::from("/tmp/speedylemon-test-splits.csv");
        let splits_vecs: Vec<u64> = vec![
            123,
            456,
            789,
            987,
            654,
            321,
        ];
        let splits: RaceLap = RaceLap {
            pb: splits_vecs.clone(),
            best: splits_vecs.clone(),
        };
        splits.export(path.clone()).context("Failed to export splits")?;
        let imported = RaceLap::import(path).context("Failed to import splits")?;
        assert_eq!(splits, imported);
        Ok(())
    }
}