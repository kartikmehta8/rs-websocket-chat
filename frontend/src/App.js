import React, { useState } from "react";
import RoomList from "./components/RoomList";
import ChatRoom from "./components/ChatRoom";

const App = () => {
  const [currentRoom, setCurrentRoom] = useState(null);

  return (
    <div className="flex h-screen">
      <RoomList onSelectRoom={setCurrentRoom} />
      {currentRoom && <ChatRoom room={currentRoom} />}
    </div>
  );
};

export default App;