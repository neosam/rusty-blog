use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::fs::File;
use log::debug;

pub struct MarkdownCache {
    work_dir: String,
    md_dir: String,
}

impl MarkdownCache {
    pub fn new(path: impl ToString, md_dir: impl ToString) -> Self {
        let path = path.to_string();
        let md_dir = md_dir.to_string();
        create_dir_all(Path::new(&path)).unwrap();
        MarkdownCache {
            work_dir: path,
            md_dir,
        }
    }

    fn gen_path(&self, name: &str) -> PathBuf {
        let path = Path::new(&self.work_dir);
        debug!("Path: {}", &path.to_str().unwrap());
        let name = name.to_string();
        debug!("Name: {}", name);
        let file = path.join(&name).with_extension("html");
        debug!("Generate cache file: {}", file.to_str().unwrap());
        file
    }

    fn gen_md_path(&self, name: &str) -> PathBuf {
        let path = Path::new(&self.md_dir);
        let name = name.to_string();
        let file = path.join(&name).with_extension("md");
        debug!("Generate md path: {}", file.to_str().unwrap());
        file
    }

    pub fn set(&self, name: impl ToString, value: impl ToString) -> String {
        let name = name.to_string();
        let value = value.to_string();
        let file = self.gen_path(&name);
        File::create(file).unwrap().write_all(value.as_bytes()).unwrap();
        value
    }

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
