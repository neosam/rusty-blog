mod error;
mod config;
mod markdowncache;
mod template;
mod filerparser;
mod servicemappings;
mod serverstate;
mod server;
mod render;
mod post;
mod list;

use error::BlogResult;

#[actix_web::main]
async fn main() -> BlogResult<()> {
    env_logger::init();
    server::run().await?;
    Ok(())
}
