use std::fs::{self};
use serde::Deserialize;

// CODE

const CONFIG_FILE_PATH:&str = "./rustbase.toml";

#[derive(Deserialize)]
pub struct MainConfig {
   pub is_dev:bool,
   pub is_debug:bool,
   pub is_debug_detailed:bool,

   pub database:DataBaseConfig
}

#[derive(Deserialize)]
struct DataBaseConfig {
    pub is_debug:bool,
    pub host:String,
    pub username:String,
    pub password:String,
    pub name:String
}

pub struct ServerConfig {}

impl ServerConfig {
    pub fn _load() -> MainConfig {
        let _config_data = fs::read_to_string(CONFIG_FILE_PATH).unwrap();
        let _config:MainConfig = toml::from_str(&_config_data).unwrap();
        _config
    }
}