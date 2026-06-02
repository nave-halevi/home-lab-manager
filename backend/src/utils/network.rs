use std::net::TcpListener;

pub fn get_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("קריסה: לא הצלחנו למצוא פורט פנוי במערכת ההפעלה");

    let port =listener.local_addr().unwrap().port();
    port
}