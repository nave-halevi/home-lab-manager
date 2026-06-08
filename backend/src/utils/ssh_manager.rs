use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use tokio::sync::mpsc;

/*
SSH Bridge Overview:

port: u16
    פורט לוקאלי (NAT) שמפנה ל-SSH של ה-VM

username: &str
    שם משתמש להתחברות ל-SSH (למשל kali-target)

password: &str
    סיסמת SSH של המכונה

input_rx: mpsc::Receiver<String>
    מקבל קלט מהדפדפן (WebSocket → Rust)
    כל הודעה = פקודה שהמשתמש הקליד

output_tx: mpsc::Sender<String>
    שולח פלט חזרה לדפדפן (Rust → WebSocket)

tcp_target:
    כתובת היעד של ה-SSH (127.0.0.1:port)

tcp:
    חיבור TCP בסיסי ל-SSH port

sess:
    Session של ssh2 (האובייקט שמנהל SSH connection)

channel:
    ערוץ SSH פעיל (shell session)

channel_read:
    צד קריאה מה-SSH (output מהמערכת)

channel_write:
    צד כתיבה ל-SSH (input למערכת)

tx_clone:
    העתק של output_tx עבור thread נפרד

Thread 1 (Read):
    קורא פלט מה-VM ושולח לדפדפן דרך channel

Thread 2 (Write):
    מקבל קלט מהדפדפן ושולח ל-VM דרך SSH
*/

pub fn connect_and_bridge(
    port:u16,
    username: &str,
    password: &str,
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), String> {
    println!("[INFO] Bridging SSH for port {}...", port);


    // 1 connect by ssh to vm 
    let tcp_target = format!("127.0.0.1:{}", port);
    let tcp = TcpStream::connect(&tcp_target).map_err(|e| e.to_string())?;

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(|e| e.to_string())?;
    sess.userauth_password(username, password).map_err(|e| e.to_string())?;

    let mut channel = sess.channel_session().map_err(|e| e.to_string())?;

    channel.request_pty("xterm", None, Some((80, 24, 0, 0))).map_err(|e| e.to_string())?;

    channel.shell().map_err(|e| e.to_string())?;
    println!("[SUCCESS] Interactive shell started on port {}.", port);

    let mut channel_read = channel.clone();
    let mut channel_write = channel;


    let tx_clone = output_tx.clone();
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match channel_read.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let output = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if tx_clone.blocking_send(output).is_err(){
                        break;
                    }
                
                }
                Err(_) => break,
            }
        }
        println!("[INFO] SSH Read thread terminated.");
    });

    thread::spawn(move || {
        while let Some(input) = input_rx.blocking_recv() {
            if channel_write.write_all(input.as_bytes()).is_err() {
                break;
            }
            let _ = channel_write.flush();
        }
        println!("[INFO] SSH Write thread terminated.");
    });

    Ok(())
}
