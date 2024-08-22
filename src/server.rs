use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
    ops::{Deref, DerefMut},
};

pub struct Server(TcpListener);

impl Deref for Server {
    type Target = TcpListener;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Server {
    /// Sets up a listener at 127.0.0.1:3000
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind address");
        Self(listener)
    }

    /// Accepts a connection, then reads lines from it until EOF or an error occurs.
    pub fn run(&mut self) {
        while let Ok((mut stream, _)) = self.accept() {
            let buf = BufReader::new(&mut stream);
            let mut lines = buf.lines();

            while let Some(Ok(body)) = lines.next() {
                println!("Received: {}", body);
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
