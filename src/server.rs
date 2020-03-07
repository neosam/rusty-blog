//! Return the servers

use actix_web::{App, HttpServer};
use log::{info, debug};
use std::sync::Arc;
use std::sync::RwLock;

use crate::config::*;
use crate::servicemappings::*;
use crate::template::init_templates;
use crate::error::*;
use crate::markdowncache::MarkdownCache;
use crate::serverstate::ServerState;

/// Run the server
pub async fn run() -> BlogResult<()> {
    let config = Arc::new(Config::read("config.yml"));
    let hostname = config.hostname.clone();
    let port = config.port;
    let doc_path = config.doc_path.clone();
    debug!("doc path: {}", &doc_path);
    let md_cache = Arc::new(MarkdownCache::new(
        format!("{}/work/md-cache/", &doc_path),
        format!("{}/posts/", &doc_path)));
    info!("Starting up, listening on {}:{}", hostname, port);
    HttpServer::new(move || {
        debug!("HttpServer new");
        let templates = RwLock::new(init_templates(&config).unwrap());
        let serverstate = ServerState {
            reg: templates,
            md_cache: md_cache.clone(),
            config: config.clone(),
        };
        App::new()
            .data(serverstate)
            .service(advanced_index)
            .service(index)
            .service(post_controller)
            .service(list_controller)
            .service(static_files)
    })
    .bind(format!("{}:{}", hostname, port))?
    .run()
    .await?;
    Ok(())
}