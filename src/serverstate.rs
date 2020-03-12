//! Internal server state

use crate::config;
use crate::markdowncache::MarkdownCache;
use handlebars::Handlebars;
use std::sync::Arc;
use std::sync::RwLock;

/// Holds the internal server state
pub struct ServerState {
    /// Access the templates
    pub reg: RwLock<Handlebars<'static>>,

    /// Holds the markdown cache
    pub md_cache: Arc<MarkdownCache>,

    pub config: Arc<config::Config>,
}
