mod error;
mod config;
mod filerparser;
mod servicemappings;
mod server;

fn main() -> std::io::Result<()> {
    env_logger::init();
    server::run()
}
