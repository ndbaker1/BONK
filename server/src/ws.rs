use crate::{
    game_engine::{game_engine::handle_event, types::SafeGameStates},
    session_manager::{
        types::{Client, SafeClients, SafeSessions},
        utilities::{get_client_session_id, handle_client_connect, handle_client_disconnect},
    },
};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

/// The Initial Setup for a WebSocket Connection
pub async fn client_connection(
    ws: WebSocket,
    id: String,
    clients: SafeClients,
    sessions: SafeSessions,
    game_states: SafeGameStates,
) {
    //======================================================
    // Splits the WebSocket into a Sink + Stream:
    // Sink - Pools the messages to get send to the client
    // Stream - receiver of messages from the client
    //======================================================
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    //======================================================
    // Gets an Unbounced Channel that can transport messages
    // between asynchronous tasks:
    // Sender - front end of the channel
    // Receiver - recieves the sender messages
    //======================================================
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    //======================================================
    // Spawn a thread to forward messages
    // from our channel into our WebSocket Sink
    // between asynchronous tasks using the same Client object
    //======================================================
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("[error] failed to send websocket msg: {}", e);
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
            id: id.clone(),
            sender: Some(client_sender),
            session_id: get_client_session_id(&id, &sessions).await,
        },
    );

    if let Some(client) = clients.read().await.get(&id) {
        handle_client_connect(&client, &sessions).await;
    }
    //======================================================
    // Synchronously wait for messages from the
    // Client Receiver Stream until an error occurs
    //======================================================
    while let Some(result) = client_ws_rcv.next().await {
        // Check that there was no error actually obtaining the Message
        match result {
            Ok(msg) => {
                handle_client_msg(&id, msg, &clients, &sessions, &game_states).await;
            }
            Err(e) => {
                eprintln!(
                    "[error] failed to recieve websocket message for id: {} :: error: {}",
                    id.clone(),
                    e,
                );
            }
        }
    }
    //======================================================
    // Remove the Client from the Map
    // when they are finished using the socket (or error)
    //======================================================
    if let Some(client) = clients.write().await.remove(&id) {
        handle_client_disconnect(&client, &sessions, &game_states).await;
    }
}

/// Handle messages from an open receiving websocket
async fn handle_client_msg(
    id: &str,
    msg: Message,
    clients: &SafeClients,
    sessions: &SafeSessions,
    game_states: &SafeGameStates,
) {
    //======================================================
    // Ensure the Message Parses to String
    //======================================================
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("[warning] websocket message: '{:?}' was not handled", msg);
            return;
        }
    };

    match message {
        //======================================================
        // ignore pings
        //======================================================
        "ping" | "ping\n" => {
            println!("[event] ignoring ping...");
        }
        //======================================================
        // Game Session Related Events
        //======================================================
        _ => {
            handle_event(id, message, clients, sessions, game_states).await;
        }
    }
}
