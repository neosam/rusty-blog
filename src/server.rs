use actix_web::{App, HttpServer};
use log::{info, debug};
use std::sync::Arc;

use crate::config::*;
use crate::servicemappings::*;
use crate::template::init_templates;
use crate::error::*;
use crate::markdowncache::MarkdownCache;
use crate::serverstate::ServerState;

pub async fn run() -> BlogResult<()> {
    let hostname = get_hostname();
    let port = get_port();
    let md_cache = Arc::new(MarkdownCache::new());
    info!("Starting up, listening on {}:{}", hostname, port);
    HttpServer::new(move || {
        debug!("HttpServer new");
        let templates = init_templates().unwrap();
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