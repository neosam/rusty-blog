use actix_web::{web, App, HttpServer, Responder, HttpResponse, http};
use actix_web::get;
use std::fs::File;
use std::io::Read;
use tinytemplate::TinyTemplate;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::{info, debug, error};
use lazy_static::lazy_static;
use std::sync::RwLock;

#[derive(Serialize)]
struct ParsedDocument {
    header: HashMap<String, String>,
    body: String,
}

#[derive(Serialize)]
struct ListContent<'a> {
    posts: &'a Vec<ParsedDocument>,
    context: &'a HashMap<String, String>,
}
impl<'a> ListContent<'a> {
    fn new(posts: &'a Vec<ParsedDocument>, context: &'a HashMap<String, String>) -> ListContent<'a> {
        ListContent { posts, context }
    }
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

type BlogResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize)]
struct Config {
    hostname: String,
    port: u32,
    doc_path: String,
    context: String,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            hostname: "127.0.0.1".to_string(),
            port: 8080,
            doc_path: "./".to_string(),
            context: "".to_string(),
        }
    }
}

lazy_static! {
    static ref BLOG_CONFIG: RwLock<Config> = RwLock::new(
        if let Ok(file) = File::open("config.yml") {
            serde_yaml::from_reader(&file).unwrap_or_default()
        } else {
            Config::default()
        }
    );
}
fn get_hostname() -> String {
    BLOG_CONFIG.read().unwrap().hostname.clone()
}
fn get_port() -> u32 {
    BLOG_CONFIG.read().unwrap().port
}
fn get_doc_path() -> String {
    BLOG_CONFIG.read().unwrap().doc_path.clone()
}
fn get_context() -> String {
    BLOG_CONFIG.read().unwrap().context.clone()
}


fn parse_header(content: &str) -> BlogResult<ParsedDocument> {
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

fn render_template(filename: &str, text: &str, context: &HashMap<String, String>) -> BlogResult<String> {
    let template_text = read_file_to_string(format!("{}/templates/{}.html", get_doc_path(), filename))?;
    
    let mut tt = TinyTemplate::new();
    tt.add_template("main", &template_text)?;

    let mut inner_context = context.clone();
    inner_context.insert("main".to_string(), text.to_string());
    inner_context.insert("ctxt".to_string(), get_context());
    
    Ok(tt.render("main", &inner_context).unwrap())
}
fn render_list_template(filename: &str, content: &Vec<ParsedDocument>, context: &HashMap<String, String>) -> BlogResult<String> {
    let template_text = read_file_to_string(format!("{}/templates/{}.html", get_doc_path(), filename))?;
    
    let mut tt = TinyTemplate::new();
    tt.add_template("list", &template_text)?;

    let mut inner_context = context.clone();
    inner_context.insert("ctxt".to_string(), get_context());
    
    Ok(tt.render("list", &ListContent::new(content, &inner_context))?)
}

fn read_file_to_string(path: String) -> BlogResult<String> {
    debug!("Opening file '{}'", path);
    let mut file_content = String::new();
    File::open(path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

fn respond(content: BlogResult<impl ToString>) -> impl Responder {
    match content {
        Ok(result) => HttpResponse::Ok().body(result.to_string()),
        Err(err) => {
            error!("Response error: {}", err);
            HttpResponse::new(http::StatusCode::NOT_FOUND)
        }
    }
}

fn get_post(filename: String) -> BlogResult<String> {
    let file_content = read_file_to_string(filename)?;
    let parsed_document = parse_header(&file_content)?;
    let html_content = &markdown::to_html(&parsed_document.body);
    let html = render_template("post", html_content, &parsed_document.header)?;
    Ok(html)
}

fn get_list(filename: String) -> BlogResult<String> {
    let list_file_content = read_file_to_string(filename)?;
    let parsed_list = parse_header(&list_file_content)?;
    let mut posts = Vec::new();
    for post in parsed_list.body.lines() {
        if post.trim() == "" {
            continue;
        }
        let post_filename = format!("{}/posts/{}.md", get_doc_path(), post.trim());
        let post_content = read_file_to_string(post_filename)?;
        
        let mut parsed_document = parse_header(&post_content)?;
        let body_as_html = markdown::to_html(&parsed_document.body);
        parsed_document.body = body_as_html;
        parsed_document.header.insert("id".to_string(), post.trim().to_string());
        posts.push(parsed_document);
    }
    let html = render_list_template("list", &posts, &parsed_list.header)?;
    Ok(html)
}

#[get("/")]
fn index() -> impl Responder {
    respond(get_list(format!("{}/lists/main.txt", get_doc_path())))
}

#[get("/{id}/{name}/index.html")]
fn advanced_index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

#[get("/static/{name}")]
fn static_files(info: web::Path<String>) -> impl Responder {
    let filename = format!("{}/static/{}", get_doc_path(), info.as_ref());
    respond(read_file_to_string(filename))
}

#[get("/post/{name}.html")]
fn post_controller(info: web::Path<(String)>) -> impl Responder {
    let filename = format!("{}/posts/{}.md", get_doc_path(), *info);
    respond(get_post(filename))
}

#[get("/list/{name}.html")]
fn list_controller(info: web::Path<(String)>) -> impl Responder {
    let filename = format!("{}/lists/{}.txt", get_doc_path(), *info);
    respond(get_list(filename))
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let hostname = get_hostname();
    let port = get_port();
    info!("Starting up, listening on {}:{}", hostname, port);
    HttpServer::new(
        || App::new()
            .service(advanced_index)
            .service(index)
            .service(post_controller)
            .service(list_controller)
            .service(static_files))
        .bind(format!("{}:{}", hostname, port))?
        .run()
}

