
pub mod server;

use server::start_dev_server;

#[tokio::main]
async fn main() {
    
    start_dev_server().await;
}