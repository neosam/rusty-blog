//! Utilities to cache the markdown files

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::fs::File;
use log::debug;

/// Utility to cache the markdown files
pub struct MarkdownCache {
    /// Path to the directory where the converted HTML code lies
    work_dir: String,

    /// Path to the markdown directory
    md_dir: String,
}

impl MarkdownCache {
    /// Generate a new markdown cache
    pub fn new(path: impl ToString, md_dir: impl ToString) -> Self {
        let path = path.to_string();
        let md_dir = md_dir.to_string();
        create_dir_all(Path::new(&path)).unwrap();
        MarkdownCache {
            work_dir: path,
            md_dir,
        }
    }

    /// Generate path the the cached html file
    fn gen_path(&self, name: &str) -> PathBuf {
        let path = Path::new(&self.work_dir);
        debug!("Path: {}", &path.to_str().unwrap());
        let name = name.to_string();
        debug!("Name: {}", name);
        let file = path.join(&name).with_extension("html");
        debug!("Generate cache file: {}", file.to_str().unwrap());
        file
    }

    /// Generate path to the markdown file
    pub fn gen_md_path(&self, name: &str) -> PathBuf {
        let path = Path::new(&self.md_dir);
        let name = name.to_string();
        let file = path.join(&name).with_extension("md");
        debug!("Generate md path: {}", file.to_str().unwrap());
        file
    }

    /// Force writing a cache file
    pub fn set(&self, name: impl ToString, value: impl ToString) -> String {
        let name = name.to_string();
        let value = value.to_string();
        let file = self.gen_path(&name);
        File::create(file).unwrap().write_all(value.as_bytes()).unwrap();
        value
    }

    /// Read a cache file
    pub fn get(&self, name: &str) -> Option<String> {
        let file = self.gen_path(name);
        if file.exists() {
            let mut result = String::new();
            File::open(file).unwrap().read_to_string(&mut result).unwrap();
            Some(result)
        } else {
            None
        }
    }

    /// Get a cached content if it is found and not outdated or generate it
    /// 
    /// If the cached file doesn't exist or if its older than the markdown file,
    /// it the load_fn callback is executed.  If there is a cache file, read
    /// and return the value.
    pub fn get_or_insert(&self, name: &str, load_fn: impl FnOnce() -> String) -> String {
        let file = self.gen_path(name);
        let md_file = self.gen_md_path(name);
        if file.exists() 
                && file.metadata().unwrap().modified().unwrap() > md_file.metadata().unwrap().modified().unwrap() {
            self.get(name).unwrap()
        } else {
            self.set(name, load_fn())
        }
    }
}
