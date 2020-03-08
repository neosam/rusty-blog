use serde::Serialize;

use crate::post::Post;
use crate::serverstate::ServerState;
use crate::error::*;
use crate::filerparser::read_file_to_string;
use crate::filerparser::parse_header;

#[derive(Clone, Debug, Serialize)]
pub struct List {
    pub name: String,
    pub title: String,
    pub posts: Vec<Post>,
}

impl List {
    pub fn from_name(state: &ServerState, name: &str) -> BlogResult<List> {
        let path = format!("{}/lists/{}.txt", &state.config.doc_path, name);
        Self::from_file(state, &path, name)
    }

    pub fn from_file(state: &ServerState, path: &str, name: impl ToString) -> BlogResult<List> {
        let res = List::from_str(state, &read_file_to_string(path)?, name);
        res
    }

    pub fn from_str(state: &ServerState, data: &str, name: impl ToString) -> BlogResult<List> {
        let parsed_document = parse_header(data)?;
        let name = name.to_string();
        let title = parsed_document.header.get("title")
                .ok_or(ParseError::new("Missing title in list"))?.clone();
        let posts: Vec<Post> = parsed_document.body.lines()
            .filter_map(|post_name| Post::from_name(state, post_name).ok()).collect();

        Ok(List {
            name, title, posts,
        })
    }
}
