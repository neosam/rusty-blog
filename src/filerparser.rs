use log::{debug};
use serde::{Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;
use handlebars::Handlebars;
use crate::serverstate::ServerState;
use crate::template::setup_templates;

use crate::error::*;
use crate::config::*;

#[derive(Serialize)]
pub struct ParsedDocument {
    header: HashMap<String, String>,
    body: String,
}

#[derive(Serialize)]
pub struct ListContent<'a> {
    posts: &'a Vec<ParsedDocument>,
    context: &'a HashMap<String, String>,
}
impl<'a> ListContent<'a> {
    fn new(
        posts: &'a Vec<ParsedDocument>,
        context: &'a HashMap<String, String>,
    ) -> ListContent<'a> {
        ListContent { posts, context }
    }
}

pub fn parse_header(content: &str) -> BlogResult<ParsedDocument> {
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
                return Err(Box::new(ParseError(
                    "Key/Value is not properly defined".to_string(),
                )));
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

    Ok(ParsedDocument { header, body })
}

pub fn render_template(
    reg: &Handlebars,
    name: &str,
    text: &str,
    context: &HashMap<String, String>,
) -> BlogResult<String> {
    let mut inner_context = context.clone();
    inner_context.insert("main".to_string(), text.to_string());
    inner_context.insert("ctxt".to_string(), get_context());

    Ok(reg.render(name, &inner_context).unwrap())
}
pub fn render_list_template(
    reg: &Handlebars,
    name: &str,
    content: &Vec<ParsedDocument>,
    context: &HashMap<String, String>,
) -> BlogResult<String> {
    let mut inner_context = context.clone();
    inner_context.insert("ctxt".to_string(), get_context());

    Ok(reg.render(name, &ListContent::new(content, &inner_context))?)
}

pub fn read_file_to_string(path: &str) -> BlogResult<String> {
    debug!("Opening file '{}'", path);
    let mut file_content = String::new();
    File::open(&path)?.read_to_string(&mut file_content)?;
    debug!("File '{}' reding done", path);
    Ok(file_content)
}



pub fn get_post(state: &ServerState, filename: String) -> BlogResult<String> {
    if !get_caching() {
        setup_templates(state.reg.write().unwrap().deref_mut())?;
    }
    let file_content = read_file_to_string(&filename)?;
    let parsed_document = parse_header(&file_content)?;
    let html_content = state.md_cache.get_or_insert(&filename, || markdown::to_html(&parsed_document.body));
    let html = render_template(&state.reg.read().unwrap(), "post", &html_content, &parsed_document.header)?;
    Ok(html)
}

pub fn get_list(state: &ServerState, filename: String) -> BlogResult<String> {
    if !get_caching() {
        setup_templates(state.reg.write().unwrap().deref_mut())?;
    }
    let list_file_content = read_file_to_string(&filename)?;
    let parsed_list = parse_header(&list_file_content)?;
    let mut posts = Vec::new();
    for post in parsed_list.body.lines() {
        if post.trim() == "" {
            continue;
        }
        let post_filename = format!("{}/posts/{}.md", get_doc_path(), post.trim());
        let post_content = read_file_to_string(&post_filename.clone())?;

        let mut parsed_document = parse_header(&post_content)?;
        debug!("Convert markdown to html");
        let body_as_html = state.md_cache.get_or_insert(&post_filename, || markdown::to_html(&parsed_document.body));
        debug!("Conversion markdown to html done");
        parsed_document.body = body_as_html.to_string();
        parsed_document
            .header
            .insert("id".to_string(), post.trim().to_string());
        posts.push(parsed_document);
    }
    let html = render_list_template(&state.reg.read().unwrap(), "list", &posts, &parsed_list.header)?;
    Ok(html)
}