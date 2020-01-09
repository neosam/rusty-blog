use actix_web::{App, HttpServer};
use log::{info, debug};
use std::sync::Arc;
use std::sync::RwLock;
use std::path::Path;

use crate::config::*;
use crate::servicemappings::*;
use crate::template::init_templates;
use crate::error::*;
use crate::markdowncache::MarkdownCache;
use crate::serverstate::ServerState;

pub async fn run() -> BlogResult<()> {
    let hostname = get_hostname();
    let port = get_port();
    let doc_path = get_doc_path();
    debug!("doc path: {}", &doc_path);
    let md_cache = Arc::new(MarkdownCache::new(
        format!("{}/work/md-cache/", &doc_path),
        format!("{}/posts/", &doc_path)));
    info!("Starting up, listening on {}:{}", hostname, port);
    HttpServer::new(move || {
        debug!("HttpServer new");
        let templates = RwLock::new(init_templates().unwrap());
        let serverstate = ServerState {
            reg: templates,
            md_cache: md_cache.clone(),
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