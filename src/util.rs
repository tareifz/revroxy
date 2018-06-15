use std::{
    path::PathBuf,
    env,
    io::{
        prelude::*,
        Error,
    },
    fs::{
        File,
    },
};

use super::{
    serde,
    serde_json,
};

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub host: String,
    pub ip: String,
    pub port: String
}

#[derive(Serialize, Deserialize)]
struct Revroxy {
    hosts: Vec<Host>,
}

pub fn revroxy_config() -> Result<String, Error> {
    let mut config_path = env::home_dir().expect("Could not find your home path!");
    config_path.push(".revroxy");

    let mut config_file = File::open(config_path)? ;
    let mut config_buffer = String::new();

    config_file.read_to_string(&mut config_buffer)? ;
    Ok(config_buffer)
}

pub fn config_vec() -> Vec<Host> {
    match revroxy_config() {
        Ok(config) => {
            let data: Revroxy = serde_json::from_str(config.as_str()).unwrap() ;
            data.hosts
        },
        Err(_) => {
            Vec::<Host>::new()
        }
    }

}
