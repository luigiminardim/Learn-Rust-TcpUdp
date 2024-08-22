use std::{
    io::Write,
    net::TcpStream,
    ops::{Deref, DerefMut},
};

/// Wrapper over a TcpStream. Mainly to namespace client operations
pub struct Client(TcpStream);

impl Deref for Client {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Client {
    /// Connects to 127.0.0.1:3000
    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:3000").expect("Failed to connect to server");
        println!("Connected to server");
        Self(stream)
    }

    /// Accepts an input from stdin, then writes it to the TCP stream.
    pub fn run(&mut self) {
        loop {
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Failed to get input from stdin!");

            self.send(buf);
        }
    }

    /// Sends the input over the TCP stream.
    pub fn send(&mut self, input: String) {
        self.write_all(input.as_bytes())
            .expect("Failed to write to stream!");
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
