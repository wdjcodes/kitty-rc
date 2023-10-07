
use std::{os::unix::net::UnixStream, io::{Result, Write, Read, IoSlice}};

const PREFIX: &[u8] = b"\x1bP@kitty-cmd";
const SUFFIX: &[u8] = b"\x1b\\";

pub struct Client {
    stream: UnixStream,
}

impl Client {

    pub fn connect(path: &str) -> Result<Self> {
        Ok(Client{stream: UnixStream::connect(path)?})
    }

    pub fn send_command(&mut self, command: &[u8]) -> Result<usize> {
        self.stream.write_vectored(&[IoSlice::new(PREFIX), IoSlice::new(command), IoSlice::new(SUFFIX)])
    }

    pub fn get_response(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stream.read(buf)
    }
}