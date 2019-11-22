use actix_web::{web, App, HttpServer, Responder, HttpResponse, http};
use actix_web::get;
use std::fs::File;
use std::io::Read;
use tinytemplate::TinyTemplate;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    main: String
}

fn render_template(filename: &str, text: &str) -> std::io::Result<String> {
    let mut template_text = String::new();
    File::open(format!("templates/{}.html", filename))?
        .read_to_string(&mut template_text)?;
    
    let mut tt = TinyTemplate::new();
    tt.add_template("main", &template_text);
    
    Ok(tt.render("main", &Context {
        main: text.to_string(),
    }).unwrap())
}

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/{id}/{name}/index.html")]
fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

#[get("/static/{name}")]
fn static_files(info: web::Path<String>) -> impl Responder {
    let filename = format!("static/{}", info.as_ref());
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

#[get("/post/{name}.html")]
fn post(info: web::Path<(String)>) -> impl Responder {
    let filename = format!("posts/{}.md", info.as_ref());
    if let Ok(mut file) = File::open(filename) {
        let mut file_content = String::new();
        if let Err(_) = file.read_to_string(&mut file_content) {
            return HttpResponse::new(http::StatusCode::NOT_FOUND);
        }
        let html = render_template(
            "post",
            &markdown::to_html(&file_content)).unwrap_or("Template error".to_string());
        HttpResponse::Ok().body(html)
    } else {
        return HttpResponse::new(http::StatusCode::NOT_FOUND);
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .service(advanced_index)
            .service(index)
            .service(post)
            .service(static_files))
        .bind("127.0.0.1:8080")?
        .run()
}