mod proto;

use proto::chat_package::Message;
use proto::chat_package::chat_service_client::ChatServiceClient;
use std::error::Error;
use tonic::Request;
use tonic::transport::{Channel, ClientTlsConfig};

const SERVER_ADDR: &str = "https://grpc-sample-35975833932.southamerica-west1.run.app"; // Demo server

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pid = std::process::id();
    // Connect to the gRPC server
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    let tls = ClientTlsConfig::new()
        .with_native_roots()
        .assume_http2(true);    
    let channel = Channel::from_static(SERVER_ADDR).tls_config(tls)?.connect().await?;
    let mut chat_service = ChatServiceClient::new(channel);

    // Build your request (replace fields as needed)
    let request = Request::new(Message {
        content: format!("Hi from PID {:?}. This is a test", pid).to_string(),
    });

    println!("Sending: {:?}", request);
    let response = chat_service.send_message(request).await?;
    println!("Resonse: {:?}", response);

    Ok(())
}
