mod proto;
mod service;

use proto::chat_package::chat_service_server::ChatServiceServer;
use service::MyChatService;
use std::error::Error;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080".parse().unwrap();
    println!("Server listening on {addr}");

    Server::builder()
        .add_service(ChatServiceServer::new(MyChatService::new()))
        .serve(addr)
        .await?;

    Ok(())
}
