use std::io::{Read, Write};

#[cfg(feature = "tcp")]
use std::io::net::TcpListener;

use learn_rust_tcp_udp::Channel;

fn main() {
    Server::new().run();
}

pub struct Server {
    channel: Channel,
}

impl Server {
    #[cfg(feature = "tcp")]
    fn create_buffers() -> (Box<dyn Read>, Box<dyn Write + Send>) {
        let stream = TcpListener::bind("127.0.0.1:3000")
            .expect("Failed to bind to address")
            .accept()
            .expect("Failed to accept connection")
            .0;
        println!("Connected to client");
        let buf_reader = Box::new(stream.try_clone().expect("Failed to clone stream"));
        let buf_writer = Box::new(stream);
        (buf_reader, buf_writer)
    }

    #[cfg(feature = "udp")]
    fn create_buffers() -> (Box<dyn Read>, Box<dyn Write + Send>) {
        use learn_rust_tcp_udp::udp::UdpSocketBuffer;

        let server_address = "127.0.0.1:3000";
        let client_address = "127.0.0.1:3001";
        let buf_reader = Box::new(UdpSocketBuffer::new(
            &server_address,
            &client_address,
        ));
        let buf_writer = Box::new(buf_reader.clone());
        (buf_reader, buf_writer)
    }

    pub fn new() -> Self {
        let (buf_reader, buf_writer) = Self::create_buffers();
        Self {
            channel: Channel::new(buf_reader, buf_writer),
        }
    }

    pub fn run(self) {
        self.channel.run();
    }
}
