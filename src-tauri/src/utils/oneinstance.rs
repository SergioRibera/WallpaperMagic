use std::net::TcpListener;

pub fn create_app_lock(port: u16) -> TcpListener {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(socket) => {
            socket
        },
        Err(_) => {
            panic!("Couldn't lock port {}: another instance already running?", port);
        }
    }
}

pub fn remove_app_lock(socket: TcpListener) {
    drop(socket);
}

pub fn send_args(args: Vec<String>) {
}
