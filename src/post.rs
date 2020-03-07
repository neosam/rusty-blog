use chrono::DateTime;
use chrono::FixedOffset;
use chrono::TimeZone;
use serde::Serialize;
use log::debug;

use crate::error::*;
use crate::filerparser::{read_file_to_string, parse_header};
use crate::serverstate::ServerState;

#[derive(Clone, Debug, Serialize)]
pub struct Post {
    pub name: String,
    pub title: String,
    pub body: String,
    pub author: String,
    pub date: DateTime<FixedOffset>,
}

impl Post {
    pub fn from_name(state: &ServerState, name: &str) -> BlogResult<Post> {
        let md_path = state.md_cache.gen_md_path(name);
        Self::from_file(md_path.to_str()
            .ok_or(ParseError::new("Couldn't generate markdown path"))?, name)
    }

    pub fn from_file(path: &str, name: impl ToString) -> BlogResult<Post> {
        debug!("Reading post {}", path);
        let res = Post::from_str(&read_file_to_string(path)?, name);
        debug!("Reading post {} done", path);
        res
    }

    pub fn from_str(data: &str, name: impl ToString) -> BlogResult<Post> {
        let parsed_document = parse_header(data)?;
        let title = parsed_document.header.get("title")
                .ok_or(ParseError::new("Missing title in post"))?.clone();
        let author = parsed_document.header.get("author")
                .ok_or(ParseError::new("Missing author in post"))?.clone();
        let date_str = parsed_document.header.get("date")
                .ok_or(ParseError::new("Missing date in post"))?;
        let body = parsed_document.body;
        let name = name.to_string();
        
        debug!("Blog date: {:?}", date_str.trim());
        let date = DateTime::parse_from_rfc3339(&date_str.trim())
            .unwrap_or(FixedOffset::east(0).ymd(1970, 1, 1).and_hms(0, 0, 0));
            
        Ok(Post {
            name, title, author, date, body,
        })
    }
}