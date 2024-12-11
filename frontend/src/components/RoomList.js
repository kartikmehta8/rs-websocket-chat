import React from "react";

const RoomList = ({ onSelectRoom }) => {
  const rooms = ["General", "Sports", "Tech", "Music"];

  return (
    <div className="w-1/4 bg-gray-800 text-white p-4">
      <h2 className="text-xl mb-4">Rooms</h2>
      {rooms.map((room) => (
        <button
          key={room}
          onClick={() => onSelectRoom(room)}
          className="block w-full py-2 px-4 mb-2 bg-gray-600 hover:bg-gray-700 rounded"
        >
          {room}
        </button>
      ))}
    </div>
  );
};

export default RoomList;
