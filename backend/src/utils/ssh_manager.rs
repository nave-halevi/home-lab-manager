use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use tokio::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn connect_and_bridge(
    port: u16,
    username: &str,
    password: &str,
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), String> {
    println!("[INFO] Bridging SSH for port {}...", port);

    let tcp_target = format!("127.0.0.1:{}", port);
    let tcp = TcpStream::connect(&tcp_target).map_err(|e| e.to_string())?;
    tcp.set_nodelay(true).map_err(|e| e.to_string())?;

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(|e| e.to_string())?;
    sess.userauth_password(username, password).map_err(|e| e.to_string())?;

    let mut channel = sess.channel_session().map_err(|e| e.to_string())?;
    channel.request_pty("xterm", None, Some((80, 24, 0, 0))).map_err(|e| e.to_string())?;
    channel.shell().map_err(|e| e.to_string())?;
    
    sess.set_blocking(false);

    println!("[SUCCESS] Interactive shell started on port {}.", port);

    let mut buffer = [0; 1024];

    loop {
        match channel.read(&mut buffer) {
            Ok(0) => {
                println!("[INFO] SSH Channel closed by remote.");
                break;
            }
            Ok(n) => {
                let output = String::from_utf8_lossy(&buffer[..n]).to_string();
                if output_tx.blocking_send(output).is_err() {
                    break;
                }
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    eprintln!("[ERROR] SSH Read failed: {}", e);
                    break;
                }
            }
        }

        match input_rx.try_recv() {
            Ok(input) => {
                println!("[DEBUG INPUT TO SSH] {:?}", input); 
                
                let mut bytes = input.as_bytes();
                while !bytes.is_empty() {
                    match channel.write(bytes) {
                        Ok(0) => break,
                        Ok(n) => bytes = &bytes[n..],
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(1));
                            continue;
                        }
                        Err(_) => break,
                    }
                }
                let _ = channel.flush();
            }
            Err(mpsc::error::TryRecvError::Empty) => {
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                println!("[INFO] WebSocket disconnected by user.");
                break;
            }
        }

        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}