use axum::extract::ws::{Message,WebSocket, WebSocketUpgrade};
use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;
use axum::http::StatusCode;

use crate::utils::jwt::verify_token;


#[derive(Deserialize)]
pub struct AuthQuery{
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade, 
    Query(query): Query<AuthQuery>,
) -> impl IntoResponse {

    match verify_token(&query.token){
        Ok(claims) => {
            println!("משתמש {} עבר בהצלחה את בדיקת האבטחה ומנסה להתחבר.", claims.sub);

            ws.on_upgrade(move |socket| handle_socket(socket, claims.sub))

        }
        Err(e) => {
            println!("Login attempt rejected: {}", e);
            (StatusCode::UNAUTHORIZED, "Invalid Token").into_response()
        }
    }
}

pub async fn handle_socket(mut socket: WebSocket, user_id: String){
    println!("the user conect to Ws!  ID: {}",user_id);

    while let Some(msg_result) = socket.recv().await{

        match msg_result{
            Ok(msg) => {
                    if let Message::Text(text) = msg {
                        println!("get it :{}", text);
                    
                    let response = format!("The server confirms receipt{}",text);
                    
                    if socket.send(Message::Text(response)).await.is_err() {
                        println!("Error: We were unable to send a message back. The client must have disconnected.");
                        break;
                    }
                }
            }
            Err(e) => {
                println!("The connection was lost or there was an error : {}",e);
                break;
            }
        }
    }
        
    println!("The customer left the chat.");
}