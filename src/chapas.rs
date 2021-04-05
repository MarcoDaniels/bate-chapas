use serde::{Deserialize, Serialize};
use std::fs;

const CHAPA_CONFIG: &str = "chapas/config";
const CHAPA_STATUS: &str = "chapas/status";

#[derive(Serialize, Deserialize)]
pub struct Process {
    command: String,
    arg: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Environment {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub code: String,
    pub environment: Option<Vec<Environment>>,
    pub init: Option<Process>,
}

impl Config {
    pub fn write(name: String, content: String) -> std::io::Result<()> {
        fs::write(format!("{}/{}.json", CHAPA_CONFIG, name), content)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}

impl Status {
    pub fn write(name: String, content: String) -> std::io::Result<()> {
        fs::write(format!("{}/{}.txt", CHAPA_STATUS, name), content)
    }
}
