extern crate serde;

use serde::Deserialize;

const CONF_FILE: &str = "config.json";

#[derive(Deserialize)]
pub struct ApiConfig {
	pub database_file: String
}

#[derive(Deserialize)]
pub struct Config {
	pub api: ApiConfig
}
