use std::{borrow::BorrowMut, path::Path};

use serde::{Deserialize, Serialize};

use crate::config::{DatabaseConfig, HoggConfig};

pub const DB_VERSION: &str = "1.0.0";

#[derive(Debug, err_derive::Error)]
pub enum Error {
    #[error(display = "IO Error")]
    IoError(#[error(source)] std::io::Error),
    #[error(display = "Serde Json Error")]
    JsonError(#[error(source)] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Detection<T> {
    pub viewed: bool,
    pub data: T,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DbStruct<T>
where
    T: Clone,
{
    pub version: String,
    pub detections: Vec<Detection<T>>,
}

pub struct HoggDatabase<T: Clone + Serialize + for<'a> Deserialize<'a>> {
    pub path: String,
    pub structure: DbStruct<T>,

    pub config: DatabaseConfig,
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> HoggDatabase<T> {
    pub fn from_file(path: String, config: HoggConfig) -> Result<Self, Error> {
        if !Path::new(&path).exists() {
            std::fs::write(
                &path,
                serde_json::to_string(&DbStruct::<T> {
                    version: DB_VERSION.to_string(),
                    detections: Vec::new(),
                })?,
            )?;
        }
        let structure = serde_json::from_str(&std::fs::read_to_string(path.clone())?)?;
        Ok(Self {
            path,
            structure,
            config: config.database,
        })
    }

    pub fn from_file_unconfigured(path: String) -> Result<Self, Error> {
        if !Path::new(&path).exists() {
            std::fs::write(
                &path,
                serde_json::to_string(&DbStruct::<T> {
                    version: DB_VERSION.to_string(),
                    detections: Vec::new(),
                })?,
            )?;
        }
        let structure = serde_json::from_str(&std::fs::read_to_string(path.clone())?)?;
        Ok(Self {
            path,
            structure,
            config: DatabaseConfig::default(),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        std::fs::write(self.path.clone(), serde_json::to_string(&self.structure)?)?;
        Ok(())
    }

    pub fn add_detection(&mut self, detection: T) {
        self.structure.detections.push(Detection {
            viewed: false,
            data: detection,
        });
    }

    pub fn get_unviewed_detections(
        &mut self,
        mark_as_viewed: bool,
    ) -> Result<Vec<&mut Detection<T>>, Error> {
        // TODO: fix saving of database
        let mut detections = Vec::new();
        for detection in self.structure.detections.iter_mut() {
            if !detection.viewed {
                if mark_as_viewed {
                    detection.viewed = true;
                }
                detections.push(detection);
            }
        }

        Ok(detections)
    }

    pub fn get_detections(&self, offset: usize, limit: usize) -> Vec<&Detection<T>> {
        self.structure.detections[offset..limit + offset]
            .iter()
            .collect()
    }
}
