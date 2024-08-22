use std::{io::{Read, Write}, net::UdpSocket};

pub struct UdpSocketBuffer(UdpSocket);

impl UdpSocketBuffer {
    pub fn new(local_address: &str, remote_address: &str) -> Self {
        let socket = UdpSocket::bind(local_address).expect("Failed to bind to address");
        socket
            .connect(remote_address)
            .expect("Failed to connect to address");
        Self(socket)
    }
}

impl Clone for UdpSocketBuffer {
    fn clone(&self) -> Self {
        Self(self.0.try_clone().expect("Failed to clone socket"))
    }
}

impl Write for UdpSocketBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.send(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for UdpSocketBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.recv(buf)
    }
}
