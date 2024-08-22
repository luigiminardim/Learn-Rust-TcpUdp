use std::io::{Read, Write};

#[cfg(feature = "tcp")]
use std::net::TcpStream;

use learn_rust_tcp_udp::{udp::UdpSocketBuffer, Channel};

fn main() {
    Client::new().run();
}

pub struct Client {
    channel: Channel,
}

impl Client {
    #[cfg(feature = "tcp")]
    fn create_buffers() -> (Box<dyn Read>, Box<dyn Write + Send>) {
        let stream = TcpStream::connect("127.0.0.1:3000").expect("Failed to connect to server");
        println!("Connected to server");
        let buf_reader = Box::new(stream.try_clone().expect("Failed to clone stream"));
        let buf_writer = Box::new(stream);
        (buf_reader, buf_writer)
    }

    #[cfg(feature = "udp")]
    fn create_buffers() -> (Box<dyn Read>, Box<dyn Write + Send>) {
        let client_address = "127.0.0.1:3001";
        let server_address = "127.0.0.1:3000";
        let buf_reader = Box::new(UdpSocketBuffer::new(
            &client_address,
            &server_address,
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

