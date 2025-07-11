mod proto;

use proto::chat_package::HistoryRequest;
use proto::chat_package::chat_service_client::ChatServiceClient;
use std::error::Error;
use tokio_stream::StreamExt;
use tonic::Request;

const SERVER_ADDR: &str = "http://[::1]:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the gRPC server
    let mut grp_service = ChatServiceClient::connect(SERVER_ADDR).await?;

    // Build your request (replace fields as needed)
    let request = Request::new(HistoryRequest { starting_at: 4 });

    println!("Sending: {:?}", request);
    let in_stream = grp_service.get_history(request).await.unwrap().into_inner();

    // stream is infinite - take just 100 elements and then disconnect
    let mut in_stream = in_stream.take(100);
    while let Some(item) = in_stream.next().await {
        println!("Received: {:?}", item.unwrap());
    }
    println!("Stream closed");

    Ok(())
}
