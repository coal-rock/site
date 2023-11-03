use figment::providers::{Format, Serialized, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub template_dir: String,
    pub static_dir: String,
    pub db_path: String,
    pub hot_reload: bool,
    pub interface: Ipv4Addr,
    pub logging: bool,
    pub port: u16,
    pub user: String,
    pub pass: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            template_dir: "templates".to_string(),
            static_dir: "static".to_string(),
            hot_reload: false,
            db_path: "site.db".to_string(),
            interface: Ipv4Addr::from([127, 0, 0, 1]),
            port: 3000,
            user: "admin".to_string(),
            pass: "password".to_string(),
            logging: false,
        }
    }
}

impl Config {
    pub fn load(config_path: String) -> Result<Config, figment::Error> {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_path))
            .extract()
    }
}
