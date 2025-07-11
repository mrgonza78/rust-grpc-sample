mod proto;

use proto::chat_package::Message;
use proto::chat_package::chat_service_client::ChatServiceClient;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

const SERVER_ADDR: &str = "http://[::1]:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pid = std::process::id();
    // Connect to the gRPC server
    let mut grp_service = ChatServiceClient::connect(SERVER_ADDR).await?;

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

    let response = grp_service
        .send_bulk_messages(ReceiverStream::new(rx))
        .await?;

    println!("Response: {:?}", response);
    Ok(())
}
