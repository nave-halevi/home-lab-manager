use std::net::TcpListener;

pub fn get_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Crash: We were unable to find a free port in the operating system");

    let port = listener.local_addr().unwrap().port();
    port
}
