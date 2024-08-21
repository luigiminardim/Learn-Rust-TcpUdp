use std::net::TcpStream;

use learn_rust_tcp_udp::Channel;

fn main() {
    Client::new().run();
}

pub struct Client {
    channel: Channel,
}

impl Client {
    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:3000").expect("Failed to connect to server");
        println!("Connected to server");
        Self {
            channel: Channel::new(stream),
        }
    }

    pub fn run(self) {
        self.channel.run();
    }
}
