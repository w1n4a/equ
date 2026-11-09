#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
}

#[derive(Debug)]
pub enum Types {
    String(String),
    Vector(Vec<String>),
}

pub trait Core {
    fn load(&self) -> Option<HashMap<String, Types>>;
    fn write(&self, content: &HashMap<String, Types>) -> Result<(), io::Error>;
    fn add(&self, content: &HashMap<String, Types>) -> Result<(), io::Error>;
}

pub fn new(path: &str) -> File {
    let file = File {
        path: path.to_string(),
    };

    file
}

impl Core for File {
    fn load(&self) -> Option<HashMap<String, Types>> {
        match fs::read_to_string(&self.path) {
            Ok(v) => {
                let data: Vec<&str> = v.split('\n').collect();
                let mut res: HashMap<String, Types> = HashMap::new();

                for i in data {
                    if i.contains("=") {
                        let r: Vec<&str> = i.split('=').collect();

                        if r[1].starts_with('[') && r[1].ends_with(']') {
                            let tmp: Vec<&str> = r[1].split(['[', ']', ',', ' ']).collect();
                            let mut re: Vec<String> = Vec::new();

                            for i in tmp {
                                if i != "" {
                                    re.push(i.to_string());
                                }
                            }
                            res.insert(r[0].to_string(), Types::Vector(re));
                        } else {
                            res.insert(r[0].to_string(), Types::String(r[1].to_string()));
                        }
                    }
                }

                Some(res)
            }
            Err(_) => None,
        }
    }

    fn write(&self, content: &HashMap<String, Types>) -> Result<(), io::Error> {
        let mut res: String = String::new();

        for i in content {
            match i.1 {
                Types::String(v) => res.push_str(&format!("{}={}\n", i.0, v).to_string()),
                Types::Vector(v) => {
                    let mut str: String = String::from(format!("{}=[", i.0));
                    for i in 0..v.len() {
                        if i == v.len() - 1 {
                            str.push_str(&format!("{}]", &v[i]).to_string());
                        } else {
                            str.push_str(&format!("{}, ", &v[i]).to_string());
                        }
                    }
                    res.push_str(&str);
                }
            }
        }

        match fs::write(&self.path, res) {
            Ok(()) => Ok(()),
            Err(v) => Err(v),
        }
    }

    fn add(&self, content: &HashMap<String, Types>) -> Result<(), io::Error> {
        let mut res: String = String::new();
        let data: HashMap<String, Types> = self.load().unwrap();

        for i in &data {
            match i.1 {
                Types::String(v) => res.push_str(&format!("{}={}\n", i.0, v).to_string()),
                Types::Vector(v) => {
                    let mut str: String = String::from(format!("{}=[", i.0));
                    for i in 0..v.len() {
                        if i == v.len() - 1 {
                            str.push_str(&format!("{}]", &v[i]).to_string());
                        } else {
                            str.push_str(&format!("{}, ", &v[i]).to_string());
                        }
                    }
                    res.push_str(&str);
                }
            }
        }

        for i in content {
            match i.1 {
                Types::String(v) => res.push_str(&format!("{}={}\n", i.0, v).to_string()),
                Types::Vector(v) => {
                    let mut str: String = String::from(format!("{}=[", i.0));
                    for i in 0..v.len() {
                        if i == v.len() - 1 {
                            str.push_str(&format!("{}]", &v[i]).to_string());
                        } else {
                            str.push_str(&format!("{}, ", &v[i]).to_string());
                        }
                    }
                    res.push_str(&str);
                }
            }
        }

        match fs::write(&self.path, res) {
            Ok(()) => Ok(()),
            Err(v) => Err(v),
        }
    }
}
