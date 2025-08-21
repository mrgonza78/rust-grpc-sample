import { useState } from 'react';
import { ChatServiceClient } from './grpc-web/ChatServiceClientPb';
import { Message } from './grpc-web/chat_pb';

const client = new ChatServiceClient(process.env.REACT_APP_GRPC_SERVER || 'http://0.0.0.0:8080');

function SendMessage() {
  const [input, setInput] = useState('');
  const [response, setResponse] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const sendMessage = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setResponse(null);
    
    const msg = new Message();
    const now = new Date().toLocaleTimeString();
    msg.setContent(`[${now}] ${input}`);
    try {
      const res = await client.sendMessage(msg, {});
      setResponse(`Messages processed: ${res.getMessagesProcessed()}`);
    } catch (err: any) {
      setResponse(`Error: ${err.message || err}`);
    }

    setLoading(false);
    setInput('')
  };

  return (
    <div className="SendMessage">
        <h2>SendMessage</h2>
        <form onSubmit={sendMessage}>
          <textarea
            value={input}
            onChange={e => setInput(e.target.value)}
            disabled={loading}
            placeholder="Type a message"
            rows={4}
          />
          <button type="submit" disabled={loading || !input}>
            {loading ? 'Sending...' : 'Send'}
          </button>
        </form>
        {response && <p>{response}</p>}
    </div>
  );
}

export default SendMessage;
