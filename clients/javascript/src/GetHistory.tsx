import React, { useEffect, useState } from 'react';
import { ChatServiceClient } from './grpc-web/ChatServiceClientPb';
import { HistoryRequest, Message } from './grpc-web/chat_pb';

const client = new ChatServiceClient('http://localhost:8080');

const GetHistory: React.FC = () => {
  const [messages, setMessages] = useState<string[]>([]);
  const [startingAt, setStartingAt] = useState<number>(0); // New state for input
  const [stream, setStream] = useState<any>(null); // To manage stream for cleanup

  const startHistoryStream = () => {
    setMessages([]); // Clear previous messages
    if (stream) {
      stream.cancel();
    }
    const request = new HistoryRequest();
    if (startingAt) {
      request.setStartingAt(startingAt); // Set the starting_at field
    }
    const newStream = client.getHistory(request);
    setStream(newStream);

    newStream.on('data', (response: Message) => {
      setMessages((prev) => [...prev, response.getContent()]);
    });

    newStream.on('end', () => {
      console.log('Stream ended');
    });
  };

  const endHistoryStream = () => {
    if (stream) {
      stream.cancel();
    }
    setStream(null);
  }

  useEffect(() => {
    // Cleanup on unmount
    return () => {
      if (stream) {
        stream.cancel();
      }
    };
    // Only run on unmount or when stream changes
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stream]);

  return (
    <div className="GetHistory">
      <h2>GetHistory</h2>
      <input
        type="number"
        placeholder="starting_at"
        value={startingAt}
        disabled={!!stream}
        onChange={(e) => setStartingAt(Number(e.target.value))}
      />
      <button onClick={startHistoryStream} disabled={!!stream}>
        {stream ? 'Streaming...' : 'Send'}
      </button>
      <button onClick={endHistoryStream} disabled={!stream}>
        Cancel
      </button>
      <ul>
        {messages.map((msg, idx) => (
          <li key={idx}>{msg}</li>
        ))}
      </ul>
    </div>
  );
};

export default GetHistory; 