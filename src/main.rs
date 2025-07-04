mod proto;
mod service;
// mod unary;
// mod streaming;

use std::error::Error;
use tonic::transport::Server;
use proto::chat_package::chat_service_server::ChatServiceServer;
use service::MyChatService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("Server listening on {addr}");

    Server::builder()
        .add_service(ChatServiceServer::new(MyChatService::new()))
        .serve(addr)
        .await?;

    Ok(())
}
