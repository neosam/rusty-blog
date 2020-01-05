use handlebars::Handlebars;
use crate::markdowncache::MarkdownCache;
use std::sync::Arc;
use std::sync::RwLock;

pub struct ServerState {
    pub reg: RwLock<Handlebars>,
    pub md_cache: Arc<MarkdownCache>,
}
