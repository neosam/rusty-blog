mod error;
mod config;
mod markdowncache;
mod template;
mod filerparser;
mod servicemappings;
mod serverstate;
mod server;


use error::BlogResult;

#[actix_rt::main]
async fn main() -> BlogResult<()> {
    env_logger::init();
    server::run().await?;
    Ok(())
}
