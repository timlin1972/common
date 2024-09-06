use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use fs2::FileExt;
use serde::{Deserialize, Serialize};

pub const DEF_NAME: &str = "center_default";
const CFG_FILE: &str = "./cfg.json";
const KEY: &str = "an example very very secret key.";   // length is fixed
const SHELL: &str = "sh";

#[derive(Serialize, Deserialize)]
pub struct Alias {
    pub alias: String,
    pub cmd: String,
}

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    pub name: String,
    pub startup: Vec<String>,
    pub polling: Vec<String>,
    pub aliases: Vec<Alias>,
    pub key: String,
    pub shell: String,
}

pub fn init() {
    let path = Path::new(CFG_FILE);

    if !path.exists() {
        let cfg = Cfg {
            name: DEF_NAME.to_owned(),
            startup: vec![
                "load plugins".to_owned(),
            ],
            polling: vec![
                "action plugin mqtt report myself".to_owned(),
                "action plugin sysinfo report myself".to_owned(),
            ],
            aliases: vec![
                Alias {
                    alias: "!d".to_owned(),
                    cmd: "status plugin devinfo".to_owned(),
                },
                Alias {
                    alias: "!s".to_owned(),
                    cmd: "status plugin sysinfo".to_owned(),
                },
                Alias {
                    alias: "!l".to_owned(),
                    cmd: "status plugin logs".to_owned(),
                },
                Alias {
                    alias: "!c".to_owned(),
                    cmd: "status plugin cfg".to_owned(),
                },
            ],
            key: KEY.to_owned(),
            shell: SHELL.to_owned(),
        };

        let file_content = serde_json::to_string_pretty(&cfg).unwrap();

        // lock
        let mut file = File::create(CFG_FILE).unwrap();
        file.lock_exclusive().unwrap();

        file.write_all(file_content.as_bytes()).unwrap();

        // unlock
        file.unlock().unwrap();
    }
}

pub fn get_cfg() -> Cfg {
    // lock
    let file = File::open(CFG_FILE).unwrap();
    file.lock_shared().unwrap();

    let file_content = fs::read_to_string(CFG_FILE).unwrap();
    let cfg: Cfg = serde_json::from_str(&file_content).unwrap();

    // unlock
    file.unlock().unwrap();

    cfg
}

pub fn get_name() -> String {
    get_cfg().name
}

pub fn get_shell() -> String {
    get_cfg().shell
}

pub fn get_key() -> String {
    get_cfg().key
}

pub fn set_name(name: &str) -> Result<(), String> {
    // lock
    let file = File::open(CFG_FILE).unwrap();
    file.lock_exclusive().unwrap();

    let file_content = fs::read_to_string(CFG_FILE).unwrap();
    let mut cfg: Cfg = serde_json::from_str(&file_content).unwrap();

    cfg.name = name.to_owned();

    let json_content = serde_json::to_string_pretty(&cfg).unwrap();

    let mut file = File::create(CFG_FILE).unwrap();
    file.write_all(json_content.as_bytes()).unwrap();

    // unlock
    file.unlock().unwrap();

    Ok(())
}

pub fn get_startup() -> Vec<String> {
    get_cfg().startup
}

pub fn get_polling() -> Vec<String> {
    get_cfg().polling
}

pub fn get_aliases() -> Vec<Alias> {
    get_cfg().aliases
}

pub fn add_alias(alias: Alias) -> Result<(), String> {
    // lock
    let file = File::open(CFG_FILE).unwrap();
    file.lock_exclusive().unwrap();

    let file_content = fs::read_to_string(CFG_FILE).unwrap();
    let mut cfg: Cfg = serde_json::from_str(&file_content).unwrap();

    cfg.aliases.push(alias);

    let json_content = serde_json::to_string_pretty(&cfg).unwrap();

    let mut file = File::create(CFG_FILE).unwrap();
    file.write_all(json_content.as_bytes()).unwrap();

    // unlock
    file.unlock().unwrap();

    Ok(())
}
