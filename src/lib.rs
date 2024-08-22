pub mod udp;

use std::{
    io::{BufRead, BufReader, Read, Write},
    thread,
};


struct Receiver {
    buf_reader: Box<dyn Read>,
}

impl Receiver {
    fn run(self) {
        let buf_reader = BufReader::new(self.buf_reader);
        for line in buf_reader
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
    buf_writer: Box<dyn Write + Send>,
}

impl Sender {
    fn run(mut self) {
        loop {
            let mut stdin_buffer = String::new();
            std::io::stdin()
                .read_line(&mut stdin_buffer)
                .expect("Failed to read from stdin");
            self.buf_writer
                .write_all(stdin_buffer.as_bytes())
                .expect("Failed to write to stream");
            self.buf_writer.flush().expect("Failed to flush stream");
        }
    }
}

pub struct Channel {
    receiver: Receiver,
    sender: Sender,
}

impl Channel {
    pub fn new(buf_reader: Box<dyn Read>, buf_writer: Box<dyn Write + Send>) -> Self {
        Self {
            receiver: Receiver { buf_reader },
            sender: Sender { buf_writer },
        }
    }

    pub fn run(self) {
        let sender_thread = thread::spawn(|| self.sender.run());
        self.receiver.run();
        drop(sender_thread);
    }
}
