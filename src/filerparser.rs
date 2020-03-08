//! Contains utilities for parsing markdown and generate html output.

use log::{debug};
use serde::{Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

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

/// Open the file on the given path and return its content
pub fn read_file_to_string(path: &str) -> BlogResult<String> {
    debug!("Opening file '{}'", path);
    let mut file_content = String::new();
    File::open(&path)?.read_to_string(&mut file_content)?;
    debug!("File '{}' reding done", path);
    Ok(file_content)
}
