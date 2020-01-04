mod error;
mod config;
mod template;
mod filerparser;
mod servicemappings;
mod server;

use error::BlogResult;

#[actix_rt::main]
async fn main() -> BlogResult<()> {
    env_logger::init();
    server::run().await?;
    Ok(())
}
