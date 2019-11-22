use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::get;

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/{id}/{name}/index.html")]
fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .service(advanced_index)
            .service(index))
        .bind("127.0.0.1:8080")?
        .run()
}