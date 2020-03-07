use serde::Serialize;
use crate::post::Post;
use crate::serverstate::ServerState;
use mdbook::utils::render_markdown;

use crate::error::*;

#[derive(Serialize, Debug, Clone)]
pub struct BlogData<'a> {
    ctxt: &'a str
}

#[derive(Serialize, Debug, Clone)]
pub struct PostData<'a> {
    pub blog: BlogData<'a>,
    pub name: &'a str,
    pub title: &'a str,
    pub body: String,
    pub date: String,
    pub author: &'a str,
}

impl<'a> PostData<'a> {
    pub fn from_post(post: &'a Post, state: &'a ServerState) -> PostData<'a> {
        PostData {
            blog: BlogData {
                ctxt: &state.config.context,
            },
            name: &post.name,
            title: &post.title,
            author: &post.author,
            body: state.md_cache.get_or_insert(&post.name, || render_markdown(&post.body, true)),
            date: post.date.to_string(),
        }
    }
}

pub fn render_post_file(state: &ServerState, name: &str) -> BlogResult<String> {
    render_post(state, &Post::from_name(state, name)?)
}

pub fn render_post(state: &ServerState, post: &Post) -> BlogResult<String> {
    let handlebars = &*state.reg.read().unwrap();
    Ok(handlebars.render("post", &PostData::from_post(post, state))?)
}