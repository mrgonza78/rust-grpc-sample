use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};

mod proto;

use proto::chat_package::Message;
use proto::chat_package::chat_service_client::ChatServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pid = std::process::id();
    // Connect to the gRPC server
    let mut grp_service = ChatServiceClient::connect("http://[::1]:50051").await?;

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
