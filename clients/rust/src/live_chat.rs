mod proto;

use proto::chat_package::Message;
use proto::chat_package::chat_service_client::ChatServiceClient;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
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
    let mut grp_service = ChatServiceClient::new(channel);

    let (tx, rx) = tokio::sync::mpsc::channel(128);

    tokio::spawn(async move {
        for i in 0..100 {
            sleep(Duration::from_secs(1)).await;
            let message = Message {
                content: format!("Hi from PID {:?}. This is test #{:?}", pid, i).to_string(),
            };
            println!("Streaming: {:?}", message);
            tx.send(message).await.unwrap();
        }
    });

    let mut in_stream = grp_service
        .live_chat(ReceiverStream::new(rx))
        .await?
        .into_inner();

    while let Some(result) = in_stream.next().await {
        println!("Got from stream: {:?}", result);
    }
    println!("Stream ended");

    Ok(())
}
