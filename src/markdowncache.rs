use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log::debug;
use crate::config::get_caching;

pub struct MarkdownCache {
    cache: Arc<RwLock<HashMap<String, Arc<String>>>>
}

impl MarkdownCache {
    pub fn new() -> Self {
        MarkdownCache {
            cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn set(&self, name: impl ToString, value: impl ToString) -> Arc<String> {
        let value = Arc::new(value.to_string());
        self.cache.write().unwrap().insert(name.to_string(), value.clone());
        debug!("markdowncache: '{}' inserted", name.to_string());
        value
    }

    pub fn get(&self, name: &str) -> Option<Arc<String>> {
        debug!("markdowncache: cachesize is {}", self.cache.read().unwrap().len());
        self.cache.read().unwrap().get(name).map(|x| x.clone())
    }

    pub fn get_or_insert(&self, name: &str, load_fn: impl FnOnce() -> String) -> Arc<String> {
        if !get_caching() {
            return Arc::new(load_fn())
        }
        if let Some(content) = self.get(name) {
            debug!("markdowncache: '{}' was in cache", name);
            content
        } else {
            debug!("markdowncache: '{}' was not in cache - loading, caching and returning", name);
            self.set(name, load_fn())
        }
    }
}

#[test]
fn test_markdown() {
    let mut x = 0;
    let string_generator = || {
        x += 1;
        "example text".to_string()
    };

    let mc = MarkdownCache::new();
    let text = mc.get_or_insert("test", string_generator);

    let string_generator = || {
        x += 1;
        "example text".to_string()
    };
    let text2 = mc.get_or_insert("test", string_generator);
    assert_eq!(1, x);
    assert_eq!("example text", &*text);
    assert_eq!("example text", &*text2)
}
