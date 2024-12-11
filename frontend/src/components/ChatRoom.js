import React, { useState, useEffect } from "react";

const ChatRoom = ({ room }) => {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const socketRef = React.useRef(null);

  useEffect(() => {
    socketRef.current = new WebSocket("ws://127.0.0.1:3030/chat");

    socketRef.current.onmessage = (event) => {
      setMessages((prev) => [...prev, event.data]);
    };

    return () => {
      socketRef.current.close();
    };
  }, [room]);

  const sendMessage = () => {
    if (socketRef.current && input) {
      socketRef.current.send(`${room}:${input}`);
      setInput("");
    }
  };

  return (
    <div className="flex flex-col flex-1 bg-gray-100 p-4">
      <h2 className="text-xl mb-4">Room: {room}</h2>
      <div className="flex-1 overflow-y-auto mb-4">
        {messages.map((msg, index) => (
          <div key={index} className="mb-2">
            {msg}
          </div>
        ))}
      </div>
      <div className="flex">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="flex-1 p-2 border rounded-l"
        />
        <button
          onClick={sendMessage}
          className="p-2 bg-blue-500 text-white rounded-r"
        >
          Send
        </button>
      </div>
    </div>
  );
};

export default ChatRoom;
