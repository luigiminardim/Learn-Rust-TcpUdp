use std::net::TcpListener;

use learn_rust_tcp_udp::Channel;

fn main() {
    Server::new().run();
}

pub struct Server {
    channel: Channel,
}

impl Server {
    pub fn new() -> Self {
        let stream = TcpListener::bind("127.0.0.1:3000")
            .expect("Failed to bind to address")
            .accept()
            .expect("Failed to accept connection")
            .0;
        println!("Connected to client");
        Self {
            channel: Channel::new(stream),
        }
    }

    pub fn run(self) {
        self.channel.run();
    }
}
