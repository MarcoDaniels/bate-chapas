use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Child, Command};

const CHAPA_CONFIG: &str = "chapas/config";
const CHAPA_SOURCE: &str = "chapas/source";
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
    code: String,
    environment: Option<Vec<Environment>>,
    init: Option<Process>,
}

impl Config {
    pub fn write(config: &Config, contents: String) -> std::io::Result<()> {
        // TODO: check if it exists already and throw error
        fs::write(format!("{}/{}.json", CHAPA_CONFIG, config.name), contents)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Source;

impl Source {
    pub fn install(config: &Config) -> std::io::Result<Child> {
        Command::new("git")
            .current_dir(CHAPA_SOURCE)
            .arg("clone")
            .arg(&config.code)
            .spawn()
    }

    pub fn init(config: &Config) -> std::io::Result<Child> {
        match &config.init {
            Some(init) => Command::new(&init.command)
                .current_dir(format!("{}/{}", CHAPA_SOURCE, config.name))
                .spawn(),
            None => Command::new("echo").arg("no init").spawn(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub message: String,
}

impl Status {
    pub fn write(config: &Config, contents: String) -> std::io::Result<()> {
        fs::write(format!("{}/{}.txt", CHAPA_STATUS, config.name), contents)
    }
}
