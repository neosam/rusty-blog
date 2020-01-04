use handlebars::Handlebars;
use crate::markdowncache::MarkdownCache;
use std::sync::Arc;

pub struct ServerState {
    pub reg: Handlebars,
    pub md_cache: Arc<MarkdownCache>,
}
