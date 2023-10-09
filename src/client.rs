
use std::{os::unix::net::UnixStream, io::{ Write, BufWriter, BufReader, IoSlice, Read, BufRead}, vec};

use crate::response::Response;

const PREFIX: &[u8] = b"\x1bP@kitty-cmd";
const SUFFIX: &[u8] = b"\x1b\\";

pub struct Client {
    reader: BufReader<UnixStream>,
    writer: BufWriter<UnixStream>,
}

impl Client {

    pub fn connect(path: &str) -> std::io::Result<Self> {
        let stream = UnixStream::connect(path)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter  ::new(stream);
        Ok(Client { reader, writer })
    }

    pub fn send_command(&mut self, command: &[u8]) -> std::io::Result<usize> {
        let resp = self.writer.write_vectored(&[IoSlice::new(PREFIX), IoSlice::new(command), IoSlice::new(SUFFIX)]);
        self.writer.flush()?;
        resp
    }

    pub fn get_response(&mut self) -> Result<Response, Error> {
        let mut prefix = [0u8; PREFIX.len()];
        let mut raw = vec![];
        let mut chunk = vec![];
        let mut bytes: usize = 0;

        self.reader.read_exact(&mut prefix)?;

        if !prefix.iter().zip(PREFIX.iter()).all(|(a, b)| a == b){
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        loop {
            bytes += self.reader.read_until(SUFFIX[0], &mut chunk)?;

            if chunk[bytes - 1] != SUFFIX[0] {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }

            raw.append(&mut chunk);
            chunk = vec![0u8];
            let b = self.reader.read(&mut chunk)?;

            if b == 0 {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }
            raw.append(&mut chunk);
            bytes += b;

            //Found the end of the response
            if raw[bytes - 1] == SUFFIX[1] {
                break;
            }
        }

        Ok(Response::try_from(&raw[..bytes - 2])?)

    }

}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(value: std::io::ErrorKind) -> Self {
        Error::IoError(value.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonError(value)
    }
}