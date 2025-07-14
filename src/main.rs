mod proto;
mod service;

use proto::chat_package::chat_service_server::ChatServiceServer;
use service::MyChatService;
use std::error::Error;
use tonic::{service::LayerExt, transport::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080".parse().unwrap();
    println!("Server listening on {addr}");

    let chat_service = tower::ServiceBuilder::new()
        .layer(tower_http::cors::CorsLayer::permissive()) // Allowing CORS
        .layer(tonic_web::GrpcWebLayer::new()) // Using GrpcWeb layer for the react app
        .into_inner()
        .named_layer(ChatServiceServer::new(MyChatService::new()));

    Server::builder()        
        .accept_http1(true) // GrpcWeb is over http1 so we must enable it.
        .add_service(chat_service)
        .serve(addr)
        .await?;

    Ok(())
}
