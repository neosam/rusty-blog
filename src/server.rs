use actix_web::{App, HttpServer};
use log::{info};

use crate::config::*;
use crate::servicemappings::*;

pub fn run() -> std::io::Result<()> {
    let hostname = get_hostname();
    let port = get_port();
    info!("Starting up, listening on {}:{}", hostname, port);
    HttpServer::new(|| {
        App::new()
            .service(advanced_index)
            .service(index)
            .service(post_controller)
            .service(list_controller)
            .service(static_files)
    })
    .bind(format!("{}:{}", hostname, port))?
    .run()
}