use axum::{
    extract::{Path, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use crate::utils::ssh_manager;


pub async fn ws_terminal_handler(
    ws: WebSocketUpgrade,
    Path(port): Path<u16>
) -> impl IntoResponse {
    println!("[INFO] Incoming WebSocket connection requested for port: {}", port);

    ws.on_upgrade(move |socket| handle_socket(socket, port))
}

async fn handle_socket(
    socket: WebSocket,
    port: u16
) {
    println!("[INFO] WebSocket connection successfully upgraded.");

    let (mut ws_sender, mut ws_receiver) = socket.split();

    let (input_tx, input_rx) = mpsc::channel::<String>(100);
    let (output_tx, mut output_rx) = mpsc::channel::<String>(100);

    let ssh_port = port;
    tokio::task::spawn_blocking(move || {
        if let Err(e) = ssh_manager::connect_and_bridge(ssh_port, "kali-target", "1213", input_rx, output_tx) {
            eprintln!("[ERROR] SSH Bridge terminated: {}", e);
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Some(text) = output_rx.recv().await {
            if ws_sender.send(Message::Text(text)).await.is_err() {
                break; // אם הדפדפן נסגר, אנחנו עוצרים
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                if input_tx.send(text).await.is_err() {
                    break;
                }
            }
        }
    });

    tokio::select!{
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    println!("[INFO] WebSocket session disconnected and cleaned up.");
}