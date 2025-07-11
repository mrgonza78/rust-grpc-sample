mod proto;

use proto::chat_package::Message;
use proto::chat_package::chat_service_client::ChatServiceClient;
use std::error::Error;
use tonic::Request;

const server_addr: &str = "http://[::1]:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pid = std::process::id();
    // Connect to the gRPC server
    let mut chat_service = ChatServiceClient::connect(server_addr).await?;

    // Build your request (replace fields as needed)
    let request = Request::new(Message {
        content: format!("Hi from PID {:?}. This is a test", pid).to_string(),
    });

    println!("Sending: {:?}", request);
    let response = chat_service.send_message(request).await?;
    println!("Resonse: {:?}", response);

    Ok(())
}
