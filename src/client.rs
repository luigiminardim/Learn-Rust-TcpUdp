use std::{io::Write, net::TcpStream};

/// Wrapper over a TcpStream. Mainly to namespace client operations
pub struct Client;

impl Client {
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
        let mut stream = TcpStream::connect("127.0.0.1:3000").expect("Failed to connect to server");
        stream
            .write_all(input.as_bytes())
            .expect("Failed to write to stream!");
        stream.flush().expect("Failed to flush stream!");
    }
}
