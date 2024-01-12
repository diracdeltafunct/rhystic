pub mod core;
pub mod server;
pub mod types;


use server::start_dev_server;

#[tokio::main]
async fn main() {
    
    start_dev_server().await;
}