use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use crate::hash;

type StorageData = HashMap<String, String>;

pub struct Storage {
    file_name: String,
    path: PathBuf,
}

pub enum Error {
    NoFile,
    NotFound,
}

pub fn format_error(e: Error) -> String {
    match e {
        Error::NoFile => "No storage file".to_string(),
        Error::NotFound => "Not found".to_string(),
    }
}

impl Storage {
    // TODO: support different path and file name
    pub fn new() -> Storage {
        let file_name = "storage.json".to_string();
        let working_dir = std::env::current_dir().unwrap();
        let path = working_dir.join(&file_name);

        Storage { file_name, path }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn get(&self, uid: &str) -> Result<String, Error> {
        if !self.exists() {
            return Err(Error::NoFile);
        }

        let file = read_to_string(&self.file_name).unwrap();
        let value: StorageData = serde_json::from_str(&file).unwrap();

        match value.get(uid) {
            Some(url) => Ok(url.to_string()),
            None => Err(Error::NotFound),
        }
    }

    pub fn save(&self, url: &str) -> Result<String, Error> {
        let short_val = hash::gen(url);

        if !self.exists() {
            return Err(Error::NoFile);
        }

        let file = read_to_string(&self.file_name).unwrap();
        let mut value: StorageData = serde_json::from_str(&file).unwrap();

        match value.get(&short_val) {
            Some(_) => Ok(short_val),
            None => {
                value.insert(short_val.clone(), url.to_string());
                let json = serde_json::to_string(&value).unwrap();
                std::fs::write(&self.file_name, json).unwrap();
                Ok(short_val)
            }
        }
    }

    pub fn list(&self) -> Result<HashMap<String, String>, Error> {
        if !self.exists() {
            return Err(Error::NoFile);
        }

        let file = read_to_string(&self.file_name).unwrap();
        let value: StorageData = serde_json::from_str(&file).unwrap();

        Ok(value)
    }
}
