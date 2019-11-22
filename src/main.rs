use actix_web::{web, App, HttpServer, Responder, HttpResponse, http};
use actix_web::get;
use std::fs::File;
use std::io::Read;

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/{id}/{name}/index.html")]
fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

#[get("/post/{name}.html")]
fn post(info: web::Path<(String)>) -> impl Responder {
    let filename = format!("{}.md", info.as_ref());
    if let Ok(mut file) = File::open(filename) {
        let mut file_content = String::new();
        if let Err(_) = file.read_to_string(&mut file_content) {
            return HttpResponse::new(http::StatusCode::NOT_FOUND);
        }
        HttpResponse::Ok().body(file_content)
    } else {
        return HttpResponse::new(http::StatusCode::NOT_FOUND);
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .service(advanced_index)
            .service(index)
            .service(post))
        .bind("127.0.0.1:8080")?
        .run()
}