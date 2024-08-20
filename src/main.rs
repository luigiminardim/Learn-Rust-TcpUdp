/// A simple TCP client and server.
/// The client sends messages to the server and the server prints them.
/// The server sends messages to the client and the client prints them.
/// To run the server use `cargo run --features "server"`
/// To run the client use `cargo run --features "client"` in a separate terminal.

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

#[cfg(feature = "server")]
fn main() {
    Server::new().run();
}

/// cargo run --features "server"
#[cfg(feature = "client")]
fn main() {
    Client::new().run();
}

struct Receiver {
    stream: TcpStream,
}

impl Receiver {
    fn run(&mut self) {
        loop {
            let mut buffer = String::new();
            self.stream
                .read_to_string(&mut buffer)
                .expect("Failed to read from stream");
            println!("Received: {}", buffer);
        }
    }
}

struct Sender {
    stream: TcpStream,
}

impl Sender {
    fn run(&mut self) {
        loop {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read from stdin");
            self.stream
                .write_all(buffer.as_bytes())
                .expect("Failed to write to stream");
        }
    }
}

struct Server {
    receiver: Receiver,
    sender: Sender,
}

impl Server {
    fn new() -> Self {
        let stream = TcpListener::bind("127.0.0.1:3000")
            .expect("Failed to bind to address")
            .accept()
            .expect("Failed to accept connection")
            .0;
        println!("Connected to client");
        Self {
            receiver: Receiver {
                stream: stream.try_clone().expect("Failed to clone stream"),
            },
            sender: Sender { stream },
        }
    }

    fn run(&mut self) {
        thread::spawn(|| self.receiver.run());
        self.sender.run();
    }
}

struct Client {
    receiver: Receiver,
    sender: Sender,
}

impl Client {
    fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:3000").expect("Failed to connect to server");
        println!("Connected to server");
        Self {
            receiver: Receiver {
                stream: stream.try_clone().expect("Failed to clone stream"),
            },
            sender: Sender { stream },
        }
    }

    fn run(&mut self) {
        thread::spawn(|| self.receiver.run());
        self.sender.run();
    }
}
