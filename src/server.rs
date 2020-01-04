use actix_web::{App, HttpServer};
use log::{info};

use crate::config::*;
use crate::servicemappings::*;
use crate::template::init_templates;
use crate::error::*;

pub async fn run() -> BlogResult<()> {
    let hostname = get_hostname();
    let port = get_port();
    info!("Starting up, listening on {}:{}", hostname, port);
    HttpServer::new(|| {
        let templates = init_templates().unwrap();
        App::new()
            .data(templates)
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