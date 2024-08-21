use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

struct Receiver {
    buf_reader: Box<BufReader<dyn Read>>,
}

impl Receiver {
    fn run(self) {
        for line in self
            .buf_reader
            .lines()
            .map(|line| line.expect("Failed to read from stream"))
            .take_while(|line| !line.is_empty())
        {
            println!("Received: {}", line);
        }
        println!("Connection closed");
    }
}

struct Sender {
    stream: TcpStream,
}

impl Sender {
    fn run(self) {
        let mut stream = self.stream.try_clone().expect("Failed to clone stream");
        loop {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read from stdin");
            stream
                .write_all(buffer.as_bytes())
                .expect("Failed to write to stream");
            stream.flush().expect("Failed to flush stream");
        }
    }
}

pub struct Channel {
    receiver: Receiver,
    sender: Sender,
}

impl Channel {
    pub fn new(stream: TcpStream) -> Self {
        let buf_reader = BufReader::new(Box::new(stream.try_clone().expect("Failed to clone stream")));

        Self {
            receiver: Receiver { buf_reader: Box::new(buf_reader) },
            sender: Sender { stream },
        }
    }

    pub fn run(self) {
        let sender_thread = thread::spawn(|| self.sender.run());
        self.receiver.run();
        drop(sender_thread);
    }
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
