//! Internal server state

use handlebars::Handlebars;
use crate::markdowncache::MarkdownCache;
use std::sync::Arc;
use std::sync::RwLock;
use crate::config;

/// Holds the internal server state
pub struct ServerState {
    /// Access the templates
    pub reg: RwLock<Handlebars<'static>>,

    /// Holds the markdown cache
    pub md_cache: Arc<MarkdownCache>,

    pub config: Arc<config::Config>,
}
