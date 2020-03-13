use crate::list::List;
use crate::post::Post;
use crate::serverstate::ServerState;
use mdbook::utils::render_markdown;
use serde::Serialize;

use crate::error::*;

#[derive(Serialize, Debug, Clone)]
pub struct BlogData<'a> {
    ctxt: &'a str,
    software_version: &'static str,
    software_name: &'static str,
}
impl<'a> BlogData<'a> {
    pub fn new(ctxt: &'a str) -> BlogData<'a> {
        let software_version: &'static str = env!("CARGO_PKG_VERSION");
        let software_name: &'static str = env!("CARGO_PKG_NAME");
        BlogData {
            ctxt,
            software_version,
            software_name,
        }
    }
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
            blog: BlogData::new(&state.config.context),
            name: &post.name,
            title: &post.title,
            author: &post.author,
            body: state
                .md_cache
                .get_or_insert(&post.name, || render_markdown(&post.body, true)),
            date: post.date.to_string(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ListData<'a> {
    pub blog: BlogData<'a>,
    pub name: &'a str,
    pub title: &'a str,
    pub posts: Vec<PostData<'a>>,
}
impl<'a> ListData<'a> {
    pub fn from_list(list: &'a List, state: &'a ServerState) -> ListData<'a> {
        ListData {
            blog: BlogData::new(&state.config.context),
            name: &list.name,
            title: &list.title,
            posts: list
                .posts
                .iter()
                .map(|post| PostData::from_post(post, state))
                .collect(),
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

pub fn render_list_file(state: &ServerState, name: &str) -> BlogResult<String> {
    render_list(state, &List::from_name(state, name)?)
}

pub fn render_list(state: &ServerState, list: &List) -> BlogResult<String> {
    let handlebars = &*state.reg.read().unwrap();
    Ok(handlebars.render("list", &ListData::from_list(list, state))?)
}
