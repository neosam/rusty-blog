//! Contains utilities for parsing markdown and generate html output.

use log::{debug};
use serde::{Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;
use crate::serverstate::ServerState;
use crate::template::setup_templates;
use mdbook::utils::render_markdown;

use crate::error::*;

/// Contains the content of a blog entry
/// 
/// A blog entry has key/value pairs as header and a String as body.
#[derive(Serialize)]
pub struct ParsedDocument {
    /// Key/value pairs for the header
    pub header: HashMap<String, String>,

    /// The content of the entry
    pub body: String,
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

/// Parse a blog entry
/// 
/// The blog entry should start with a line which only contains ---.  The next
/// lines are key/value pairs separated by a colon.  The header is terminated
/// by another thre dashes.  The rest until EOF is the body.
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
            let value = splitted
                    .collect::<Vec<&str>>()
                    .join(":");
            if let (Some(key), value) = (key, value) {
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

/// Renders a list page
/// 
/// It requires the Handlebars template content, the name of the template,
/// the ParsedDocuments to display in the list and key/value pairs.
pub fn render_list_template(
    state: &ServerState,
    name: &str,
    content: &Vec<ParsedDocument>,
    context: &HashMap<String, String>,
) -> BlogResult<String> {
    let mut inner_context = context.clone();
    inner_context.insert("ctxt".to_string(), state.config.context.clone());

    Ok(state.reg.read().unwrap().render(name, &ListContent::new(content, &inner_context))?)
}

/// Open the file on the given path and return its content
pub fn read_file_to_string(path: &str) -> BlogResult<String> {
    debug!("Opening file '{}'", path);
    let mut file_content = String::new();
    File::open(&path)?.read_to_string(&mut file_content)?;
    debug!("File '{}' reding done", path);
    Ok(file_content)
}

/// Return the full html code for a list
/// 
/// It requires the ServerState and a filename where the list file lies.
pub fn get_list(state: &ServerState, filename: String) -> BlogResult<String> {
    if !state.config.caching {
        setup_templates(state.reg.write().unwrap().deref_mut(), &state.config)?;
    }
    let list_file_content = read_file_to_string(&filename)?;
    let parsed_list = parse_header(&list_file_content)?;
    let mut posts = Vec::new();
    for post in parsed_list.body.lines() {
        if post.trim() == "" {
            continue;
        }
        let post_filename = format!("{}/posts/{}.md", state.config.doc_path, post.trim());
        let post_content = read_file_to_string(&post_filename.clone())?;

        let mut parsed_document = parse_header(&post_content)?;
        debug!("Convert markdown to html");
        let body_as_html = state.md_cache.get_or_insert(&post.trim(), || render_markdown(&parsed_document.body, true));
        debug!("Conversion markdown to html done");
        parsed_document.body = body_as_html.to_string();
        parsed_document
            .header
            .insert("id".to_string(), post.trim().to_string());
        posts.push(parsed_document);
    }
    let html = render_list_template(state, "list", &posts, &parsed_list.header)?;
    Ok(html)
}