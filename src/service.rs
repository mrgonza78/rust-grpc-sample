use std::pin::Pin;
use tokio_stream::{Stream, wrappers::ReceiverStream,StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::proto::chat_package::{chat_service_server::ChatService, Message,SendMessageResponse, HistoryRequest};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

const MAX_MESSAGES: usize = 100;

pub struct MyChatService {
    messages: Arc<Mutex<Vec<String>>>,
    participants: Arc<Mutex<Vec<mpsc::Sender<Result<Message, Status>>>>>,
}

impl MyChatService {
    pub fn new() -> Self {
        Self { 
            messages: Arc::new(Mutex::new(vec![])),
            participants: Arc::new(Mutex::new(vec![]))
        }
    }
}

async fn push_message(content: String, messages: Arc<Mutex<Vec<String>>>) {
    let mut messages = messages.lock().await;
    messages.push(content);
    let len = messages.len();
    if len > MAX_MESSAGES {
        messages.drain(0..(len - MAX_MESSAGES));
    }
}

async fn push_participant(participant: mpsc::Sender<Result<Message, Status>>, participants: Arc<Mutex<Vec<mpsc::Sender<Result<Message, Status>>>>>) {
    let mut participants = participants.lock().await;
    participants.push(participant);
}

async fn broadcast_message(content: &String, participants: Arc<Mutex<Vec<mpsc::Sender<Result<Message, Status>>>>>) {
    let mut participants = participants.lock().await;
    let mut still_working = Vec::new();
    for tx in participants.iter() {
        if send_message(content, tx).await.is_ok() {
            still_working.push(tx.clone());
        }
    }
    // Replace the old vector with only the working participants
    *participants = still_working;
}

async fn send_message(content: &String, tx: &mpsc::Sender<Result<Message, Status>>) -> Result<(),mpsc::error::SendError<Result<Message, Status>>>{
    let message = Message {
        content: content.clone(),
    };
    let result = tx.send(Ok(message)).await;
    match &result {
        Ok(_) =>
            println!("Sent message ({:?}) to channel ({:?})", content, tx),
        Err(err) => 
            println!("Error ({:?}) sending message ({:?}) to channel: {:?}", err, content, tx)
    }
    result
}

#[tonic::async_trait]
impl ChatService for MyChatService {
    /// Unary function
    async fn send_message(
        &self,
        request: Request<Message>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        println!("send_message request received: {:?}", request);

        let message = request.into_inner();        
        broadcast_message(&message.content, self.participants.clone()).await;
        push_message(message.content, self.messages.clone()).await;

        let reply = SendMessageResponse {
            messages_processed: 1
        };
        Ok(Response::new(reply))
    }

    /// Client streaming
    async fn send_bulk_messages(
        &self,
        request: Request<Streaming<Message>>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        println!("send_bulk_messages stream started: {:?}", request);
        let mut in_stream = request.into_inner();
        let mut processed = 0;
        while let Some(item) = in_stream.next().await {
            println!("send_bulk_messages received: {:?}", item);
            let message = item.unwrap();
            broadcast_message(&message.content, self.participants.clone()).await;
            push_message(message.content, self.messages.clone()).await;
            processed += 1;
        }
        println!("send_bulk_messages stream ended");
        let reply = SendMessageResponse {
            messages_processed: processed
        };
        Ok(Response::new(reply))
    }
    
    /// Server streaming
    type GetHistoryStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;
    async fn get_history(
        &self,
        request: Request<HistoryRequest>
    ) -> Result<
        Response<Self::GetHistoryStream>,Status> {
            println!("get_history request received: {:?}", request);

            let (tx, rx) = tokio::sync::mpsc::channel(128);
            let starting_at = request.into_inner().starting_at as usize;

            let messages = self.messages.lock().await;
            let len = messages.len();
            for message in messages.iter().skip(len - starting_at) {
                let _ = send_message(&message,&tx).await;
            }

            push_participant(tx, self.participants.clone()).await;

            Ok(Response::new(Box::pin(ReceiverStream::new(rx)) as Self::GetHistoryStream))
    }
        
    /// Bidirectional streaming
    type LiveChatStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;
    async fn live_chat(
        &self,
        request: Request<Streaming<Message>>,
    ) -> Result<Response<Self::LiveChatStream>, Status> {
        println!("live_chat stream started: {:?}", request);
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);
        let messages = self.messages.clone();
        let participants = self.participants.clone();
        push_participant(tx, participants.clone()).await;

        tokio::spawn(async move {
            while let Some(item) = in_stream.next().await {
                println!("live_chat received: {:?}", item);
                let message = item.unwrap();
                broadcast_message(&message.content, participants.clone()).await;
                push_message(message.content, messages.clone()).await;
            }
            println!("live_chat stream ended");
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx)) as Self::LiveChatStream))
    }
}
