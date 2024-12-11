use warp::{ws::Message, ws::WebSocket, Filter};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures::{StreamExt, SinkExt};
use tokio::sync::mpsc;

type Users = Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<Message>>>>>;

#[tokio::main]
async fn main() {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    let chat = warp::path("chat")
        .and(warp::ws())
        .and(warp::any().map(move || Arc::clone(&users)))
        .map(|ws: warp::ws::Ws, users| {
            println!("New connection attempt");
            ws.on_upgrade(move |socket| handle_connection(socket, users))
        });

    println!("Server started on ws://127.0.0.1:3030/chat");
    warp::serve(chat)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_connection(ws: WebSocket, users: Users) {
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let user_tx = tx.clone();
    tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Sending message: {:?}", message);
            if user_ws_tx.send(message).await.is_err() {
                println!("Failed to send message, closing connection");
                break;
            }
        }
    });

    tokio::task::spawn(async move {
        while let Some(result) = user_ws_rx.next().await {
            match result {
                Ok(msg) => {
                    println!("Received message: {:?}", msg);
                    if let Ok(text) = msg.to_str() {
                        let msg: Vec<&str> = text.splitn(2, ':').collect();
                        if msg.len() == 2 {
                            let room = msg[0].to_string();
                            let message = msg[1].to_string();
                            println!("Room: {}, Message: {}", room, message);

                            let mut users = users.lock().unwrap();

                            // Ensure `user_tx` is added only once to the room
                            let room_users = users.entry(room.clone()).or_insert_with(Vec::new);
                            if !room_users.iter().any(|existing_tx| existing_tx.same_channel(&user_tx)) {
                                room_users.push(user_tx.clone());
                            }

                            for tx in room_users.iter() {
                                if tx.send(Message::text(message.clone())).is_err() {
                                    println!("Failed to send message to a user");
                                }
                            }
                        } else {
                            println!("Invalid message format");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading websocket message: {}", e);
                    break;
                }
            }
        }
        println!("Connection closed");
    });
}
