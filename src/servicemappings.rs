//! Contains all the mappings for the server

use actix_web::get;
use actix_web::{http, web, HttpResponse, Responder};
use log::{error, debug};
use crate::serverstate::ServerState;

use crate::error::*;
use crate::config::*;
use crate::filerparser::*;

/// Return a valid response the client
fn respond(content: BlogResult<impl ToString>) -> impl Responder {
    match content {
        Ok(result) => HttpResponse::Ok().body(result.to_string()),
        Err(err) => {
            error!("Response error: {}", err);
            HttpResponse::new(http::StatusCode::NOT_FOUND)
        }
    }
}

/// Display the main list of the blog
#[get("/")]
pub async fn index(reg: web::Data<ServerState>) -> impl Responder {
    respond(get_list(&reg, format!("{}/lists/main.txt", get_doc_path())))
}

/// Just a test
#[get("/{id}/{name}/index.html")]
pub async fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

/// Return static files from the static directory
#[get("/static/{name}")]
pub async fn static_files(info: web::Path<String>) -> impl Responder {
    let filename = format!("{}/static/{}", get_doc_path(), info.as_ref());
    respond(read_file_to_string(&filename))
}

/// Respond a blog post
#[get("/post/{name}.html")]
pub async fn post_controller(info: web::Path<String>, reg: web::Data<ServerState>) -> impl Responder {
    debug!("start post '{}'", info);
    let filename = format!("{}/posts/{}.md", get_doc_path(), *info);
    let result = respond(get_post(&reg, filename, &*info));
    debug!("finished post '{}'", info);
    result
}

/// Respond a list
#[get("/list/{name}.html")]
pub async fn list_controller(info: web::Path<String>, reg: web::Data<ServerState>) -> impl Responder {
    let filename = format!("{}/lists/{}.txt", get_doc_path(), *info);
    respond(get_list(&reg, filename))
}
