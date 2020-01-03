use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::RwLock;

#[derive(Serialize, Deserialize)]
pub struct Config {
    hostname: String,
    port: u32,
    doc_path: String,
    context: String,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            hostname: "127.0.0.1".to_string(),
            port: 8080,
            doc_path: "./".to_string(),
            context: "".to_string(),
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
