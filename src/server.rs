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

impl Default for Server {
    fn default() -> Self {
        Self::new()
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
        for stream in self.incoming() {
            match stream {
                Ok(s) => {
                    let buf_reader = BufReader::new(&s);
                    let line = buf_reader.lines().next().unwrap().unwrap();

                    println!("Received: {line}");
                }
                Err(e) => {
                    eprintln!("Failed to receive connection!");
                    eprintln!("{}", e);
                    println!("Exiting...");
                    break;
                }
            }
        }
    }
}
