#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
}

pub trait Core {
    fn load(&self) -> Option<HashMap<String, String>>;
    fn write(&self, content: &HashMap<String, String>) -> Result<(), io::Error>;
    fn add(&self, content: &HashMap<String, String>) -> Result<(), io::Error>;
}

pub fn new(path: &str) -> File {
    let file = File {
        path: path.to_string(),
    };

    file
}

impl Core for File {
    fn load(&self) -> Option<HashMap<String, String>> {
        match fs::read_to_string(&self.path) {
            Ok(v) => {
                let data: Vec<&str> = v.split('\n').collect();
                let mut res: HashMap<String, String> = HashMap::new();

                for i in data {
                    if i.contains("=") {
                        let r: Vec<&str> = i.split('=').collect();

                        res.insert(r[0].to_string(), r[1].to_string());
                    }
                }

                Some(res)
            }
            Err(_) => None,
        }
    }

    fn write(&self, content: &HashMap<String, String>) -> Result<(), io::Error> {
        let mut res: String = String::new();

        for i in content {
            res.push_str(&format!("{}={}\n", i.0, i.1).to_string());
        }

        match fs::write(&self.path, res) {
            Ok(()) => Ok(()),
            Err(v) => Err(v),
        }
    }

    fn add(&self, content: &HashMap<String, String>) -> Result<(), io::Error> {
        let mut res: String = String::new();
        let data: HashMap<String, String> = self.load().unwrap();

        for i in &data {
            res.push_str(&format!("{}={}\n", i.0, i.1).to_string());
        }

        for i in content {
            res.push_str(&format!("{}={}\n", i.0, i.1).to_string());
        }

        match fs::write(&self.path, res) {
            Ok(()) => Ok(()),
            Err(v) => Err(v),
        }
    }
}
