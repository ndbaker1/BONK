use crate::{game_engine, Client, Clients, Session, Sessions};
use futures::{FutureExt, StreamExt};
use std::collections::HashSet;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

/// The Initial Setup for a WebSocket Connection
pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, sessions: Sessions) {
    //======================================================
    // Complicated WebSocket Portion
    //======================================================
    // Splits the WebSocket into a Sink + Stream:
    // Sink - Pools the messages to get send to the client
    // Stream - receiver of messages from the client
    //======================================================
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    //======================================================
    // Gets an Unbounced Channel that can transport messages:
    // Sender - front end of the channel
    // Receiver - recieves the sender messages
    //======================================================
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    //======================================================
    // Spawn a thread to forward messages
    // from our channel into our WebSocket Sink
    //======================================================
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    //======================================================
    // From now on we can use our client_sender.send(val: T)
    // to send messages to a given client websocket
    //======================================================

    //======================================================
    // Create a new Client and insert them into the Map
    //======================================================
    clients.write().await.insert(
        id.clone(),
        Client {
            user_id: id.clone(),
            sender: Some(client_sender),
        },
    );
    println!("{} connected", id);
    //======================================================
    // Synchronously wait for messages from the
    // Client Receiver Stream until an error occurs
    //======================================================
    while let Some(result) = client_ws_rcv.next().await {
        handle_client_msg(
            &id,
            // Check that there was no error actually obtaining the Message
            match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                    break;
                }
            },
            &clients,
            &sessions,
        )
        .await;
    }
    //======================================================
    // Remove the Client from the Map
    // when they are finished using the socket (or error)
    //======================================================
    clients.write().await.remove(&id);
    println!("{} disconnected", id);
}

async fn handle_client_msg(id: &str, msg: Message, clients: &Clients, sessions: &Sessions) {
    println!("received message from {}: {:?}", id, msg);
    //======================================================
    // Ensure the Message Parses to String
    //======================================================
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    //======================================================
    // ignore pings
    //======================================================
    match message {
        "ping" | "ping\n" => {
            println!("ignoring ping...");
            return;
        }
        "create_session" => {
            let mut locked_sessions = sessions.write().await;
            locked_sessions.insert(
                String::from("Test"),
                Session {
                    client_ids: HashSet::new(),
                    session_id: 3,
                    name: String::from("Test"),
                },
            );
            if let Some(sess) = locked_sessions.get_mut("Test") {
                sess.client_ids.insert(String::from(id));
            }
        }
        _ => {
            //======================================================
            // Primary Logic For Handling Messages Follows
            //======================================================
            for session in sessions.read().await.values() {
                if session.client_ids.contains(id) {
                    println!("Theres one");
                    game_engine::handle_event(id, message, clients, session).await;
                }
            }
        }
    }
    println!("finished handling of message from {}", id);
}
