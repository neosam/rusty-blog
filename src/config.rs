use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::RwLock;

fn default_hostname() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u32 {
    8080
}

fn default_doc_path() -> String {
    "./".to_string()
}

fn default_context() -> String {
    String::new()
}

fn default_caching() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_hostname")]
    hostname: String,

    #[serde(default = "default_port")]
    port: u32,

    #[serde(default = "default_doc_path")]
    doc_path: String,

    #[serde(default = "default_context")]
    context: String,

    #[serde(default = "default_caching")]
    caching: bool,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            hostname: "127.0.0.1".to_string(),
            port: 8080,
            doc_path: "./".to_string(),
            context: "".to_string(),
            caching: true,
        }
    }
}

lazy_static! {
    pub static ref BLOG_CONFIG: RwLock<Config> =
        RwLock::new(if let Ok(file) = File::open("config.yml") {
            serde_yaml::from_reader(&file).unwrap_or_default()
        } else {
            Config::default()
        });
}
pub fn get_hostname() -> String {
    BLOG_CONFIG.read().unwrap().hostname.clone()
}
pub fn get_port() -> u32 {
    BLOG_CONFIG.read().unwrap().port
}
pub fn get_doc_path() -> String {
    BLOG_CONFIG.read().unwrap().doc_path.clone()
}
pub fn get_context() -> String {
    BLOG_CONFIG.read().unwrap().context.clone()
}
pub fn get_caching() -> bool {
    BLOG_CONFIG.read().unwrap().caching
}
