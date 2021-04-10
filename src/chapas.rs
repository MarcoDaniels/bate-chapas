use serde::ser::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::{Child, Command, Output};

const CHAPA_CONFIG: &str = "chapas/config";
const CHAPA_SOURCE: &str = "chapas/source";
const CHAPA_STATUS: &str = "chapas/status";

#[derive(Serialize, Deserialize)]
pub struct Environment {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    command: String,
    args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    code: String,
    init: Option<Process>,
    run: Option<Process>,
    environment: Option<Vec<Environment>>,
}

impl Config {
    pub fn write(config: &Config, contents: String) -> std::io::Result<()> {
        let path = format!("{}/{}.json", CHAPA_CONFIG, config.name);

        if Path::new(path.as_str()).exists() {
            Result::Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists))
        } else {
            fs::write(path, contents)
        }
    }

    pub fn read(name: &String) -> serde_json::Result<Config> {
        let path = format!("{}/{}.json", CHAPA_CONFIG, name);

        match fs::File::open(path) {
            Ok(file) => serde_json::from_reader(file),
            Err(err) => serde_json::Result::Err(serde_json::error::Error::custom(err)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Source;

impl Source {
    pub fn install(config: &Config) -> std::io::Result<Output> {
        Command::new("git")
            .current_dir(CHAPA_SOURCE)
            .arg("clone")
            .arg(&config.code)
            .output()
    }

    pub fn init(config: &Config) -> std::io::Result<Child> {
        match &config.init {
            Some(init) => Command::new(&init.command)
                // TODO: use args
                // .args()
                .current_dir(format!("{}/{}", CHAPA_SOURCE, config.name))
                .spawn(),
            None => Command::new("echo").arg("no init").spawn(),
        }
    }

    pub fn run(config: &Config) -> &'static str {
        println!("{:?}", config.run);
        "something"
    }
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub message: String,
}

impl Status {
    pub fn write(config: &Config, contents: &str) -> Option<()> {
        fs::write(format!("{}/{}.txt", CHAPA_STATUS, config.name), contents).ok()
    }
}
