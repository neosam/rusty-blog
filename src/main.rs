use actix_web::{web, App, HttpServer, Responder, HttpResponse, http};
use actix_web::get;
use std::fs::File;
use std::io::Read;
use tinytemplate::TinyTemplate;
use serde::Serialize;
use std::collections::HashMap;

struct ParsedDocument {
    header: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
struct ParseError(String);
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}
impl std::error::Error for ParseError {

}

fn parse_header(content: &str) -> Result<ParsedDocument, Box<dyn std::error::Error>> {
    let mut header = HashMap::new();
    let mut body = String::new();

    let mut lines = content.lines();
    let mut line_opt = lines.next();
    if line_opt != Some("---") {
        let error_msg = if let Some(line) = line_opt {
            format!("Expected starting --- but got {}", line).to_string()
        } else {
            "File seems to be empty".to_string()
        };
        return Err(Box::new(ParseError(error_msg)));
    }
    line_opt = lines.next();
    while line_opt != Some("---") {
        if let Some(line) = line_opt {
            let mut splitted = line.split(":");
            let key = splitted.next();
            let value = splitted.next();
            if let (Some(key), Some(value)) = (key, value) {
                header.insert(key.to_string(), value.to_string());
            } else {
                return Err(Box::new(ParseError("Key/Value is not properly defined".to_string())));
            }
        } else {
            return Err(Box::new(ParseError("Header is never closed".to_string())));
        }        
        line_opt = lines.next();
    }
    for line in lines {
        body.push_str(line);
        body.push('\n');
    }

    Ok(ParsedDocument {
        header,
        body,
    })
}

#[derive(Serialize)]
struct Context {
    main: String
}

fn render_template(filename: &str, text: &str, context: &HashMap<String, String>) -> std::io::Result<String> {
    let mut template_text = String::new();
    File::open(format!("templates/{}.html", filename))?
        .read_to_string(&mut template_text)?;
    
    let mut tt = TinyTemplate::new();
    tt.add_template("main", &template_text);

    let mut inner_context = context.clone();
    inner_context.insert("main".to_string(), text.to_string());
    
    Ok(tt.render("main", &inner_context).unwrap())
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
        let parsed_document = parse_header(&file_content).unwrap();
        let html = render_template(
            "post",
            &markdown::to_html(&parsed_document.body),
            &parsed_document.header).unwrap_or("Template error".to_string());
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