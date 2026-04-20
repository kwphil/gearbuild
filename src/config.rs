use std::{ fs::File, io::Read, path::PathBuf };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GeneralConfig {
    _name: String,
    _version: String,
    _gear: String
}

#[derive(Deserialize)]
pub struct Config {
    _general: GeneralConfig,
}

pub fn open_config(src_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_path = src_path.to_path_buf();
    config_path.push("gear.toml");
    
    let mut config_file = File::open(&config_path)?;

    let mut content: String = Default::default(); 
    config_file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?;
    Ok(config)
}