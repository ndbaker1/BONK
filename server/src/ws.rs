use crate::{game_engine, Client, Clients, Sessions};
use futures::{FutureExt, StreamExt};
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

/// Handles messages from a receiving websocket
async fn handle_client_msg(id: &str, msg: Message, clients: &Clients, sessions: &Sessions) {
    println!("received message from {}: {:?}", id, msg);
    //======================================================
    // Ensure the Message Parses to String
    //======================================================
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    match message {
        //======================================================
        // ignore pings
        //======================================================
        "ping" | "ping\n" => {
            println!("ignoring ping...");
        }
        //======================================================
        // Game Session Related Events
        //======================================================
        _ => {
            game_engine::handle_event(id, message, clients, sessions).await;
        }
    }
}
