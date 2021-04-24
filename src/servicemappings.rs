//! Contains all the mappings for the server

use actix_web::get;
use actix_web::{http, web, HttpResponse, Responder};
use log::{error, debug};
use crate::serverstate::ServerState;

use crate::error::*;
use crate::filerparser::*;
use crate::render::*;

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
pub async fn index(state: web::Data<ServerState>) -> impl Responder {
    respond(render_list_file(&state, "main"))
        .with_header("content-type", "text/html")
}

/// Just a test
#[get("/{id}/{name}/index.html")]
pub async fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0.0)
}

/// Return static files from the static directory
#[get("/static/{name}")]
pub async fn static_files(state: web::Data<ServerState>, info: web::Path<String>) -> impl Responder {
    let filename = format!("{}/static/{}", state.config.doc_path, info.as_ref());
    respond(read_file_to_string(&filename))
}

/// Respond a blog post
#[get("/post/{name}.html")]
pub async fn post_controller(name: web::Path<String>, state: web::Data<ServerState>) -> impl Responder {
    debug!("start post '{}'", name);
    let result = respond(render_post_file(&state, &name))
        .with_header("content-type", "text/html");
    debug!("finished post '{}'", name);
    result
}

/// Respond a list
#[get("/list/{name}.html")]
pub async fn list_controller(name: web::Path<String>, state: web::Data<ServerState>) -> impl Responder {
    respond(render_list_file(&state, &name))
        .with_header("content-type", "text/html")
}
