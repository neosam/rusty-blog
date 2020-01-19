//! Access data from the configuration file.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::RwLock;

/// The default hostname is on local loopback.
fn default_hostname() -> String {
    "127.0.0.1".to_string()
}

/// The default port is 8080
fn default_port() -> u32 {
    8080
}

/// The default doc path is the current working directory.
fn default_doc_path() -> String {
    "./".to_string()
}

/// The default deployment context is the an empty string
fn default_context() -> String {
    String::new()
}


/// By default, caching is enabled
fn default_caching() -> bool {
    true
}

/// Contains the configuration for the application
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Hostname which is should be bound
    /// 
    /// Use 127.0.0.1 to bind it to the localhost only.  Then the blog will
    /// not be accessable from the outside.  This is good for editing content
    /// and not for hosting.
    /// 
    /// Use 0.0.0.0 to make the page accessable from the network / the internet.
    #[serde(default = "default_hostname")]
    hostname: String,

    /// The server will be bound to this port
    #[serde(default = "default_port")]
    port: u32,

    /// Path to the documents
    #[serde(default = "default_doc_path")]
    doc_path: String,

    /// The context should be the https path to the root of the blog.
    /// 
    /// This is used to build links in the blog.
    #[serde(default = "default_context")]
    context: String,

    /// If caching is enabled or not.  If enabled, it the theme will only
    /// be loaded on startup.  If not, the theme will be loaded on every page
    /// request which can be used to edit the theme.
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

/// Get the configured hostname
pub fn get_hostname() -> String {
    BLOG_CONFIG.read().unwrap().hostname.clone()
}
/// Get the configured port
pub fn get_port() -> u32 {
    BLOG_CONFIG.read().unwrap().port
}
/// Get the configured path to the blog files
pub fn get_doc_path() -> String {
    BLOG_CONFIG.read().unwrap().doc_path.clone()
}
/// Get the root URL of the blog
pub fn get_context() -> String {
    BLOG_CONFIG.read().unwrap().context.clone()
}
pub fn get_caching() -> bool {
    BLOG_CONFIG.read().unwrap().caching
}
