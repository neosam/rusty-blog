use actix_web::get;
use actix_web::{http, web, HttpResponse, Responder};
use log::{error, debug};
use handlebars::Handlebars;

use crate::error::*;
use crate::config::*;
use crate::filerparser::*;

fn respond(content: BlogResult<impl ToString>) -> impl Responder {
    match content {
        Ok(result) => HttpResponse::Ok().body(result.to_string()),
        Err(err) => {
            error!("Response error: {}", err);
            HttpResponse::new(http::StatusCode::NOT_FOUND)
        }
    }
}

#[get("/")]
pub async fn index(reg: web::Data<Handlebars>) -> impl Responder {
    respond(get_list(&reg, format!("{}/lists/main.txt", get_doc_path())))
}

#[get("/{id}/{name}/index.html")]
pub async fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

#[get("/static/{name}")]
pub async fn static_files(info: web::Path<String>) -> impl Responder {
    let filename = format!("{}/static/{}", get_doc_path(), info.as_ref());
    respond(read_file_to_string(filename))
}

#[get("/post/{name}.html")]
pub async fn post_controller(info: web::Path<String>, reg: web::Data<Handlebars>) -> impl Responder {
    debug!("start post '{}'", info);
    let filename = format!("{}/posts/{}.md", get_doc_path(), *info);
    let result = respond(get_post(&reg, filename));
    debug!("finished post '{}'", info);
    result
}

#[get("/list/{name}.html")]
pub async fn list_controller(info: web::Path<String>, reg: web::Data<Handlebars>) -> impl Responder {
    let filename = format!("{}/lists/{}.txt", get_doc_path(), *info);
    respond(get_list(&reg, filename))
}
